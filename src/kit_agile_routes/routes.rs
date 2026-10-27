#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Valid(i32),
    US0_Invalid,
}
#[derive(Clone)]
enum US2 {
    US2_AgilePlanned,
    US2_AgileActive,
    US2_AgileBlocked,
    US2_AgilePaused,
    US2_AgileDone,
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
fn agile_subcommand_shape_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = &*v0 == "init";
    let mut v4: Rc<str> = if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    };
    let v5: bool = &*v4 == "";
    let mut v7: bool = if v5 {
        let v6: bool = &*v0 == "begin";
        v6
    } else {
        false
    };
    let v9: Rc<str> = if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <title>"); } LIT.with(|lit| lit.clone()) };
        v8.clone()
    } else {
        v4.clone()
    };
    let v10: bool = &*v9 == "";
    let v12: bool = if v10 {
        let v11: bool = &*v0 == "lease";
        v11
    } else {
        false
    };
    let v14: Rc<str> = if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v13.clone()
    } else {
        v9.clone()
    };
    let v15: bool = &*v14 == "";
    let v17: bool = if v15 {
        let v16: bool = &*v0 == "status";
        v16
    } else {
        false
    };
    let v19: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v18.clone()
    } else {
        v14.clone()
    };
    let v20: bool = &*v19 == "";
    let v22: bool = if v20 {
        let v21: bool = &*v0 == "handoff";
        v21
    } else {
        false
    };
    let v24: Rc<str> = if v22 {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v23.clone()
    } else {
        v19.clone()
    };
    let v25: bool = &*v24 == "";
    let v27: bool = if v25 {
        let v26: bool = &*v0 == "list";
        v26
    } else {
        false
    };
    let v29: Rc<str> = if v27 {
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json]"); } LIT.with(|lit| lit.clone()) };
        v28.clone()
    } else {
        v24.clone()
    };
    let v30: bool = &*v29 == "";
    let v32: bool = if v30 {
        let v31: bool = &*v0 == "check";
        v31
    } else {
        false
    };
    let v34: Rc<str> = if v32 {
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--compiler <absolute>]"); } LIT.with(|lit| lit.clone()) };
        v33.clone()
    } else {
        v29.clone()
    };
    let v35: bool = &*v34 == "";
    let v37: bool = if v35 {
        let v36: bool = &*v0 == "record";
        v36
    } else {
        false
    };
    let v39: Rc<str> = if v37 {
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v38.clone()
    } else {
        v34.clone()
    };
    let v40: bool = &*v39 == "";
    let v42: bool = if v40 {
        let v41: bool = &*v0 == "set";
        v41
    } else {
        false
    };
    let v44: Rc<str> = if v42 {
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
        v43.clone()
    } else {
        v39.clone()
    };
    let v45: bool = &*v44 == "";
    let v47: bool = if v45 {
        let v46: bool = &*v0 == "add";
        v46
    } else {
        false
    };
    let v49: Rc<str> = if v47 {
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>]"); } LIT.with(|lit| lit.clone()) };
        v48.clone()
    } else {
        v44.clone()
    };
    let v50: bool = &*v49 == "";
    let v52: bool = if v50 {
        let v51: bool = &*v0 == "edit";
        v51
    } else {
        false
    };
    let v54: Rc<str> = if v52 {
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>"); } LIT.with(|lit| lit.clone()) };
        v53.clone()
    } else {
        v49.clone()
    };
    let v55: bool = &*v54 == "";
    let v57: bool = if v55 {
        let v56: bool = &*v0 == "show";
        v56
    } else {
        false
    };
    let v59: Rc<str> = if v57 {
        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
        v58.clone()
    } else {
        v54.clone()
    };
    let v60: bool = &*v59 == "";
    let v62: bool = if v60 {
        let v61: bool = &*v0 == "plan";
        v61
    } else {
        false
    };
    let v64: Rc<str> = if v62 {
        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--json]"); } LIT.with(|lit| lit.clone()) };
        v63.clone()
    } else {
        v59.clone()
    };
    let v65: bool = &*v64 == "";
    let v67: bool = if v65 {
        let v66: bool = &*v0 == "migrate-tags";
        v66
    } else {
        false
    };
    let v69: Rc<str> = if v67 {
        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
        v68.clone()
    } else {
        v64.clone()
    };
    let v70: bool = &*v69 == "";
    let v72: bool = if v70 {
        let v71: bool = &*v0 == "note";
        v71
    } else {
        false
    };
    let v74: Rc<str> = if v72 {
        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
        v73.clone()
    } else {
        v69.clone()
    };
    let v75: bool = &*v74 == "";
    let v77: bool = if v75 {
        let v76: bool = &*v0 == "note-edit";
        v76
    } else {
        false
    };
    let v79: Rc<str> = if v77 {
        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>]"); } LIT.with(|lit| lit.clone()) };
        v78.clone()
    } else {
        v74.clone()
    };
    let v80: bool = &*v79 == "";
    let v82: bool = if v80 {
        let v81: bool = &*v0 == "notes";
        v81
    } else {
        false
    };
    let v84: Rc<str> = if v82 {
        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
        v83.clone()
    } else {
        v79.clone()
    };
    let v85: bool = &*v84 == "";
    let v87: bool = if v85 {
        let v86: bool = &*v0 == "rename";
        v86
    } else {
        false
    };
    let v89: Rc<str> = if v87 {
        let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
        v88.clone()
    } else {
        v84.clone()
    };
    let v90: bool = &*v89 == "";
    let v92: bool = if v90 {
        let v91: bool = &*v0 == "todos";
        v91
    } else {
        false
    };
    let v94: Rc<str> = if v92 {
        let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record"); } LIT.with(|lit| lit.clone()) };
        v93.clone()
    } else {
        v89.clone()
    };
    let v95: bool = &*v94 == "";
    let v97: bool = if v95 {
        let v96: bool = &*v0 == "next";
        v96
    } else {
        false
    };
    let v99: Rc<str> = if v97 {
        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count]"); } LIT.with(|lit| lit.clone()) };
        v98.clone()
    } else {
        v94.clone()
    };
    let v100: bool = &*v99 == "";
    let v102: bool = if v100 {
        let v101: bool = &*v0 == "roots";
        v101
    } else {
        false
    };
    let v104: Rc<str> = if v102 {
        let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [add|drop <path>...]"); } LIT.with(|lit| lit.clone()) };
        v103.clone()
    } else {
        v99.clone()
    };
    let v105: bool = &*v104 == "";
    let v107: bool = if v105 {
        let v106: bool = &*v0 == "links";
        v106
    } else {
        false
    };
    let v109: Rc<str> = if v107 {
        let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --memory <dir> [--json]"); } LIT.with(|lit| lit.clone()) };
        v108.clone()
    } else {
        v104.clone()
    };
    let v110: bool = &*v109 == "";
    let v112: bool = if v110 {
        let v111: bool = &*v0 == "views";
        v111
    } else {
        false
    };
    let v114: Rc<str> = if v112 {
        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
        v113.clone()
    } else {
        v109.clone()
    };
    let v115: bool = &*v114 == "";
    let v117: bool = if v115 {
        let v116: bool = &*v0 == "view";
        v116
    } else {
        false
    };
    let v119: Rc<str> = if v117 {
        let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
        v118.clone()
    } else {
        v114.clone()
    };
    let v120: bool = &*v119 == "";
    let v122: bool = if v120 {
        let v121: bool = &*v0 == "view-add";
        v121
    } else {
        false
    };
    if v122 {
        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
        v123.clone()
    } else {
        v119.clone()
    }
}
fn agile_usage_error_0() -> i32 {
    let v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: Rc<str> = agile_subcommand_shape_1(v1.clone());
    let v3: bool = &*v2 == "";
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root> [--compiler <absolute>], record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>], edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>], notes from-title <root> <id>, rename <root> <old-id> <new-id>, todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record, next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count], roots <root> [add|drop <path>...], links <root> --memory <dir> [--json], views <root>, view <root> <name> [extra flags...], view-add <root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v4 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v5
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v1));
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v8)), v2));
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
        let v13: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see kit agile --help"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
        let v17: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v16 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v17
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_usage_error_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let v0: i32 = 1i32;
        let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v2: i32 = 2i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: i32 = kit_agile_lease::kit_lease_title_words();
        let v5: bool = 0 < v4;
        if v5 {
            let v6: i32 = kit_agile_state::agile_begin_owned(v1.as_ref(), v3.as_ref());
            v6
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin title must contain 3..7 visible words"); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v7 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v8
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let v0: i32 = 1i32;
        let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v2: i32 = 2i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: i32 = 3i32;
        let v5: i32 = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
        let v6: i32 = 3i32;
        let mut v7: i32 = 0i32;
        let v8: i32 = usize::try_from(v6).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v7).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
        let v9: bool = 48i32 <= v8;
        let v11: bool = if v9 {
            let v10: bool = v8 <= 57i32;
            v10
        } else {
            false
        };
        let v14: i32 = if v11 {
            let v12: i32 = v8 - 48;
            v12
        } else {
            let v13: i32 = -1;
            v13
        };
        let v15: i32 = 3i32;
        let v16: i32 = 1i32;
        let v17: i32 = usize::try_from(v15).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v16).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
        let v18: bool = 48i32 <= v17;
        let v20: bool = if v18 {
            let v19: bool = v17 <= 57i32;
            v19
        } else {
            false
        };
        let v23: i32 = if v20 {
            let v21: i32 = v17 - 48;
            v21
        } else {
            let v22: i32 = -1;
            v22
        };
        let v24: i32 = 3i32;
        let v25: i32 = 2i32;
        let v26: i32 = usize::try_from(v24).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v25).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
        let v27: bool = 48i32 <= v26;
        let v29: bool = if v27 {
            let v28: bool = v26 <= 57i32;
            v28
        } else {
            false
        };
        let v32: i32 = if v29 {
            let v30: i32 = v26 - 48;
            v30
        } else {
            let v31: i32 = -1;
            v31
        };
        let v33: bool = v5 == 1i32;
        let v34: bool = 0 <= v14;
        let v35: bool = v33 && v34;
        let v59: US0 = if v35 {
            US0::US0_Valid(v14)
        } else {
            let v37: bool = v5 == 2i32;
            let v38: bool = 0 <= v14;
            let v39: bool = 0 <= v23;
            let v41: bool = if v37 {
                let v40: bool = v38 && v39;
                v40
            } else {
                false
            };
            if v41 {
                let v42: i32 = v14 * 10 + v23;
                US0::US0_Valid(v42)
            } else {
                let v44: bool = v5 == 3i32;
                let v45: bool = 0 <= v14;
                let v46: bool = 0 <= v23;
                let v47: bool = 0 <= v32;
                let v50: bool = if v44 {
                    if v45 {
                        let v48: bool = v46 && v47;
                        v48
                    } else {
                        false
                    }
                } else {
                    false
                };
                if v50 {
                    let v51: i32 = v14 * 100 + v23 * 10 + v32;
                    let v52: bool = v51 <= 100i32;
                    if v52 {
                        US0::US0_Valid(v51)
                    } else {
                        US0::US0_Invalid
                    }
                } else {
                    US0::US0_Invalid
                }
            }
        };
        let v60: i32 = 4i32;
        let v61: Rc<str> = usize::try_from(v60).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v62: bool = &*v61 == "Planned";
        let v66: US1 = if v62 {
            let v63: US2 = US2::US2_AgilePlanned;
            US1::US1_Valid(v63.clone())
        } else {
            US1::US1_Invalid
        };
        let v67: bool = &*v61 == "Active";
        let v70: US1 = if v67 {
            let v68: US2 = US2::US2_AgileActive;
            US1::US1_Valid(v68.clone())
        } else {
            v66.clone()
        };
        let v71: bool = &*v61 == "Blocked";
        let v74: US1 = if v71 {
            let v72: US2 = US2::US2_AgileBlocked;
            US1::US1_Valid(v72.clone())
        } else {
            v70.clone()
        };
        let v75: bool = &*v61 == "Paused";
        let v78: US1 = if v75 {
            let v76: US2 = US2::US2_AgilePaused;
            US1::US1_Valid(v76.clone())
        } else {
            v74.clone()
        };
        let v79: bool = &*v61 == "Done";
        let v82: US1 = if v79 {
            let v80: US2 = US2::US2_AgileDone;
            US1::US1_Valid(v80.clone())
        } else {
            v78.clone()
        };
        match &v59 {
            US0::US0_Invalid => {
                let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
                let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a whole number 0..100"); } LIT.with(|lit| lit.clone()) };
                let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set: "); } LIT.with(|lit| lit.clone()) };
                let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set: progress"); } LIT.with(|lit| lit.clone()) };
                let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" must be "); } LIT.with(|lit| lit.clone()) };
                let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" must be a whole number 0..100"); } LIT.with(|lit| lit.clone()) };
                let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" must be a whole number 0..100, received '"); } LIT.with(|lit| lit.clone()) };
                let v128: i32 = 3i32;
                let v129: Rc<str> = usize::try_from(v128).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v129));
                let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v123, Rc::<str>::from(format!("{}{}", v130, v131))));
                let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; usage: agile set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
                let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v134));
                let v136: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v135 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v136
            }
            US0::US0_Valid(v83) => {
                let v83: i32 = *v83;
                match &v82 {
                    US1::US1_Invalid => {
                        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                        let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned, Active, Blocked, Paused or Done"); } LIT.with(|lit| lit.clone()) };
                        let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set: "); } LIT.with(|lit| lit.clone()) };
                        let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set: status"); } LIT.with(|lit| lit.clone()) };
                        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" must be "); } LIT.with(|lit| lit.clone()) };
                        let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" must be Planned, Active, Blocked, Paused or Done"); } LIT.with(|lit| lit.clone()) };
                        let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                        let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" must be Planned, Active, Blocked, Paused or Done, received '"); } LIT.with(|lit| lit.clone()) };
                        let v109: i32 = 4i32;
                        let v110: Rc<str> = usize::try_from(v109).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v108, v110));
                        let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                        let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v104, Rc::<str>::from(format!("{}{}", v111, v112))));
                        let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; usage: agile set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
                        let v116: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v115));
                        let v117: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v116 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v117
                    }
                    US1::US1_Valid(v84) => {
                        let v84: US2 = v84.clone();
                        match &v84 {
                            US2::US2_AgileDone => {
                                let v85: Rc<str> = std::rc::Rc::<str>::from(agile_todo_scan::kit_agile_todo_sites(&*v1, &*v3).trim_end());
                                let v86: bool = &*v85 == "";
                                if v86 {
                                    let v87: i32 = kit_agile_state::agile_set_validated_owned(v1.as_ref(), v3.as_ref(), v83, 4i32);
                                    v87
                                } else {
                                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set "); } LIT.with(|lit| lit.clone()) };
                                    let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v3));
                                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: TODO blocks still name it; cut them or keep the item open:\n"); } LIT.with(|lit| lit.clone()) };
                                    let v92: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v89, v90)), v85));
                                    let v93: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v92 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v93
                                }
                            }
                            _ => {
                                let v98: i32 = match &v84 {
                                    US2::US2_AgileActive => {
                                        1i32
                                    }
                                    US2::US2_AgileBlocked => {
                                        2i32
                                    }
                                    US2::US2_AgilePaused => {
                                        3i32
                                    }
                                    US2::US2_AgilePlanned => {
                                        0i32
                                    }
                                    _ => unreachable!(),
                                };
                                let v99: i32 = kit_agile_state::agile_set_validated_owned(v1.as_ref(), v3.as_ref(), v83, v98);
                                v99
                            }
                        }
                    }
                }
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_field_named_2(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = std::rc::Rc::<str>::from(v1.to_ascii_lowercase());
    let v3: bool = &*v0 == &*v2;
    if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = std::rc::Rc::<str>::from(v4.to_ascii_lowercase());
        let v6: bool = &*v0 == &*v5;
        if v6 {
            true
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
            let v9: bool = &*v0 == &*v8;
            v9
        }
    }
}
fn closure3() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let v0: i32 = 3i32;
        let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v2: bool = planning_field_named_2(v1.clone());
        if v2 {
            let v3: i32 = agile_planning_revision::kit_agile_planning_edit_run();
            v3
        } else {
            let mut v4: i32 = agile_task_revision::kit_agile_edit_run();
            v4
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_usage_error() -> i32 {
    closure0()()
}
pub fn kit_agile_begin_route() -> i32 {
    closure1()()
}
pub fn kit_agile_set_route() -> i32 {
    closure2()()
}
pub fn kit_agile_edit_route() -> i32 {
    closure3()()
}
