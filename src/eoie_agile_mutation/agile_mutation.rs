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
#[derive(Clone)]
enum US2 {
    US2_RootLeads,
    US2_RootAfterMode,
}
#[derive(Clone)]
enum US3 {
    US3_RootNamed(Rc<str>),
    US3_RootFlagged(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Valid(i32),
    US4_Invalid,
}
#[derive(Clone)]
enum US6 {
    US6_AgilePlanned,
    US6_AgileActive,
    US6_AgileBlocked,
    US6_AgilePaused,
    US6_AgileDone,
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US6),
    US5_Invalid,
}
fn state_lock_refused_1() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile state/agile.spi stayed locked by another writer for 30 s; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
    let mut v1: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v0 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v1
}
fn agile_subcommand_shape_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    let mut v5: Rc<str> = if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    };
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: bool = if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v0 == v7 ;
        v8
    } else {
        false
    };
    let mut v11: Rc<str> = if v9 {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <title>"); } LIT.with(|lit| lit.clone()) };
        v10.clone()
    } else {
        v5.clone()
    };
    let mut v12: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v15: bool = if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease"); } LIT.with(|lit| lit.clone()) };
        let mut v14: bool = v0 == v13 ;
        v14
    } else {
        false
    };
    let mut v17: Rc<str> = if v15 {
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v16.clone()
    } else {
        v11.clone()
    };
    let mut v18: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v21: bool = if v18 {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let mut v20: bool = v0 == v19 ;
        v20
    } else {
        false
    };
    let mut v23: Rc<str> = if v21 {
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v22.clone()
    } else {
        v17.clone()
    };
    let mut v24: bool = v23.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v27: bool = if v24 {
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff"); } LIT.with(|lit| lit.clone()) };
        let mut v26: bool = v0 == v25 ;
        v26
    } else {
        false
    };
    let mut v29: Rc<str> = if v27 {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v28.clone()
    } else {
        v23.clone()
    };
    let mut v30: bool = v29.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v33: bool = if v30 {
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
        let mut v32: bool = v0 == v31 ;
        v32
    } else {
        false
    };
    let mut v35: Rc<str> = if v33 {
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json]"); } LIT.with(|lit| lit.clone()) };
        v34.clone()
    } else {
        v29.clone()
    };
    let mut v36: bool = v35.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v39: bool = if v36 {
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
        let mut v38: bool = v0 == v37 ;
        v38
    } else {
        false
    };
    let mut v41: Rc<str> = if v39 {
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--compiler <absolute>]"); } LIT.with(|lit| lit.clone()) };
        v40.clone()
    } else {
        v35.clone()
    };
    let mut v42: bool = v41.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v45: bool = if v42 {
        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record"); } LIT.with(|lit| lit.clone()) };
        let mut v44: bool = v0 == v43 ;
        v44
    } else {
        false
    };
    let mut v47: Rc<str> = if v45 {
        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v46.clone()
    } else {
        v41.clone()
    };
    let mut v48: bool = v47.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v51: bool = if v48 {
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
        let mut v50: bool = v0 == v49 ;
        v50
    } else {
        false
    };
    let mut v53: Rc<str> = if v51 {
        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
        v52.clone()
    } else {
        v47.clone()
    };
    let mut v54: bool = v53.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v57: bool = if v54 {
        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
        let mut v56: bool = v0 == v55 ;
        v56
    } else {
        false
    };
    let mut v59: Rc<str> = if v57 {
        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>]"); } LIT.with(|lit| lit.clone()) };
        v58.clone()
    } else {
        v53.clone()
    };
    let mut v60: bool = v59.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v63: bool = if v60 {
        let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
        let mut v62: bool = v0 == v61 ;
        v62
    } else {
        false
    };
    let mut v65: Rc<str> = if v63 {
        let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>"); } LIT.with(|lit| lit.clone()) };
        v64.clone()
    } else {
        v59.clone()
    };
    let mut v66: bool = v65.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v69: bool = if v66 {
        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
        let mut v68: bool = v0 == v67 ;
        v68
    } else {
        false
    };
    let mut v71: Rc<str> = if v69 {
        let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
        v70.clone()
    } else {
        v65.clone()
    };
    let mut v72: bool = v71.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v75: bool = if v72 {
        let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan"); } LIT.with(|lit| lit.clone()) };
        let mut v74: bool = v0 == v73 ;
        v74
    } else {
        false
    };
    let mut v77: Rc<str> = if v75 {
        let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--json]"); } LIT.with(|lit| lit.clone()) };
        v76.clone()
    } else {
        v71.clone()
    };
    let mut v78: bool = v77.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v81: bool = if v78 {
        let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags"); } LIT.with(|lit| lit.clone()) };
        let mut v80: bool = v0 == v79 ;
        v80
    } else {
        false
    };
    let mut v83: Rc<str> = if v81 {
        let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
        v82.clone()
    } else {
        v77.clone()
    };
    let mut v84: bool = v83.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v87: bool = if v84 {
        let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
        let mut v86: bool = v0 == v85 ;
        v86
    } else {
        false
    };
    let mut v89: Rc<str> = if v87 {
        let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
        v88.clone()
    } else {
        v83.clone()
    };
    let mut v90: bool = v89.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v93: bool = if v90 {
        let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note-edit"); } LIT.with(|lit| lit.clone()) };
        let mut v92: bool = v0 == v91 ;
        v92
    } else {
        false
    };
    let mut v95: Rc<str> = if v93 {
        let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <note-number> <text> [--as <author>] [--append]"); } LIT.with(|lit| lit.clone()) };
        v94.clone()
    } else {
        v89.clone()
    };
    let mut v96: bool = v95.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v99: bool = if v96 {
        let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
        let mut v98: bool = v0 == v97 ;
        v98
    } else {
        false
    };
    let mut v101: Rc<str> = if v99 {
        let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
        v100.clone()
    } else {
        v95.clone()
    };
    let mut v102: bool = v101.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v105: bool = if v102 {
        let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
        let mut v104: bool = v0 == v103 ;
        v104
    } else {
        false
    };
    let mut v107: Rc<str> = if v105 {
        let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
        v106.clone()
    } else {
        v101.clone()
    };
    let mut v108: bool = v107.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v111: bool = if v108 {
        let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
        let mut v110: bool = v0 == v109 ;
        v110
    } else {
        false
    };
    let mut v113: Rc<str> = if v111 {
        let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record"); } LIT.with(|lit| lit.clone()) };
        v112.clone()
    } else {
        v107.clone()
    };
    let mut v114: bool = v113.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v117: bool = if v114 {
        let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
        let mut v116: bool = v0 == v115 ;
        v116
    } else {
        false
    };
    let mut v119: Rc<str> = if v117 {
        let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count]"); } LIT.with(|lit| lit.clone()) };
        v118.clone()
    } else {
        v113.clone()
    };
    let mut v120: bool = v119.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v123: bool = if v120 {
        let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("roots"); } LIT.with(|lit| lit.clone()) };
        let mut v122: bool = v0 == v121 ;
        v122
    } else {
        false
    };
    let mut v125: Rc<str> = if v123 {
        let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [add|drop <path>...]"); } LIT.with(|lit| lit.clone()) };
        v124.clone()
    } else {
        v119.clone()
    };
    let mut v126: bool = v125.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v129: bool = if v126 {
        let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("links"); } LIT.with(|lit| lit.clone()) };
        let mut v128: bool = v0 == v127 ;
        v128
    } else {
        false
    };
    let mut v131: Rc<str> = if v129 {
        let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --memory <dir> [--json]"); } LIT.with(|lit| lit.clone()) };
        v130.clone()
    } else {
        v125.clone()
    };
    let mut v132: bool = v131.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v135: bool = if v132 {
        let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("views"); } LIT.with(|lit| lit.clone()) };
        let mut v134: bool = v0 == v133 ;
        v134
    } else {
        false
    };
    let mut v137: Rc<str> = if v135 {
        let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v136.clone()
    } else {
        v131.clone()
    };
    let mut v138: bool = v137.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v141: bool = if v138 {
        let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view"); } LIT.with(|lit| lit.clone()) };
        let mut v140: bool = v0 == v139 ;
        v140
    } else {
        false
    };
    let mut v143: Rc<str> = if v141 {
        let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
        v142.clone()
    } else {
        v137.clone()
    };
    let mut v144: bool = v143.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v147: bool = if v144 {
        let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view-add"); } LIT.with(|lit| lit.clone()) };
        let mut v146: bool = v0 == v145 ;
        v146
    } else {
        false
    };
    if v147 {
        let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
        v148.clone()
    } else {
        v143.clone()
    }
}
fn agile_usage_error_2() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: Rc<str> = agile_subcommand_shape_3(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2 == v3 ;
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root> [--compiler <absolute>], record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>], edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], note-edit <root> <id> <note-number> <text> [--as <author>] [--append], notes from-title <root> <id>, rename <root> <old-id> <new-id>, todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record, next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count], roots <root> [add|drop <path>...], links <root> --memory <dir> [--json], views <root>, view <root> <name> [extra flags...], view-add <root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v5 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v6
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v1, &*v8, &*v2, &*v9].concat());
        let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v3, &*v3].concat());
        let mut v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v13 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v14
    }
}
fn agile_subcommand_of_4(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    let mut v6: US0 = if v2 {
        let mut v3: US1 = US1::US1_AgileInit;
        US0::US0_AgileKnown(v3.clone())
    } else {
        US0::US0_AgileUnknownName
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin"); } LIT.with(|lit| lit.clone()) };
    let mut v8: bool = v0 == v7 ;
    let mut v11: US0 = if v8 {
        let mut v9: US1 = US1::US1_AgileBegin;
        US0::US0_AgileKnown(v9.clone())
    } else {
        v6.clone()
    };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease"); } LIT.with(|lit| lit.clone()) };
    let mut v13: bool = v0 == v12 ;
    let mut v16: US0 = if v13 {
        let mut v14: US1 = US1::US1_AgileLease;
        US0::US0_AgileKnown(v14.clone())
    } else {
        v11.clone()
    };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let mut v18: bool = v0 == v17 ;
    let mut v21: US0 = if v18 {
        let mut v19: US1 = US1::US1_AgileStatus;
        US0::US0_AgileKnown(v19.clone())
    } else {
        v16.clone()
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff"); } LIT.with(|lit| lit.clone()) };
    let mut v23: bool = v0 == v22 ;
    let mut v26: US0 = if v23 {
        let mut v24: US1 = US1::US1_AgileHandoff;
        US0::US0_AgileKnown(v24.clone())
    } else {
        v21.clone()
    };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
    let mut v28: bool = v0 == v27 ;
    let mut v31: US0 = if v28 {
        let mut v29: US1 = US1::US1_AgileList;
        US0::US0_AgileKnown(v29.clone())
    } else {
        v26.clone()
    };
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
    let mut v33: bool = v0 == v32 ;
    let mut v36: US0 = if v33 {
        let mut v34: US1 = US1::US1_AgileCheck;
        US0::US0_AgileKnown(v34.clone())
    } else {
        v31.clone()
    };
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record"); } LIT.with(|lit| lit.clone()) };
    let mut v38: bool = v0 == v37 ;
    let mut v41: US0 = if v38 {
        let mut v39: US1 = US1::US1_AgileRecord;
        US0::US0_AgileKnown(v39.clone())
    } else {
        v36.clone()
    };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
    let mut v43: bool = v0 == v42 ;
    let mut v46: US0 = if v43 {
        let mut v44: US1 = US1::US1_AgileSet;
        US0::US0_AgileKnown(v44.clone())
    } else {
        v41.clone()
    };
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
    let mut v48: bool = v0 == v47 ;
    let mut v51: US0 = if v48 {
        let mut v49: US1 = US1::US1_AgileAdd;
        US0::US0_AgileKnown(v49.clone())
    } else {
        v46.clone()
    };
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
    let mut v53: bool = v0 == v52 ;
    let mut v56: US0 = if v53 {
        let mut v54: US1 = US1::US1_AgileEdit;
        US0::US0_AgileKnown(v54.clone())
    } else {
        v51.clone()
    };
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
    let mut v58: bool = v0 == v57 ;
    let mut v61: US0 = if v58 {
        let mut v59: US1 = US1::US1_AgileShow;
        US0::US0_AgileKnown(v59.clone())
    } else {
        v56.clone()
    };
    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan"); } LIT.with(|lit| lit.clone()) };
    let mut v63: bool = v0 == v62 ;
    let mut v66: US0 = if v63 {
        let mut v64: US1 = US1::US1_AgilePlan;
        US0::US0_AgileKnown(v64.clone())
    } else {
        v61.clone()
    };
    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags"); } LIT.with(|lit| lit.clone()) };
    let mut v68: bool = v0 == v67 ;
    let mut v71: US0 = if v68 {
        let mut v69: US1 = US1::US1_AgileMigrateTags;
        US0::US0_AgileKnown(v69.clone())
    } else {
        v66.clone()
    };
    let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
    let mut v73: bool = v0 == v72 ;
    let mut v76: US0 = if v73 {
        let mut v74: US1 = US1::US1_AgileNote;
        US0::US0_AgileKnown(v74.clone())
    } else {
        v71.clone()
    };
    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note-edit"); } LIT.with(|lit| lit.clone()) };
    let mut v78: bool = v0 == v77 ;
    let mut v81: US0 = if v78 {
        let mut v79: US1 = US1::US1_AgileNoteEdit;
        US0::US0_AgileKnown(v79.clone())
    } else {
        v76.clone()
    };
    let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
    let mut v83: bool = v0 == v82 ;
    let mut v86: US0 = if v83 {
        let mut v84: US1 = US1::US1_AgileNotes;
        US0::US0_AgileKnown(v84.clone())
    } else {
        v81.clone()
    };
    let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
    let mut v88: bool = v0 == v87 ;
    let mut v91: US0 = if v88 {
        let mut v89: US1 = US1::US1_AgileRename;
        US0::US0_AgileKnown(v89.clone())
    } else {
        v86.clone()
    };
    let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
    let mut v93: bool = v0 == v92 ;
    let mut v96: US0 = if v93 {
        let mut v94: US1 = US1::US1_AgileTodos;
        US0::US0_AgileKnown(v94.clone())
    } else {
        v91.clone()
    };
    let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
    let mut v98: bool = v0 == v97 ;
    let mut v101: US0 = if v98 {
        let mut v99: US1 = US1::US1_AgileNext;
        US0::US0_AgileKnown(v99.clone())
    } else {
        v96.clone()
    };
    let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("roots"); } LIT.with(|lit| lit.clone()) };
    let mut v103: bool = v0 == v102 ;
    let mut v106: US0 = if v103 {
        let mut v104: US1 = US1::US1_AgileRoots;
        US0::US0_AgileKnown(v104.clone())
    } else {
        v101.clone()
    };
    let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("links"); } LIT.with(|lit| lit.clone()) };
    let mut v108: bool = v0 == v107 ;
    let mut v111: US0 = if v108 {
        let mut v109: US1 = US1::US1_AgileLinks;
        US0::US0_AgileKnown(v109.clone())
    } else {
        v106.clone()
    };
    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("views"); } LIT.with(|lit| lit.clone()) };
    let mut v113: bool = v0 == v112 ;
    let mut v116: US0 = if v113 {
        let mut v114: US1 = US1::US1_AgileViews;
        US0::US0_AgileKnown(v114.clone())
    } else {
        v111.clone()
    };
    let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view"); } LIT.with(|lit| lit.clone()) };
    let mut v118: bool = v0 == v117 ;
    let mut v121: US0 = if v118 {
        let mut v119: US1 = US1::US1_AgileView;
        US0::US0_AgileKnown(v119.clone())
    } else {
        v116.clone()
    };
    let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view-add"); } LIT.with(|lit| lit.clone()) };
    let mut v123: bool = v0 == v122 ;
    if v123 {
        let mut v124: US1 = US1::US1_AgileViewAdd;
        US0::US0_AgileKnown(v124.clone())
    } else {
        v121.clone()
    }
}
fn state_lock_taken_6(mut v0: Rc<str>) -> bool {
    let mut v1: bool = { let state = std::path::Path::new(&*v0).join("state"); let lock = state.join("agile.spi.lock"); let started = std::time::Instant::now(); loop { if std::fs::create_dir(&lock).is_ok() { break true; } if !state.is_dir() { break true; } let stale = std::fs::metadata(&lock).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() >= 10).unwrap_or(false); if stale { let _ = std::fs::remove_dir(&lock); continue; } if started.elapsed() >= std::time::Duration::from_secs(30) { break false; } std::thread::sleep(std::time::Duration::from_millis(25)); } };
    v1
}
fn state_lock_held_5(mut v0: bool, mut v1: Rc<str>) -> bool {
    if v0 {
        state_lock_taken_6(v1.clone())
    } else {
        true
    }
}
fn planning_field_named_7(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v1.to_ascii_lowercase());
    let mut v3: bool = v0.clone() == v2.clone();
    if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v4.to_ascii_lowercase());
        let mut v6: bool = v0.clone() == v5.clone();
        if v6 {
            true
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
            let mut v9: bool = v0.clone() == v8.clone();
            v9
        }
    }
}
fn state_lock_released_9(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("state").join("agile.spi.lock")); };
    ()
}
fn state_lock_done_8(mut v0: bool, mut v1: Rc<str>) -> () {
    if v0 {
        state_lock_released_9(v1.clone())
    }
}
fn agile_entry_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: US0 = agile_subcommand_of_4(v1.clone());
    match &v2 {
        US0::US0_AgileKnown(v3) => {
            let mut v3: US1 = v3.clone();
            let mut v8: US2 = match &v3 {
                US1::US1_AgileMigrateTags => {
                    US2::US2_RootAfterMode
                }
                US1::US1_AgileNotes => {
                    US2::US2_RootAfterMode
                }
                _ => {
                    US2::US2_RootLeads
                }
            };
            let mut v10: i32 = match &v8 {
                US2::US2_RootAfterMode => {
                    2i32
                }
                US2::US2_RootLeads => {
                    1i32
                }
            };
            let mut v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            let mut v13: bool = v11.starts_with(&*v12);
            let mut v16: US3 = if v13 {
                US3::US3_RootFlagged(v11.clone())
            } else {
                US3::US3_RootNamed(v11.clone())
            };
            match &v16 {
                US3::US3_RootFlagged(v17) => {
                    let mut v17: Rc<str> = v17.clone();
                    let (mut v116, mut v117): (Rc<str>, Rc<str>) = match &v3 {
                        US1::US1_AgileInit => {
                            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init"); } LIT.with(|lit| lit.clone()) };
                            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                            (v18.clone(), v19.clone())
                        }
                        _ => {
                            let (mut v114, mut v115): (Rc<str>, Rc<str>) = match &v3 {
                                US1::US1_AgileBegin => {
                                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin"); } LIT.with(|lit| lit.clone()) };
                                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <title>"); } LIT.with(|lit| lit.clone()) };
                                    (v20.clone(), v21.clone())
                                }
                                _ => {
                                    let (mut v112, mut v113): (Rc<str>, Rc<str>) = match &v3 {
                                        US1::US1_AgileLease => {
                                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease"); } LIT.with(|lit| lit.clone()) };
                                            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                            (v22.clone(), v23.clone())
                                        }
                                        _ => {
                                            let (mut v110, mut v111): (Rc<str>, Rc<str>) = match &v3 {
                                                US1::US1_AgileStatus => {
                                                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                    (v24.clone(), v25.clone())
                                                }
                                                _ => {
                                                    let (mut v108, mut v109): (Rc<str>, Rc<str>) = match &v3 {
                                                        US1::US1_AgileHandoff => {
                                                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                            (v26.clone(), v27.clone())
                                                        }
                                                        _ => {
                                                            let (mut v106, mut v107): (Rc<str>, Rc<str>) = match &v3 {
                                                                US1::US1_AgileList => {
                                                                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                                                                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                    (v28.clone(), v29.clone())
                                                                }
                                                                _ => {
                                                                    let (mut v104, mut v105): (Rc<str>, Rc<str>) = match &v3 {
                                                                        US1::US1_AgileCheck => {
                                                                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--compiler <absolute>]"); } LIT.with(|lit| lit.clone()) };
                                                                            (v30.clone(), v31.clone())
                                                                        }
                                                                        _ => {
                                                                            let (mut v102, mut v103): (Rc<str>, Rc<str>) = match &v3 {
                                                                                US1::US1_AgileRecord => {
                                                                                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record"); } LIT.with(|lit| lit.clone()) };
                                                                                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                                    (v32.clone(), v33.clone())
                                                                                }
                                                                                _ => {
                                                                                    let (mut v100, mut v101): (Rc<str>, Rc<str>) = match &v3 {
                                                                                        US1::US1_AgileSet => {
                                                                                            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
                                                                                            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
                                                                                            (v34.clone(), v35.clone())
                                                                                        }
                                                                                        _ => {
                                                                                            let (mut v98, mut v99): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                US1::US1_AgileAdd => {
                                                                                                    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
                                                                                                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                    (v36.clone(), v37.clone())
                                                                                                }
                                                                                                _ => {
                                                                                                    let (mut v96, mut v97): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                        US1::US1_AgileEdit => {
                                                                                                            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
                                                                                                            let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>"); } LIT.with(|lit| lit.clone()) };
                                                                                                            (v38.clone(), v39.clone())
                                                                                                        }
                                                                                                        _ => {
                                                                                                            let (mut v94, mut v95): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                US1::US1_AgileShow => {
                                                                                                                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                                                                                                                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                    (v40.clone(), v41.clone())
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    let (mut v92, mut v93): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                        US1::US1_AgilePlan => {
                                                                                                                            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan"); } LIT.with(|lit| lit.clone()) };
                                                                                                                            let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                            (v42.clone(), v43.clone())
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            let (mut v90, mut v91): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                US1::US1_AgileMigrateTags => {
                                                                                                                                    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                    (v44.clone(), v45.clone())
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    let (mut v88, mut v89): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                        US1::US1_AgileNote => {
                                                                                                                                            let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                            let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                            (v46.clone(), v47.clone())
                                                                                                                                        }
                                                                                                                                        _ => {
                                                                                                                                            let (mut v86, mut v87): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                US1::US1_AgileNoteEdit => {
                                                                                                                                                    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note-edit"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <note-number> <text> [--as <author>] [--append]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                    (v48.clone(), v49.clone())
                                                                                                                                                }
                                                                                                                                                _ => {
                                                                                                                                                    let (mut v84, mut v85): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                        US1::US1_AgileNotes => {
                                                                                                                                                            let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                            let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                            (v50.clone(), v51.clone())
                                                                                                                                                        }
                                                                                                                                                        _ => {
                                                                                                                                                            let (mut v82, mut v83): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                US1::US1_AgileRename => {
                                                                                                                                                                    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                    (v52.clone(), v53.clone())
                                                                                                                                                                }
                                                                                                                                                                _ => {
                                                                                                                                                                    let (mut v80, mut v81): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                        US1::US1_AgileTodos => {
                                                                                                                                                                            let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                            let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                            (v54.clone(), v55.clone())
                                                                                                                                                                        }
                                                                                                                                                                        _ => {
                                                                                                                                                                            let (mut v78, mut v79): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                US1::US1_AgileNext => {
                                                                                                                                                                                    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                    (v56.clone(), v57.clone())
                                                                                                                                                                                }
                                                                                                                                                                                _ => {
                                                                                                                                                                                    let (mut v76, mut v77): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                        US1::US1_AgileRoots => {
                                                                                                                                                                                            let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("roots"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                            let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [add|drop <path>...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                            (v58.clone(), v59.clone())
                                                                                                                                                                                        }
                                                                                                                                                                                        _ => {
                                                                                                                                                                                            let (mut v74, mut v75): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                US1::US1_AgileLinks => {
                                                                                                                                                                                                    let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("links"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --memory <dir> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                    (v60.clone(), v61.clone())
                                                                                                                                                                                                }
                                                                                                                                                                                                _ => {
                                                                                                                                                                                                    let (mut v72, mut v73): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                        US1::US1_AgileViews => {
                                                                                                                                                                                                            let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("views"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                            let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                            (v62.clone(), v63.clone())
                                                                                                                                                                                                        }
                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                            let (mut v70, mut v71): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                US1::US1_AgileView => {
                                                                                                                                                                                                                    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                    (v64.clone(), v65.clone())
                                                                                                                                                                                                                }
                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                    let (mut v68, mut v69): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                        US1::US1_AgileViewAdd => {
                                                                                                                                                                                                                            let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view-add"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                            let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                            (v66.clone(), v67.clone())
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                        _ => unreachable!(),
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
                    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                    let mut v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v116));
                    let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" takes <root> before any flag, got "); } LIT.with(|lit| lit.clone()) };
                    let mut v121: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v120));
                    let mut v122: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v17));
                    let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; usage: agile "); } LIT.with(|lit| lit.clone()) };
                    let mut v124: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v123));
                    let mut v125: Rc<str> = Rc::<str>::from(format!("{}{}", v124, v116));
                    let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let mut v127: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v126));
                    let mut v128: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v117));
                    let mut v129: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v128 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v129
                }
                US3::US3_RootNamed(v130) => {
                    let mut v130: Rc<str> = v130.clone();
                    let mut v155: bool = match &v3 {
                        US1::US1_AgileAdd => {
                            true
                        }
                        US1::US1_AgileBegin => {
                            true
                        }
                        US1::US1_AgileCheck => {
                            false
                        }
                        US1::US1_AgileEdit => {
                            true
                        }
                        US1::US1_AgileHandoff => {
                            false
                        }
                        US1::US1_AgileInit => {
                            false
                        }
                        US1::US1_AgileLease => {
                            false
                        }
                        US1::US1_AgileLinks => {
                            false
                        }
                        US1::US1_AgileList => {
                            false
                        }
                        US1::US1_AgileMigrateTags => {
                            true
                        }
                        US1::US1_AgileNext => {
                            false
                        }
                        US1::US1_AgileNote => {
                            true
                        }
                        US1::US1_AgileNoteEdit => {
                            true
                        }
                        US1::US1_AgileNotes => {
                            true
                        }
                        US1::US1_AgilePlan => {
                            false
                        }
                        US1::US1_AgileRecord => {
                            true
                        }
                        US1::US1_AgileRename => {
                            true
                        }
                        US1::US1_AgileRoots => {
                            true
                        }
                        US1::US1_AgileSet => {
                            true
                        }
                        US1::US1_AgileShow => {
                            false
                        }
                        US1::US1_AgileStatus => {
                            false
                        }
                        US1::US1_AgileTodos => {
                            true
                        }
                        US1::US1_AgileView => {
                            false
                        }
                        US1::US1_AgileViewAdd => {
                            true
                        }
                        US1::US1_AgileViews => {
                            false
                        }
                    };
                    let mut v156: bool = state_lock_held_5(v155, v130.clone());
                    if v156 {
                        let mut v495: i32 = match &v3 {
                            US1::US1_AgileAdd => {
                                let mut v374: i32 = std::env::args().skip(2).count() as i32;
                                let mut v375: bool = 8i32 <= v374;
                                if v375 {
                                    let mut v376: i32 = agile_task_creation::eoie_agile_add_run();
                                    v376
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileBegin => {
                                let mut v164: i32 = 3i32;
                                let mut v165: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v164 ;
                                if v165 {
                                    let mut v166: i32 = 1i32;
                                    let mut v167: Rc<str> = usize::try_from(v166).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v168: i32 = 2i32;
                                    let mut v169: Rc<str> = usize::try_from(v168).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v170: i32 = eoie_agile_lease::eoie_lease_title_words();
                                    let mut v171: bool = 0 < v170;
                                    if v171 {
                                        let mut v172: i32 = eoie_agile_state::agile_begin_owned(v167.as_ref(), v169.as_ref());
                                        v172
                                    } else {
                                        let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin title must contain 3..7 visible words"); } LIT.with(|lit| lit.clone()) };
                                        let mut v174: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v173 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v174
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileCheck => {
                                let mut v204: i32 = 2i32;
                                let mut v205: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v204 ;
                                if v205 {
                                    let mut v206: i32 = 1i32;
                                    let mut v207: Rc<str> = usize::try_from(v206).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v208: i32 = eoie_agile_state::agile_check_owned(v207.as_ref());
                                    v208
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileEdit => {
                                let mut v379: i32 = 5i32;
                                let mut v380: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v379 ;
                                if v380 {
                                    let mut v381: i32 = 3i32;
                                    let mut v382: Rc<str> = usize::try_from(v381).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v383: bool = planning_field_named_7(v382.clone());
                                    if v383 {
                                        let mut v384: i32 = agile_planning_revision::eoie_agile_planning_edit_run();
                                        v384
                                    } else {
                                        let mut v385: i32 = agile_task_revision::eoie_agile_edit_run();
                                        v385
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileHandoff => {
                                let mut v192: i32 = 2i32;
                                let mut v193: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v192 ;
                                if v193 {
                                    let mut v194: i32 = 1i32;
                                    let mut v195: Rc<str> = usize::try_from(v194).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v196: i32 = eoie_handoff_render::handoff_render_owned(v195.as_ref());
                                    v196
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileInit => {
                                let mut v157: i32 = 2i32;
                                let mut v158: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v157 ;
                                if v158 {
                                    let mut v159: i32 = 1i32;
                                    let mut v160: Rc<str> = usize::try_from(v159).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v161: i32 = agile_backlog_scaffold::eoie_agile_init_run();
                                    v161
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLease => {
                                let mut v178: i32 = 2i32;
                                let mut v179: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v178 ;
                                if v179 {
                                    let mut v180: i32 = 1i32;
                                    let mut v181: Rc<str> = usize::try_from(v180).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v182: i32 = eoie_agile_state::agile_lease_status_owned(v181.as_ref());
                                    v182
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLinks => {
                                let mut v451: i32 = std::env::args().skip(2).count() as i32;
                                let mut v452: bool = 2i32 <= v451;
                                if v452 {
                                    let mut v453: i32 = agile_memory_links::eoie_agile_links_run();
                                    v453
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileList => {
                                let mut v199: i32 = std::env::args().skip(2).count() as i32;
                                let mut v200: bool = 2i32 <= v199;
                                if v200 {
                                    let mut v201: i32 = agile_task_listing::eoie_agile_list_run();
                                    v201
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileMigrateTags => {
                                let mut v402: i32 = std::env::args().skip(2).count() as i32;
                                let mut v403: bool = 2i32 <= v402;
                                if v403 {
                                    let mut v404: i32 = agile_planning_migration::eoie_agile_migrate_tags_run();
                                    v404
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNext => {
                                let mut v441: i32 = std::env::args().skip(2).count() as i32;
                                let mut v442: bool = 2i32 <= v441;
                                if v442 {
                                    let mut v443: i32 = agile_task_next::eoie_agile_next_run();
                                    v443
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNote => {
                                let mut v407: i32 = 4i32;
                                let mut v408: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v407 ;
                                let mut v411: bool = if v408 {
                                    true
                                } else {
                                    let mut v409: i32 = 6i32;
                                    let mut v410: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v409 ;
                                    v410
                                };
                                if v411 {
                                    let mut v412: i32 = agile_task_notes::eoie_agile_note_run();
                                    v412
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNoteEdit => {
                                let mut v415: i32 = 5i32;
                                let mut v416: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v415 ;
                                let mut v419: bool = if v416 {
                                    true
                                } else {
                                    let mut v417: i32 = 7i32;
                                    let mut v418: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v417 ;
                                    v418
                                };
                                if v419 {
                                    let mut v420: i32 = agile_note_edit::eoie_agile_note_edit_run();
                                    v420
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNotes => {
                                let mut v423: i32 = 4i32;
                                let mut v424: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v423 ;
                                if v424 {
                                    let mut v425: i32 = agile_title_notes::eoie_agile_notes_from_title_run();
                                    v425
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgilePlan => {
                                let mut v397: i32 = std::env::args().skip(2).count() as i32;
                                let mut v398: bool = 2i32 <= v397;
                                if v398 {
                                    let mut v399: i32 = agile_planning_rollup::eoie_agile_planning_rollup_run();
                                    v399
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRecord => {
                                let mut v211: i32 = 2i32;
                                let mut v212: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v211 ;
                                if v212 {
                                    let mut v213: i32 = 1i32;
                                    let mut v214: Rc<str> = usize::try_from(v213).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v215: i32 = eoie_agile_state::agile_record_owned(v214.as_ref());
                                    v215
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRename => {
                                let mut v428: i32 = 4i32;
                                let mut v429: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v428 ;
                                if v429 {
                                    let mut v430: i32 = agile_task_rename::eoie_agile_rename_run();
                                    v430
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRoots => {
                                let mut v446: i32 = std::env::args().skip(2).count() as i32;
                                let mut v447: bool = 2i32 <= v446;
                                if v447 {
                                    let mut v448: i32 = agile_todo_roots::eoie_agile_roots_run();
                                    v448
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSet => {
                                let mut v218: i32 = 7i32;
                                let mut v219: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v218 ;
                                let mut v224: bool = if v219 {
                                    let mut v220: i32 = 5i32;
                                    let mut v221: Rc<str> = usize::try_from(v220).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
                                    let mut v223: bool = v221 == v222 ;
                                    v223
                                } else {
                                    false
                                };
                                if v224 {
                                    let mut v225: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set records no author (only agile note does); drop --as "); } LIT.with(|lit| lit.clone()) };
                                    let mut v226: i32 = 6i32;
                                    let mut v227: Rc<str> = usize::try_from(v226).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v228: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", or add a note with --as to say who moved "); } LIT.with(|lit| lit.clone()) };
                                    let mut v229: i32 = 2i32;
                                    let mut v230: Rc<str> = usize::try_from(v229).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v232: Rc<str> = std::rc::Rc::<str>::from([&*v225, &*v227, &*v228, &*v230, &*v231].concat());
                                    let mut v233: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v232 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v233
                                } else {
                                    let mut v234: i32 = 5i32;
                                    let mut v235: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v234 ;
                                    if v235 {
                                        let mut v236: i32 = 1i32;
                                        let mut v237: Rc<str> = usize::try_from(v236).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v238: i32 = 2i32;
                                        let mut v239: Rc<str> = usize::try_from(v238).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v240: i32 = 3i32;
                                        let mut v241: i32 = usize::try_from(v240).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
                                        let mut v242: i32 = 3i32;
                                        let mut v243: i32 = 0i32;
                                        let mut v244: i32 = usize::try_from(v242).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v243).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                        let mut v245: bool = 48i32 <= v244;
                                        let mut v247: bool = if v245 {
                                            let mut v246: bool = v244 <= 57i32;
                                            v246
                                        } else {
                                            false
                                        };
                                        let mut v250: i32 = if v247 {
                                            let mut v248: i32 = v244 - 48;
                                            v248
                                        } else {
                                            let mut v249: i32 = -1;
                                            v249
                                        };
                                        let mut v251: i32 = 3i32;
                                        let mut v252: i32 = 1i32;
                                        let mut v253: i32 = usize::try_from(v251).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v252).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                        let mut v254: bool = 48i32 <= v253;
                                        let mut v256: bool = if v254 {
                                            let mut v255: bool = v253 <= 57i32;
                                            v255
                                        } else {
                                            false
                                        };
                                        let mut v259: i32 = if v256 {
                                            let mut v257: i32 = v253 - 48;
                                            v257
                                        } else {
                                            let mut v258: i32 = -1;
                                            v258
                                        };
                                        let mut v260: i32 = 3i32;
                                        let mut v261: i32 = 2i32;
                                        let mut v262: i32 = usize::try_from(v260).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v261).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                        let mut v263: bool = 48i32 <= v262;
                                        let mut v265: bool = if v263 {
                                            let mut v264: bool = v262 <= 57i32;
                                            v264
                                        } else {
                                            false
                                        };
                                        let mut v268: i32 = if v265 {
                                            let mut v266: i32 = v262 - 48;
                                            v266
                                        } else {
                                            let mut v267: i32 = -1;
                                            v267
                                        };
                                        let mut v269: bool = v241 == 1i32;
                                        let mut v270: bool = 0 <= v250;
                                        let mut v271: bool = v269 && v270;
                                        let mut v295: US4 = if v271 {
                                            US4::US4_Valid(v250)
                                        } else {
                                            let mut v273: bool = v241 == 2i32;
                                            let mut v274: bool = 0 <= v250;
                                            let mut v275: bool = 0 <= v259;
                                            let mut v277: bool = if v273 {
                                                let mut v276: bool = v274 && v275;
                                                v276
                                            } else {
                                                false
                                            };
                                            if v277 {
                                                let mut v278: i32 = v250 * 10 + v259;
                                                US4::US4_Valid(v278)
                                            } else {
                                                let mut v280: bool = v241 == 3i32;
                                                let mut v281: bool = 0 <= v250;
                                                let mut v282: bool = 0 <= v259;
                                                let mut v283: bool = 0 <= v268;
                                                let mut v286: bool = if v280 {
                                                    if v281 {
                                                        let mut v284: bool = v282 && v283;
                                                        v284
                                                    } else {
                                                        false
                                                    }
                                                } else {
                                                    false
                                                };
                                                if v286 {
                                                    let mut v287: i32 = v250 * 100 + v259 * 10 + v268;
                                                    let mut v288: bool = v287 <= 100i32;
                                                    if v288 {
                                                        US4::US4_Valid(v287)
                                                    } else {
                                                        US4::US4_Invalid
                                                    }
                                                } else {
                                                    US4::US4_Invalid
                                                }
                                            }
                                        };
                                        let mut v296: i32 = 4i32;
                                        let mut v297: Rc<str> = usize::try_from(v296).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v298: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
                                        let mut v299: bool = v297 == v298 ;
                                        let mut v303: US5 = if v299 {
                                            let mut v300: US6 = US6::US6_AgilePlanned;
                                            US5::US5_Valid(v300.clone())
                                        } else {
                                            US5::US5_Invalid
                                        };
                                        let mut v304: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
                                        let mut v305: bool = v297 == v304 ;
                                        let mut v308: US5 = if v305 {
                                            let mut v306: US6 = US6::US6_AgileActive;
                                            US5::US5_Valid(v306.clone())
                                        } else {
                                            v303.clone()
                                        };
                                        let mut v309: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
                                        let mut v310: bool = v297 == v309 ;
                                        let mut v313: US5 = if v310 {
                                            let mut v311: US6 = US6::US6_AgileBlocked;
                                            US5::US5_Valid(v311.clone())
                                        } else {
                                            v308.clone()
                                        };
                                        let mut v314: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) };
                                        let mut v315: bool = v297 == v314 ;
                                        let mut v318: US5 = if v315 {
                                            let mut v316: US6 = US6::US6_AgilePaused;
                                            US5::US5_Valid(v316.clone())
                                        } else {
                                            v313.clone()
                                        };
                                        let mut v319: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
                                        let mut v320: bool = v297 == v319 ;
                                        let mut v323: US5 = if v320 {
                                            let mut v321: US6 = US6::US6_AgileDone;
                                            US5::US5_Valid(v321.clone())
                                        } else {
                                            v318.clone()
                                        };
                                        match &v295 {
                                            US4::US4_Invalid => {
                                                let mut v356: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
                                                let mut v357: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a whole number 0..100"); } LIT.with(|lit| lit.clone()) };
                                                let mut v358: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set: "); } LIT.with(|lit| lit.clone()) };
                                                let mut v359: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" must be "); } LIT.with(|lit| lit.clone()) };
                                                let mut v360: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                                                let mut v361: i32 = 3i32;
                                                let mut v362: Rc<str> = usize::try_from(v361).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let mut v363: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                                let mut v364: Rc<str> = std::rc::Rc::<str>::from([&*v359, &*v357, &*v360, &*v362, &*v363].concat());
                                                let mut v365: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; usage: agile set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
                                                let mut v366: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                let mut v367: Rc<str> = std::rc::Rc::<str>::from([&*v358, &*v356, &*v364, &*v365, &*v366].concat());
                                                let mut v368: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v367 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                v368
                                            }
                                            US4::US4_Valid(v324) => {
                                                let mut v324: i32 = *v324;
                                                match &v323 {
                                                    US5::US5_Invalid => {
                                                        let mut v341: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v342: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned, Active, Blocked, Paused or Done"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v343: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set: "); } LIT.with(|lit| lit.clone()) };
                                                        let mut v344: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" must be "); } LIT.with(|lit| lit.clone()) };
                                                        let mut v345: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v346: i32 = 4i32;
                                                        let mut v347: Rc<str> = usize::try_from(v346).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                        let mut v348: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v349: Rc<str> = std::rc::Rc::<str>::from([&*v344, &*v342, &*v345, &*v347, &*v348].concat());
                                                        let mut v350: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; usage: agile set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v351: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        let mut v352: Rc<str> = std::rc::Rc::<str>::from([&*v343, &*v341, &*v349, &*v350, &*v351].concat());
                                                        let mut v353: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v352 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                        v353
                                                    }
                                                    US5::US5_Valid(v325) => {
                                                        let mut v325: US6 = v325.clone();
                                                        match &v325 {
                                                            US6::US6_AgileDone => {
                                                                let mut v326: Rc<str> = std::rc::Rc::<str>::from(agile_todo_scan::eoie_agile_todo_sites(&*v237, &*v239).trim_end());
                                                                let mut v327: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                let mut v328: bool = v326 == v327 ;
                                                                if v328 {
                                                                    let mut v329: i32 = eoie_agile_state::agile_set_validated_owned(v237.as_ref(), v239.as_ref(), v324, 4i32);
                                                                    v329
                                                                } else {
                                                                    let mut v330: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set "); } LIT.with(|lit| lit.clone()) };
                                                                    let mut v331: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: TODO blocks still name it; cut them or keep the item open:\n"); } LIT.with(|lit| lit.clone()) };
                                                                    let mut v332: Rc<str> = std::rc::Rc::<str>::from([&*v330, &*v239, &*v331, &*v326, &*v327].concat());
                                                                    let mut v333: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v332 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                    v333
                                                                }
                                                            }
                                                            _ => {
                                                                let mut v338: i32 = match &v325 {
                                                                    US6::US6_AgileActive => {
                                                                        1i32
                                                                    }
                                                                    US6::US6_AgileBlocked => {
                                                                        2i32
                                                                    }
                                                                    US6::US6_AgilePaused => {
                                                                        3i32
                                                                    }
                                                                    US6::US6_AgilePlanned => {
                                                                        0i32
                                                                    }
                                                                    _ => unreachable!(),
                                                                };
                                                                let mut v339: i32 = eoie_agile_state::agile_set_validated_owned(v237.as_ref(), v239.as_ref(), v324, v338);
                                                                v339
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        agile_usage_error_2()
                                    }
                                }
                            }
                            US1::US1_AgileShow => {
                                let mut v389: i32 = 3i32;
                                let mut v390: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v389 ;
                                let mut v393: bool = if v390 {
                                    true
                                } else {
                                    let mut v391: i32 = 4i32;
                                    let mut v392: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v391 ;
                                    v392
                                };
                                if v393 {
                                    let mut v394: i32 = agile_task_inspection::eoie_agile_show_run();
                                    v394
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileStatus => {
                                let mut v185: i32 = 2i32;
                                let mut v186: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v185 ;
                                if v186 {
                                    let mut v187: i32 = 1i32;
                                    let mut v188: Rc<str> = usize::try_from(v187).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v189: i32 = eoie_agile_state::agile_lease_status_owned(v188.as_ref());
                                    v189
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileTodos => {
                                let mut v433: i32 = std::env::args().skip(2).count() as i32;
                                let mut v434: bool = 2i32 <= v433;
                                if v434 {
                                    let mut v435: bool = std::env::args().skip(4).any(|argument| argument == "--record");
                                    if v435 {
                                        let mut v436: i32 = agile_todo_record::eoie_agile_todos_record_run();
                                        v436
                                    } else {
                                        let mut v437: i32 = agile_todo_scan::eoie_agile_todos_run();
                                        v437
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileView => {
                                let mut v461: i32 = std::env::args().skip(2).count() as i32;
                                let mut v462: bool = 2i32 <= v461;
                                if v462 {
                                    let mut v463: i32 = agile_task_views::eoie_agile_views_run();
                                    v463
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileViewAdd => {
                                let mut v466: i32 = std::env::args().skip(2).count() as i32;
                                let mut v467: bool = 2i32 <= v466;
                                if v467 {
                                    let mut v468: i32 = agile_task_views::eoie_agile_views_run();
                                    v468
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileViews => {
                                let mut v456: i32 = std::env::args().skip(2).count() as i32;
                                let mut v457: bool = 2i32 <= v456;
                                if v457 {
                                    let mut v458: i32 = agile_task_views::eoie_agile_views_run();
                                    v458
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                        };
                        state_lock_done_8(v155, v130.clone());
                        v495
                    } else {
                        state_lock_refused_1()
                    }
                }
            }
        }
        US0::US0_AgileUnknownName => {
            agile_usage_error_2()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_run() -> i32 {
    closure0()()
}
