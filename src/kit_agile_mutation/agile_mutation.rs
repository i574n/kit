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
    let v86: bool = &*v0 == "roots";
    let v89: US0 = if v86 {
        let v87: US1 = US1::US1_AgileRoots;
        US0::US0_AgileKnown(v87)
    } else {
        v85
    };
    let v90: bool = &*v0 == "links";
    let v93: US0 = if v90 {
        let v91: US1 = US1::US1_AgileLinks;
        US0::US0_AgileKnown(v91)
    } else {
        v89
    };
    let v94: bool = &*v0 == "wip";
    let v97: US0 = if v94 {
        let v95: US1 = US1::US1_AgileWip;
        US0::US0_AgileKnown(v95)
    } else {
        v93
    };
    let v98: bool = &*v0 == "views";
    let v101: US0 = if v98 {
        let v99: US1 = US1::US1_AgileViews;
        US0::US0_AgileKnown(v99)
    } else {
        v97
    };
    let v102: bool = &*v0 == "view";
    let v105: US0 = if v102 {
        let v103: US1 = US1::US1_AgileView;
        US0::US0_AgileKnown(v103)
    } else {
        v101
    };
    let v106: bool = &*v0 == "view-add";
    if v106 {
        let v107: US1 = US1::US1_AgileViewAdd;
        US0::US0_AgileKnown(v107)
    } else {
        v105
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
                    let v49: bool = match &v3 {
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
                    let v50: bool = state_lock_held_4(v49, v22.clone());
                    if v50 {
                        let v272: i32 = match &v3 {
                            US1::US1_AgileAdd => {
                                let v117: usize = std::env::args().count();
                                let v118: i32 = ((v117) as i32);
                                let v119: bool = v118 > 2i32;
                                let v121: i32 = if v119 {
                                    let v120: i32 = v118.wrapping_sub(2i32);
                                    v120
                                } else {
                                    0i32
                                };
                                let v122: bool = 8i32 <= v121;
                                if v122 {
                                    let v123: i32 = agile_task_creation::kit_agile_add_run();
                                    v123
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileBegin => {
                                let v58: i32 = 3i32;
                                let v59: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v58 ;
                                if v59 {
                                    let v60: i32 = kit_agile_routes::kit_agile_begin_route();
                                    v60
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileCheck => {
                                let v81: i32 = 2i32;
                                let v82: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v81 ;
                                if v82 {
                                    let v83: i32 = 1i32;
                                    let v84: Rc<str> = usize::try_from(v83).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v85: i32 = kit_agile_state::agile_check_owned(v84.as_ref());
                                    v85
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileEdit => {
                                let v126: i32 = 5i32;
                                let v127: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v126 ;
                                if v127 {
                                    let v128: i32 = kit_agile_routes::kit_agile_edit_route();
                                    v128
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileHandoff => {
                                let v65: i32 = 2i32;
                                let v66: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v65 ;
                                if v66 {
                                    let v67: i32 = 1i32;
                                    let v68: Rc<str> = usize::try_from(v67).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v69: i32 = kit_handoff_render::handoff_render_owned(v68.as_ref());
                                    v69
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileInit => {
                                let v51: i32 = 2i32;
                                let v52: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v51 ;
                                if v52 {
                                    let v53: i32 = 1i32;
                                    let v54: Rc<str> = usize::try_from(v53).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v55: i32 = agile_backlog_scaffold::kit_agile_init_run();
                                    v55
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileLease => {
                                method6()
                            }
                            US1::US1_AgileLinks => {
                                let v225: usize = std::env::args().count();
                                let v226: i32 = ((v225) as i32);
                                let v227: bool = v226 > 2i32;
                                let v229: i32 = if v227 {
                                    let v228: i32 = v226.wrapping_sub(2i32);
                                    v228
                                } else {
                                    0i32
                                };
                                let v230: bool = 2i32 <= v229;
                                if v230 {
                                    let v231: i32 = agile_memory_links::kit_agile_links_run();
                                    v231
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileList => {
                                let v72: usize = std::env::args().count();
                                let v73: i32 = ((v72) as i32);
                                let v74: bool = v73 > 2i32;
                                let v76: i32 = if v74 {
                                    let v75: i32 = v73.wrapping_sub(2i32);
                                    v75
                                } else {
                                    0i32
                                };
                                let v77: bool = 2i32 <= v76;
                                if v77 {
                                    let v78: i32 = agile_task_listing::kit_agile_list_run();
                                    v78
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileMigrateTags => {
                                let v149: usize = std::env::args().count();
                                let v150: i32 = ((v149) as i32);
                                let v151: bool = v150 > 2i32;
                                let v153: i32 = if v151 {
                                    let v152: i32 = v150.wrapping_sub(2i32);
                                    v152
                                } else {
                                    0i32
                                };
                                let v154: bool = 2i32 <= v153;
                                if v154 {
                                    let v155: i32 = agile_planning_migration::kit_agile_migrate_tags_run();
                                    v155
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNext => {
                                let v207: usize = std::env::args().count();
                                let v208: i32 = ((v207) as i32);
                                let v209: bool = v208 > 2i32;
                                let v211: i32 = if v209 {
                                    let v210: i32 = v208.wrapping_sub(2i32);
                                    v210
                                } else {
                                    0i32
                                };
                                let v212: bool = 2i32 <= v211;
                                if v212 {
                                    let v213: i32 = agile_task_next::kit_agile_next_run();
                                    v213
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNote => {
                                let v158: i32 = 4i32;
                                let v159: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v158 ;
                                let v162: bool = if v159 {
                                    true
                                } else {
                                    let v160: i32 = 6i32;
                                    let v161: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v160 ;
                                    v161
                                };
                                if v162 {
                                    let v163: i32 = agile_task_notes::kit_agile_note_run();
                                    v163
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNoteEdit => {
                                let v166: usize = std::env::args().count();
                                let v167: i32 = ((v166) as i32);
                                let v168: bool = v167 > 2i32;
                                let v170: i32 = if v168 {
                                    let v169: i32 = v167.wrapping_sub(2i32);
                                    v169
                                } else {
                                    0i32
                                };
                                let v171: bool = 2i32 <= v170;
                                if v171 {
                                    let v172: i32 = agile_note_edit::kit_agile_note_edit_run();
                                    v172
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileNotes => {
                                let v175: i32 = 4i32;
                                let v176: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v175 ;
                                if v176 {
                                    let v177: i32 = agile_title_notes::kit_agile_notes_from_title_run();
                                    v177
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgilePlan => {
                                let v140: usize = std::env::args().count();
                                let v141: i32 = ((v140) as i32);
                                let v142: bool = v141 > 2i32;
                                let v144: i32 = if v142 {
                                    let v143: i32 = v141.wrapping_sub(2i32);
                                    v143
                                } else {
                                    0i32
                                };
                                let v145: bool = 2i32 <= v144;
                                if v145 {
                                    let v146: i32 = agile_planning_rollup::kit_agile_planning_rollup_run();
                                    v146
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRecord => {
                                let v88: i32 = 2i32;
                                let v89: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v88 ;
                                if v89 {
                                    let v90: i32 = 1i32;
                                    let v91: Rc<str> = usize::try_from(v90).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v92: i32 = kit_agile_state::agile_record_owned(v91.as_ref());
                                    v92
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRename => {
                                let v180: i32 = 4i32;
                                let v181: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v180 ;
                                if v181 {
                                    let v182: i32 = agile_task_rename::kit_agile_rename_run();
                                    v182
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileRoots => {
                                let v216: usize = std::env::args().count();
                                let v217: i32 = ((v216) as i32);
                                let v218: bool = v217 > 2i32;
                                let v220: i32 = if v218 {
                                    let v219: i32 = v217.wrapping_sub(2i32);
                                    v219
                                } else {
                                    0i32
                                };
                                let v221: bool = 2i32 <= v220;
                                if v221 {
                                    let v222: i32 = agile_todo_roots::kit_agile_roots_run();
                                    v222
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSeal => {
                                let v185: usize = std::env::args().count();
                                let v186: i32 = ((v185) as i32);
                                let v187: bool = v186 > 2i32;
                                let v189: i32 = if v187 {
                                    let v188: i32 = v186.wrapping_sub(2i32);
                                    v188
                                } else {
                                    0i32
                                };
                                let v190: bool = 2i32 <= v189;
                                if v190 {
                                    let v191: i32 = agile_note_seal::kit_agile_seal_run();
                                    v191
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileSet => {
                                let v95: i32 = 7i32;
                                let v96: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v95 ;
                                let v100: bool = if v96 {
                                    let v97: i32 = 5i32;
                                    let v98: Rc<str> = usize::try_from(v97).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v99: bool = &*v98 == "--as";
                                    v99
                                } else {
                                    false
                                };
                                if v100 {
                                    let v101: i32 = 6i32;
                                    let v102: Rc<str> = usize::try_from(v101).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set records no author (only agile note does); drop --as "); } LIT.with(|lit| lit.clone()) };
                                    let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v102));
                                    let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", or add a note with --as to say who moved "); } LIT.with(|lit| lit.clone()) };
                                    let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
                                    let v107: i32 = 2i32;
                                    let v108: Rc<str> = usize::try_from(v107).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v108));
                                    let v110: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v109 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v110
                                } else {
                                    let v111: i32 = 5i32;
                                    let v112: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v111 ;
                                    if v112 {
                                        let v113: i32 = kit_agile_routes::kit_agile_set_route();
                                        v113
                                    } else {
                                        agile_usage_error_2()
                                    }
                                }
                            }
                            US1::US1_AgileShow => {
                                let v131: usize = std::env::args().count();
                                let v132: i32 = ((v131) as i32);
                                let v133: bool = v132 > 2i32;
                                let v135: i32 = if v133 {
                                    let v134: i32 = v132.wrapping_sub(2i32);
                                    v134
                                } else {
                                    0i32
                                };
                                let v136: bool = 2i32 <= v135;
                                if v136 {
                                    let v137: i32 = agile_task_inspection::kit_agile_show_run();
                                    v137
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                            US1::US1_AgileStatus => {
                                method6()
                            }
                            US1::US1_AgileTodos => {
                                let v194: usize = std::env::args().count();
                                let v195: i32 = ((v194) as i32);
                                let v196: bool = v195 > 2i32;
                                let v198: i32 = if v196 {
                                    let v197: i32 = v195.wrapping_sub(2i32);
                                    v197
                                } else {
                                    0i32
                                };
                                let v199: bool = 2i32 <= v198;
                                if v199 {
                                    let v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--record"); } LIT.with(|lit| lit.clone()) };
                                    let v201: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v200);
                                    if v201 {
                                        let v202: i32 = agile_todo_record::kit_agile_todos_record_run();
                                        v202
                                    } else {
                                        let v203: i32 = agile_todo_scan::kit_agile_todos_run();
                                        v203
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
                            US1::US1_AgileWip => {
                                let v234: usize = std::env::args().count();
                                let v235: i32 = ((v234) as i32);
                                let v236: bool = v235 > 2i32;
                                let v238: i32 = if v236 {
                                    let v237: i32 = v235.wrapping_sub(2i32);
                                    v237
                                } else {
                                    0i32
                                };
                                let v239: bool = 2i32 <= v238;
                                if v239 {
                                    let v240: i32 = agile_todo_merge::kit_agile_wip_run();
                                    v240
                                } else {
                                    agile_usage_error_2()
                                }
                            }
                        };
                        state_lock_done_8(v49, v22.clone());
                        v272
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
