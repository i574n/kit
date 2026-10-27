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
fn method7() -> i32 {
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
fn state_lock_released_9(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("state").join("agile.spi.lock")); };
    ()
}
fn state_lock_done_8(mut v0: bool, mut v1: Rc<str>) -> () {
    if v0 {
        state_lock_released_9(v1)
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
                    let v47: bool = match &v3 {
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
                    let v48: bool = state_lock_held_4(v47, v22.clone());
                    if v48 {
                        let v249: i32 = match &v3 {
                            US1::US1_AgileAdd => {
                                let v115: usize = std::env::args().count();
                                let v116: i32 = ((v115) as i32);
                                let v117: bool = v116 > 2i32;
                                let v119: i32 = if v117 {
                                    let v118: i32 = v116.wrapping_sub(2i32);
                                    v118
                                } else {
                                    0i32
                                };
                                let v120: bool = 8i32 <= v119;
                                if v120 {
                                    let v121: i32 = agile_task_creation::kit_agile_add_run();
                                    v121
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileBegin => {
                                let v56: i32 = 3i32;
                                let v57: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v56 ;
                                if v57 {
                                    let v58: i32 = kit_agile_routes::kit_agile_begin_route();
                                    v58
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileCheck => {
                                let v79: i32 = 2i32;
                                let v80: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v79 ;
                                if v80 {
                                    let v81: i32 = 1i32;
                                    let v82: Rc<str> = usize::try_from(v81).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v83: i32 = kit_agile_state::agile_check_owned(v82.as_ref());
                                    v83
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileEdit => {
                                let v124: i32 = 5i32;
                                let v125: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v124 ;
                                if v125 {
                                    let v126: i32 = kit_agile_routes::kit_agile_edit_route();
                                    v126
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileHandoff => {
                                let v63: i32 = 2i32;
                                let v64: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v63 ;
                                if v64 {
                                    let v65: i32 = 1i32;
                                    let v66: Rc<str> = usize::try_from(v65).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v67: i32 = kit_handoff_render::handoff_render_owned(v66.as_ref());
                                    v67
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileInit => {
                                let v49: i32 = 2i32;
                                let v50: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v49 ;
                                if v50 {
                                    let v51: i32 = 1i32;
                                    let v52: Rc<str> = usize::try_from(v51).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v53: i32 = agile_backlog_scaffold::kit_agile_init_run();
                                    v53
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLease => {
                                method6()
                            }
                            US1::US1_AgileLinks => {
                                let v213: usize = std::env::args().count();
                                let v214: i32 = ((v213) as i32);
                                let v215: bool = v214 > 2i32;
                                let v217: i32 = if v215 {
                                    let v216: i32 = v214.wrapping_sub(2i32);
                                    v216
                                } else {
                                    0i32
                                };
                                let v218: bool = 2i32 <= v217;
                                if v218 {
                                    let v219: i32 = agile_memory_links::kit_agile_links_run();
                                    v219
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileList => {
                                let v70: usize = std::env::args().count();
                                let v71: i32 = ((v70) as i32);
                                let v72: bool = v71 > 2i32;
                                let v74: i32 = if v72 {
                                    let v73: i32 = v71.wrapping_sub(2i32);
                                    v73
                                } else {
                                    0i32
                                };
                                let v75: bool = 2i32 <= v74;
                                if v75 {
                                    let v76: i32 = agile_task_listing::kit_agile_list_run();
                                    v76
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileMigrateTags => {
                                let v147: usize = std::env::args().count();
                                let v148: i32 = ((v147) as i32);
                                let v149: bool = v148 > 2i32;
                                let v151: i32 = if v149 {
                                    let v150: i32 = v148.wrapping_sub(2i32);
                                    v150
                                } else {
                                    0i32
                                };
                                let v152: bool = 2i32 <= v151;
                                if v152 {
                                    let v153: i32 = agile_planning_migration::kit_agile_migrate_tags_run();
                                    v153
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNext => {
                                let v195: usize = std::env::args().count();
                                let v196: i32 = ((v195) as i32);
                                let v197: bool = v196 > 2i32;
                                let v199: i32 = if v197 {
                                    let v198: i32 = v196.wrapping_sub(2i32);
                                    v198
                                } else {
                                    0i32
                                };
                                let v200: bool = 2i32 <= v199;
                                if v200 {
                                    let v201: i32 = agile_task_next::kit_agile_next_run();
                                    v201
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNote => {
                                let v156: i32 = 4i32;
                                let v157: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v156 ;
                                let v160: bool = if v157 {
                                    true
                                } else {
                                    let v158: i32 = 6i32;
                                    let v159: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v158 ;
                                    v159
                                };
                                if v160 {
                                    let v161: i32 = agile_task_notes::kit_agile_note_run();
                                    v161
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNoteEdit => {
                                let v164: usize = std::env::args().count();
                                let v165: i32 = ((v164) as i32);
                                let v166: bool = v165 > 2i32;
                                let v168: i32 = if v166 {
                                    let v167: i32 = v165.wrapping_sub(2i32);
                                    v167
                                } else {
                                    0i32
                                };
                                let v169: bool = 2i32 <= v168;
                                if v169 {
                                    let v170: i32 = agile_note_edit::kit_agile_note_edit_run();
                                    v170
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNotes => {
                                let v173: i32 = 4i32;
                                let v174: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v173 ;
                                if v174 {
                                    let v175: i32 = agile_title_notes::kit_agile_notes_from_title_run();
                                    v175
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgilePlan => {
                                let v138: usize = std::env::args().count();
                                let v139: i32 = ((v138) as i32);
                                let v140: bool = v139 > 2i32;
                                let v142: i32 = if v140 {
                                    let v141: i32 = v139.wrapping_sub(2i32);
                                    v141
                                } else {
                                    0i32
                                };
                                let v143: bool = 2i32 <= v142;
                                if v143 {
                                    let v144: i32 = agile_planning_rollup::kit_agile_planning_rollup_run();
                                    v144
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRecord => {
                                let v86: i32 = 2i32;
                                let v87: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v86 ;
                                if v87 {
                                    let v88: i32 = 1i32;
                                    let v89: Rc<str> = usize::try_from(v88).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v90: i32 = kit_agile_state::agile_record_owned(v89.as_ref());
                                    v90
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRename => {
                                let v178: i32 = 4i32;
                                let v179: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v178 ;
                                if v179 {
                                    let v180: i32 = agile_task_rename::kit_agile_rename_run();
                                    v180
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRoots => {
                                let v204: usize = std::env::args().count();
                                let v205: i32 = ((v204) as i32);
                                let v206: bool = v205 > 2i32;
                                let v208: i32 = if v206 {
                                    let v207: i32 = v205.wrapping_sub(2i32);
                                    v207
                                } else {
                                    0i32
                                };
                                let v209: bool = 2i32 <= v208;
                                if v209 {
                                    let v210: i32 = agile_todo_roots::kit_agile_roots_run();
                                    v210
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSet => {
                                let v93: i32 = 7i32;
                                let v94: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v93 ;
                                let v98: bool = if v94 {
                                    let v95: i32 = 5i32;
                                    let v96: Rc<str> = usize::try_from(v95).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v97: bool = &*v96 == "--as";
                                    v97
                                } else {
                                    false
                                };
                                if v98 {
                                    let v99: i32 = 6i32;
                                    let v100: Rc<str> = usize::try_from(v99).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set records no author (only agile note does); drop --as "); } LIT.with(|lit| lit.clone()) };
                                    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v100));
                                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", or add a note with --as to say who moved "); } LIT.with(|lit| lit.clone()) };
                                    let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v103));
                                    let v105: i32 = 2i32;
                                    let v106: Rc<str> = usize::try_from(v105).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v106));
                                    let v108: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v107 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v108
                                } else {
                                    let v109: i32 = 5i32;
                                    let v110: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v109 ;
                                    if v110 {
                                        let v111: i32 = kit_agile_routes::kit_agile_set_route();
                                        v111
                                    } else {
                                        agile_usage_error_2()
                                    }
                                }
                            }
                            US1::US1_AgileShow => {
                                let v129: usize = std::env::args().count();
                                let v130: i32 = ((v129) as i32);
                                let v131: bool = v130 > 2i32;
                                let v133: i32 = if v131 {
                                    let v132: i32 = v130.wrapping_sub(2i32);
                                    v132
                                } else {
                                    0i32
                                };
                                let v134: bool = 2i32 <= v133;
                                if v134 {
                                    let v135: i32 = agile_task_inspection::kit_agile_show_run();
                                    v135
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileStatus => {
                                method6()
                            }
                            US1::US1_AgileTodos => {
                                let v183: usize = std::env::args().count();
                                let v184: i32 = ((v183) as i32);
                                let v185: bool = v184 > 2i32;
                                let v187: i32 = if v185 {
                                    let v186: i32 = v184.wrapping_sub(2i32);
                                    v186
                                } else {
                                    0i32
                                };
                                let v188: bool = 2i32 <= v187;
                                if v188 {
                                    let v189: bool = std::env::args().skip(4).any(|argument| argument == "--record");
                                    if v189 {
                                        let v190: i32 = agile_todo_record::kit_agile_todos_record_run();
                                        v190
                                    } else {
                                        let v191: i32 = agile_todo_scan::kit_agile_todos_run();
                                        v191
                                    }
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileView => {
                                method7()
                            }
                            US1::US1_AgileViewAdd => {
                                method7()
                            }
                            US1::US1_AgileViews => {
                                method7()
                            }
                        };
                        state_lock_done_8(v47, v22.clone());
                        v249
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
pub fn kit_agile_run() -> i32 {
    closure0()()
}
