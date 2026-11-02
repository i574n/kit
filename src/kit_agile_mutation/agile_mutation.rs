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
fn state_lock_refused_1() -> i32 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile state/agile.spi stayed locked by another writer for 30 s; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
    let v1: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v0 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v1
}
fn agile_usage_error_2() -> i32 {
    let v0: i32 = kit_agile_routes::kit_agile_usage_error();
    v0
}
fn agile_subcommand_of_3(mut v0: Rc<str>) -> US0 {
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
fn state_lock_taken_5(mut v0: Rc<str>) -> bool {
    let v1: bool = { let state = std::path::Path::new(&*v0).join("state"); let lock = state.join("agile.spi.lock"); let started = std::time::Instant::now(); loop { if std::fs::create_dir(&lock).is_ok() { break true; } if !state.is_dir() { break true; } let stale = std::fs::metadata(&lock).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() >= 10).unwrap_or(false); if stale { let _ = std::fs::remove_dir(&lock); continue; } if started.elapsed() >= std::time::Duration::from_secs(30) { break false; } std::thread::sleep(std::time::Duration::from_millis(25)); } };
    v1
}
fn state_lock_held_4(mut v0: bool, mut v1: Rc<str>) -> bool {
    if v0 {
        state_lock_taken_5(v1)
    } else {
        true
    }
}
fn method6() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: bool = v1 > 2i32;
    let v4: i32 = if v2 {
        let v3: i32 = v1.wrapping_sub(2i32);
        v3
    } else {
        0i32
    };
    let v5: bool = 2i32 <= v4;
    if v5 {
        let v6: i32 = agile_claim_cli::kit_agile_claim_run();
        v6
    } else {
        agile_usage_error_2()
    }
}
fn method7() -> i32 {
    let v0: i32 = 2i32;
    let v1: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v0 ;
    if v1 {
        let v2: i32 = 1i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v4: i32 = kit_agile_state::agile_lease_status_owned(v3.as_ref());
        v4
    } else {
        agile_usage_error_2()
    }
}
fn method8() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: bool = v1 > 2i32;
    let v4: i32 = if v2 {
        let v3: i32 = v1.wrapping_sub(2i32);
        v3
    } else {
        0i32
    };
    let v5: bool = 2i32 <= v4;
    if v5 {
        let v6: i32 = agile_task_views::kit_agile_views_run();
        v6
    } else {
        agile_usage_error_2()
    }
}
fn state_lock_released_10(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("state").join("agile.spi.lock")); };
    ()
}
fn state_lock_done_9(mut v0: bool, mut v1: Rc<str>) -> () {
    if v0 {
        state_lock_released_10(v1)
    }
}
fn agile_entry_0() -> i32 {
    let v0: i32 = 0i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: US0 = agile_subcommand_of_3(v1.clone());
    match &v2 {
        US0::US0_AgileKnown(v3) => {
            let v3: US1 = v3.clone();
            let v8: US2 = match &v3 {
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
            let v10: i32 = match &v8 {
                US2::US2_RootAfterMode => {
                    2i32
                }
                US2::US2_RootLeads => {
                    1i32
                }
            };
            let v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            let v13: bool = v11.starts_with(&*v12);
            let v16: US3 = if v13 {
                US3::US3_RootFlagged(v11.clone())
            } else {
                US3::US3_RootNamed(v11.clone())
            };
            match &v16 {
                US3::US3_RootFlagged(v17) => {
                    let v17: Rc<str> = v17.clone();
                    let v18: i32 = 0i32;
                    let v19: Rc<str> = usize::try_from(v18).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let v20: Rc<str> = kit_command_help::kit_agile_root_flag_refusal(&*v19, &*v17);
                    let v21: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v20 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v21
                }
                US3::US3_RootNamed(v22) => {
                    let v22: Rc<str> = v22.clone();
                    let v51: bool = match &v3 {
                        US1::US1_AgileAdd => {
                            true
                        }
                        US1::US1_AgileBegin => {
                            true
                        }
                        US1::US1_AgileCheck => {
                            false
                        }
                        US1::US1_AgileClaim => {
                            true
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
                        US1::US1_AgileRelease => {
                            true
                        }
                        US1::US1_AgileRename => {
                            true
                        }
                        US1::US1_AgileRoots => {
                            true
                        }
                        US1::US1_AgileSeal => {
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
                        US1::US1_AgileWip => {
                            false
                        }
                    };
                    let v57: bool = if v51 {
                        true
                    } else {
                        match &v3 {
                            US1::US1_AgileClaim => {
                                true
                            }
                            US1::US1_AgileNext => {
                                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
                                let v53: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v52);
                                v53
                            }
                            US1::US1_AgileRelease => {
                                true
                            }
                            _ => {
                                false
                            }
                        }
                    };
                    let v58: bool = state_lock_held_4(v57, v22.clone());
                    if v58 {
                        let v288: i32 = match &v3 {
                            US1::US1_AgileAdd => {
                                let v125: usize = std::env::args().count();
                                let v126: i32 = ((v125) as i32);
                                let v127: bool = v126 > 2i32;
                                let v129: i32 = if v127 {
                                    let v128: i32 = v126.wrapping_sub(2i32);
                                    v128
                                } else {
                                    0i32
                                };
                                let v130: bool = 8i32 <= v129;
                                if v130 {
                                    let v131: i32 = agile_task_creation::kit_agile_add_run();
                                    v131
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileBegin => {
                                let v66: i32 = 3i32;
                                let v67: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v66 ;
                                if v67 {
                                    let v68: i32 = kit_agile_routes::kit_agile_begin_route();
                                    v68
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileCheck => {
                                let v89: i32 = 2i32;
                                let v90: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v89 ;
                                if v90 {
                                    let v91: i32 = 1i32;
                                    let v92: Rc<str> = usize::try_from(v91).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v93: i32 = kit_agile_state::agile_check_owned(v92.as_ref());
                                    v93
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileClaim => {
                                method6()
                            }
                            US1::US1_AgileEdit => {
                                let v134: i32 = 5i32;
                                let v135: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v134 ;
                                if v135 {
                                    let v136: i32 = kit_agile_routes::kit_agile_edit_route();
                                    v136
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileHandoff => {
                                let v73: i32 = 2i32;
                                let v74: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v73 ;
                                if v74 {
                                    let v75: i32 = 1i32;
                                    let v76: Rc<str> = usize::try_from(v75).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v77: i32 = kit_handoff_render::handoff_render_owned(v76.as_ref());
                                    v77
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileInit => {
                                let v59: i32 = 2i32;
                                let v60: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v59 ;
                                if v60 {
                                    let v61: i32 = 1i32;
                                    let v62: Rc<str> = usize::try_from(v61).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v63: i32 = agile_backlog_scaffold::kit_agile_init_run();
                                    v63
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLease => {
                                method7()
                            }
                            US1::US1_AgileLinks => {
                                let v239: usize = std::env::args().count();
                                let v240: i32 = ((v239) as i32);
                                let v241: bool = v240 > 2i32;
                                let v243: i32 = if v241 {
                                    let v242: i32 = v240.wrapping_sub(2i32);
                                    v242
                                } else {
                                    0i32
                                };
                                let v244: bool = 2i32 <= v243;
                                if v244 {
                                    let v245: i32 = agile_memory_links::kit_agile_links_run();
                                    v245
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileList => {
                                let v80: usize = std::env::args().count();
                                let v81: i32 = ((v80) as i32);
                                let v82: bool = v81 > 2i32;
                                let v84: i32 = if v82 {
                                    let v83: i32 = v81.wrapping_sub(2i32);
                                    v83
                                } else {
                                    0i32
                                };
                                let v85: bool = 2i32 <= v84;
                                if v85 {
                                    let v86: i32 = agile_task_listing::kit_agile_list_run();
                                    v86
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileMigrateTags => {
                                let v157: usize = std::env::args().count();
                                let v158: i32 = ((v157) as i32);
                                let v159: bool = v158 > 2i32;
                                let v161: i32 = if v159 {
                                    let v160: i32 = v158.wrapping_sub(2i32);
                                    v160
                                } else {
                                    0i32
                                };
                                let v162: bool = 2i32 <= v161;
                                if v162 {
                                    let v163: i32 = agile_planning_migration::kit_agile_migrate_tags_run();
                                    v163
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNext => {
                                let v215: usize = std::env::args().count();
                                let v216: i32 = ((v215) as i32);
                                let v217: bool = v216 > 2i32;
                                let v219: i32 = if v217 {
                                    let v218: i32 = v216.wrapping_sub(2i32);
                                    v218
                                } else {
                                    0i32
                                };
                                let v220: bool = 2i32 <= v219;
                                if v220 {
                                    let v221: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
                                    let v222: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v221);
                                    if v222 {
                                        let v223: i32 = agile_claim_cli::kit_agile_claim_run();
                                        v223
                                    } else {
                                        let v224: i32 = agile_task_next::kit_agile_next_run();
                                        v224
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNote => {
                                let v166: i32 = 4i32;
                                let v167: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v166 ;
                                let v170: bool = if v167 {
                                    true
                                } else {
                                    let v168: i32 = 6i32;
                                    let v169: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v168 ;
                                    v169
                                };
                                if v170 {
                                    let v171: i32 = agile_task_notes::kit_agile_note_run();
                                    v171
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNoteEdit => {
                                let v174: usize = std::env::args().count();
                                let v175: i32 = ((v174) as i32);
                                let v176: bool = v175 > 2i32;
                                let v178: i32 = if v176 {
                                    let v177: i32 = v175.wrapping_sub(2i32);
                                    v177
                                } else {
                                    0i32
                                };
                                let v179: bool = 2i32 <= v178;
                                if v179 {
                                    let v180: i32 = agile_note_edit::kit_agile_note_edit_run();
                                    v180
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNotes => {
                                let v183: i32 = 4i32;
                                let v184: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v183 ;
                                if v184 {
                                    let v185: i32 = agile_title_notes::kit_agile_notes_from_title_run();
                                    v185
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgilePlan => {
                                let v148: usize = std::env::args().count();
                                let v149: i32 = ((v148) as i32);
                                let v150: bool = v149 > 2i32;
                                let v152: i32 = if v150 {
                                    let v151: i32 = v149.wrapping_sub(2i32);
                                    v151
                                } else {
                                    0i32
                                };
                                let v153: bool = 2i32 <= v152;
                                if v153 {
                                    let v154: i32 = agile_planning_rollup::kit_agile_planning_rollup_run();
                                    v154
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRecord => {
                                let v96: i32 = 2i32;
                                let v97: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v96 ;
                                if v97 {
                                    let v98: i32 = 1i32;
                                    let v99: Rc<str> = usize::try_from(v98).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v100: i32 = kit_agile_state::agile_record_owned(v99.as_ref());
                                    v100
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRelease => {
                                method6()
                            }
                            US1::US1_AgileRename => {
                                let v188: i32 = 4i32;
                                let v189: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v188 ;
                                if v189 {
                                    let v190: i32 = agile_task_rename::kit_agile_rename_run();
                                    v190
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRoots => {
                                let v230: usize = std::env::args().count();
                                let v231: i32 = ((v230) as i32);
                                let v232: bool = v231 > 2i32;
                                let v234: i32 = if v232 {
                                    let v233: i32 = v231.wrapping_sub(2i32);
                                    v233
                                } else {
                                    0i32
                                };
                                let v235: bool = 2i32 <= v234;
                                if v235 {
                                    let v236: i32 = agile_todo_roots::kit_agile_roots_run();
                                    v236
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSeal => {
                                let v193: usize = std::env::args().count();
                                let v194: i32 = ((v193) as i32);
                                let v195: bool = v194 > 2i32;
                                let v197: i32 = if v195 {
                                    let v196: i32 = v194.wrapping_sub(2i32);
                                    v196
                                } else {
                                    0i32
                                };
                                let v198: bool = 2i32 <= v197;
                                if v198 {
                                    let v199: i32 = agile_note_seal::kit_agile_seal_run();
                                    v199
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSet => {
                                let v103: i32 = 7i32;
                                let v104: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v103 ;
                                let v108: bool = if v104 {
                                    let v105: i32 = 5i32;
                                    let v106: Rc<str> = usize::try_from(v105).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v107: bool = &*v106 == "--as";
                                    v107
                                } else {
                                    false
                                };
                                if v108 {
                                    let v109: i32 = 6i32;
                                    let v110: Rc<str> = usize::try_from(v109).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set records no author (only agile note does); drop --as "); } LIT.with(|lit| lit.clone()) };
                                    let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v110));
                                    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", or add a note with --as to say who moved "); } LIT.with(|lit| lit.clone()) };
                                    let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v113));
                                    let v115: i32 = 2i32;
                                    let v116: Rc<str> = usize::try_from(v115).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v116));
                                    let v118: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v117 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v118
                                } else {
                                    let v119: i32 = 5i32;
                                    let v120: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v119 ;
                                    if v120 {
                                        let v121: i32 = kit_agile_routes::kit_agile_set_route();
                                        v121
                                    } else {
                                        agile_usage_error_2()
                                    }
                                }
                            }
                            US1::US1_AgileShow => {
                                let v139: usize = std::env::args().count();
                                let v140: i32 = ((v139) as i32);
                                let v141: bool = v140 > 2i32;
                                let v143: i32 = if v141 {
                                    let v142: i32 = v140.wrapping_sub(2i32);
                                    v142
                                } else {
                                    0i32
                                };
                                let v144: bool = 2i32 <= v143;
                                if v144 {
                                    let v145: i32 = agile_task_inspection::kit_agile_show_run();
                                    v145
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileStatus => {
                                method7()
                            }
                            US1::US1_AgileTodos => {
                                let v202: usize = std::env::args().count();
                                let v203: i32 = ((v202) as i32);
                                let v204: bool = v203 > 2i32;
                                let v206: i32 = if v204 {
                                    let v205: i32 = v203.wrapping_sub(2i32);
                                    v205
                                } else {
                                    0i32
                                };
                                let v207: bool = 2i32 <= v206;
                                if v207 {
                                    let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--record"); } LIT.with(|lit| lit.clone()) };
                                    let v209: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v208);
                                    if v209 {
                                        let v210: i32 = agile_todo_record::kit_agile_todos_record_run();
                                        v210
                                    } else {
                                        let v211: i32 = agile_todo_scan::kit_agile_todos_run();
                                        v211
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileView => {
                                method8()
                            }
                            US1::US1_AgileViewAdd => {
                                method8()
                            }
                            US1::US1_AgileViews => {
                                method8()
                            }
                            US1::US1_AgileWip => {
                                let v248: usize = std::env::args().count();
                                let v249: i32 = ((v248) as i32);
                                let v250: bool = v249 > 2i32;
                                let v252: i32 = if v250 {
                                    let v251: i32 = v249.wrapping_sub(2i32);
                                    v251
                                } else {
                                    0i32
                                };
                                let v253: bool = 2i32 <= v252;
                                if v253 {
                                    let v254: i32 = agile_todo_merge::kit_agile_wip_run();
                                    v254
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                        };
                        state_lock_done_9(v57, v22.clone());
                        v288
                    } else {
                        let v293: bool = match &v3 {
                            US1::US1_AgileClaim => {
                                true
                            }
                            US1::US1_AgileNext => {
                                let v289: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
                                let v290: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v289);
                                v290
                            }
                            US1::US1_AgileRelease => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        if v293 {
                            let v294: i32 = 0i32;
                            let v295: Rc<str> = usize::try_from(v294).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v296: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile "); } LIT.with(|lit| lit.clone()) };
                            let v297: Rc<str> = Rc::<str>::from(format!("{}{}", v296, v295));
                            let v298: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" refused reason=contention"); } LIT.with(|lit| lit.clone()) };
                            let v299: Rc<str> = Rc::<str>::from(format!("{}{}", v297, v298));
                            let v300: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v299.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                            4i32
                        } else {
                            state_lock_refused_1()
                        }
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
pub fn kit_agile_run() -> i32 {
    closure0()()
}
