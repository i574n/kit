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
    US1_AgileMigrateActivity,
    US1_AgileSprint,
    US1_AgileLint,
    US1_AgileGate,
    US1_AgileCadence,
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
#[derive(Clone)]
enum US2 {
    US2_ReadsState,
    US2_WritesState,
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
    let v58: bool = &*v0 == "migrate-activity";
    let v61: US0 = if v58 {
        let v59: US1 = US1::US1_AgileMigrateActivity;
        US0::US0_AgileKnown(v59)
    } else {
        v57
    };
    let v62: bool = &*v0 == "sprint";
    let v65: US0 = if v62 {
        let v63: US1 = US1::US1_AgileSprint;
        US0::US0_AgileKnown(v63)
    } else {
        v61
    };
    let v66: bool = &*v0 == "lint";
    let v69: US0 = if v66 {
        let v67: US1 = US1::US1_AgileLint;
        US0::US0_AgileKnown(v67)
    } else {
        v65
    };
    let v70: bool = &*v0 == "gate";
    let v73: US0 = if v70 {
        let v71: US1 = US1::US1_AgileGate;
        US0::US0_AgileKnown(v71)
    } else {
        v69
    };
    let v74: bool = &*v0 == "cadence";
    let v77: US0 = if v74 {
        let v75: US1 = US1::US1_AgileCadence;
        US0::US0_AgileKnown(v75)
    } else {
        v73
    };
    let v78: bool = &*v0 == "note";
    let v81: US0 = if v78 {
        let v79: US1 = US1::US1_AgileNote;
        US0::US0_AgileKnown(v79)
    } else {
        v77
    };
    let v82: bool = &*v0 == "note-edit";
    let v85: US0 = if v82 {
        let v83: US1 = US1::US1_AgileNoteEdit;
        US0::US0_AgileKnown(v83)
    } else {
        v81
    };
    let v86: bool = &*v0 == "notes";
    let v89: US0 = if v86 {
        let v87: US1 = US1::US1_AgileNotes;
        US0::US0_AgileKnown(v87)
    } else {
        v85
    };
    let v90: bool = &*v0 == "rename";
    let v93: US0 = if v90 {
        let v91: US1 = US1::US1_AgileRename;
        US0::US0_AgileKnown(v91)
    } else {
        v89
    };
    let v94: bool = &*v0 == "seal";
    let v97: US0 = if v94 {
        let v95: US1 = US1::US1_AgileSeal;
        US0::US0_AgileKnown(v95)
    } else {
        v93
    };
    let v98: bool = &*v0 == "todos";
    let v101: US0 = if v98 {
        let v99: US1 = US1::US1_AgileTodos;
        US0::US0_AgileKnown(v99)
    } else {
        v97
    };
    let v102: bool = &*v0 == "next";
    let v105: US0 = if v102 {
        let v103: US1 = US1::US1_AgileNext;
        US0::US0_AgileKnown(v103)
    } else {
        v101
    };
    let v106: bool = &*v0 == "claim";
    let v109: US0 = if v106 {
        let v107: US1 = US1::US1_AgileClaim;
        US0::US0_AgileKnown(v107)
    } else {
        v105
    };
    let v110: bool = &*v0 == "release";
    let v113: US0 = if v110 {
        let v111: US1 = US1::US1_AgileRelease;
        US0::US0_AgileKnown(v111)
    } else {
        v109
    };
    let v114: bool = &*v0 == "roots";
    let v117: US0 = if v114 {
        let v115: US1 = US1::US1_AgileRoots;
        US0::US0_AgileKnown(v115)
    } else {
        v113
    };
    let v118: bool = &*v0 == "links";
    let v121: US0 = if v118 {
        let v119: US1 = US1::US1_AgileLinks;
        US0::US0_AgileKnown(v119)
    } else {
        v117
    };
    let v122: bool = &*v0 == "wip";
    let v125: US0 = if v122 {
        let v123: US1 = US1::US1_AgileWip;
        US0::US0_AgileKnown(v123)
    } else {
        v121
    };
    let v126: bool = &*v0 == "views";
    let v129: US0 = if v126 {
        let v127: US1 = US1::US1_AgileViews;
        US0::US0_AgileKnown(v127)
    } else {
        v125
    };
    let v130: bool = &*v0 == "view";
    let v133: US0 = if v130 {
        let v131: US1 = US1::US1_AgileView;
        US0::US0_AgileKnown(v131)
    } else {
        v129
    };
    let v134: bool = &*v0 == "view-add";
    if v134 {
        let v135: US1 = US1::US1_AgileViewAdd;
        US0::US0_AgileKnown(v135)
    } else {
        v133
    }
}
fn writes_named_0(mut v0: Rc<str>) -> bool {
    let v1: US0 = agile_subcommand_of_1(v0);
    match v1 {
        US0::US0_AgileKnown(v2) => {
            let v2: US1 = v2;
            let v70: US2 = match v2 {
                US1::US1_AgileAdd => {
                    US2::US2_WritesState
                }
                US1::US1_AgileBegin => {
                    US2::US2_WritesState
                }
                US1::US1_AgileCadence => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileCheck => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileClaim => {
                    US2::US2_WritesState
                }
                US1::US1_AgileEdit => {
                    US2::US2_WritesState
                }
                US1::US1_AgileGate => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileHandoff => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileInit => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileLease => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileLinks => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileLint => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileList => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileMigrateActivity => {
                    US2::US2_WritesState
                }
                US1::US1_AgileMigrateTags => {
                    US2::US2_WritesState
                }
                US1::US1_AgileNext => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileNote => {
                    US2::US2_WritesState
                }
                US1::US1_AgileNoteEdit => {
                    US2::US2_WritesState
                }
                US1::US1_AgileNotes => {
                    US2::US2_WritesState
                }
                US1::US1_AgilePlan => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileRecord => {
                    US2::US2_WritesState
                }
                US1::US1_AgileRelease => {
                    US2::US2_WritesState
                }
                US1::US1_AgileRename => {
                    US2::US2_WritesState
                }
                US1::US1_AgileRoots => {
                    US2::US2_WritesState
                }
                US1::US1_AgileSeal => {
                    US2::US2_WritesState
                }
                US1::US1_AgileSet => {
                    US2::US2_WritesState
                }
                US1::US1_AgileShow => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileSprint => {
                    US2::US2_WritesState
                }
                US1::US1_AgileStatus => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileTodos => {
                    US2::US2_WritesState
                }
                US1::US1_AgileView => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileViewAdd => {
                    US2::US2_WritesState
                }
                US1::US1_AgileViews => {
                    US2::US2_ReadsState
                }
                US1::US1_AgileWip => {
                    US2::US2_ReadsState
                }
            };
            match v70 {
                US2::US2_WritesState => {
                    true
                }
                _ => {
                    false
                }
            }
        }
        US0::US0_AgileUnknownName => {
            false
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        writes_named_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_writes_named(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
