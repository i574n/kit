#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_EditTitle,
    US1_AppendTitle,
    US1_EditKind,
    US1_EditPriority,
    US1_EditDifficulty,
    US1_EditDeps,
    US1_EditTags,
}
#[derive(Clone)]
enum US0 {
    US0_Valid(US1),
    US0_Invalid,
}
#[derive(Clone)]
enum US2 {
    US2_Valid(Rc<str>),
    US2_Invalid,
}
#[derive(Clone)]
enum US3 {
    US3_OpeningSentence,
    US3_FollowingSentence,
    US3_CallerPunctuated,
}
#[derive(Clone)]
enum US5 {
    US5_AgileD0,
    US5_AgileD1,
    US5_AgileD2,
    US5_AgileD3,
    US5_AgileD4,
    US5_AgileD5,
    US5_AgileD6,
    US5_AgileD7,
    US5_AgileD8,
}
#[derive(Clone)]
enum US4 {
    US4_Valid(US5),
    US4_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_AgileEpic,
    US7_AgileFeature,
    US7_AgileStory,
    US7_AgileTaskKind,
}
#[derive(Clone)]
enum US6 {
    US6_Valid(US7),
    US6_Invalid,
}
#[derive(Clone)]
enum US9 {
    US9_AgileP0,
    US9_AgileP1,
    US9_AgileP2,
    US9_AgileP3,
    US9_AgileP4,
    US9_AgileP5,
}
#[derive(Clone)]
enum US8 {
    US8_Valid(US9),
    US8_Invalid,
}
fn parse_edit_field_1(mut v0: Rc<str>) -> US0 {
    let v1: bool = &*v0 == "title";
    let v5: US0 = if v1 {
        let v2: US1 = US1::US1_EditTitle;
        US0::US0_Valid(v2)
    } else {
        US0::US0_Invalid
    };
    let v6: bool = &*v0 == "title-append";
    let v9: US0 = if v6 {
        let v7: US1 = US1::US1_AppendTitle;
        US0::US0_Valid(v7)
    } else {
        v5
    };
    let v10: bool = &*v0 == "kind";
    let v13: US0 = if v10 {
        let v11: US1 = US1::US1_EditKind;
        US0::US0_Valid(v11)
    } else {
        v9
    };
    let v14: bool = &*v0 == "priority";
    let v17: US0 = if v14 {
        let v15: US1 = US1::US1_EditPriority;
        US0::US0_Valid(v15)
    } else {
        v13
    };
    let v18: bool = &*v0 == "difficulty";
    let v21: US0 = if v18 {
        let v19: US1 = US1::US1_EditDifficulty;
        US0::US0_Valid(v19)
    } else {
        v17
    };
    let v22: bool = &*v0 == "deps";
    let v25: US0 = if v22 {
        let v23: US1 = US1::US1_EditDeps;
        US0::US0_Valid(v23)
    } else {
        v21
    };
    let v26: bool = &*v0 == "tags";
    if v26 {
        let v27: US1 = US1::US1_EditTags;
        US0::US0_Valid(v27)
    } else {
        v25
    }
}
fn edit_field_position_0(mut v0: Rc<str>) -> i32 {
    let v1: US0 = parse_edit_field_1(v0);
    match &v1 {
        US0::US0_Invalid => {
            -1i32
        }
        US0::US0_Valid(v2) => {
            let v2: US1 = v2.clone();
            match &v2 {
                US1::US1_AppendTitle => {
                    1i32
                }
                US1::US1_EditDeps => {
                    7i32
                }
                US1::US1_EditDifficulty => {
                    4i32
                }
                US1::US1_EditKind => {
                    2i32
                }
                US1::US1_EditPriority => {
                    3i32
                }
                US1::US1_EditTags => {
                    8i32
                }
                US1::US1_EditTitle => {
                    1i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        edit_field_position_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn quoted_field_value_3(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v4.replace(&*v1, &*v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v8));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v11));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v15, &*v1, &*v16, &*v16].concat());
    US2::US2_Valid(v17)
}
fn edit_field_token_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: US0 = parse_edit_field_1(v0.clone());
    match &v3 {
        US0::US0_Invalid => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_Valid(v5) => {
            let v5: US1 = v5.clone();
            let v174: US2 = match &v5 {
                US1::US1_AppendTitle => {
                    let v9: i32 = (v1.clone().len() as i32);
                    let v10: i32 = v9.wrapping_sub(1i32);
                    let v11: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v10 as usize).unwrap_or(""));
                    let v12: i32 = (v11.clone().len() as i32);
                    let v13: bool = v12 <= 1i32;
                    let v26: US3 = if v13 {
                        US3::US3_OpeningSentence
                    } else {
                        let v15: i32 = v2.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                        let v16: bool = v15 == 59i32;
                        let v22: bool = if v16 {
                            true
                        } else {
                            let v17: bool = v15 == 44i32;
                            if v17 {
                                true
                            } else {
                                let v18: bool = v15 == 46i32;
                                if v18 {
                                    true
                                } else {
                                    let v19: bool = v15 == 32i32;
                                    v19
                                }
                            }
                        };
                        if v22 {
                            US3::US3_CallerPunctuated
                        } else {
                            US3::US3_FollowingSentence
                        }
                    };
                    let v32: Rc<str> = match &v26 {
                        US3::US3_CallerPunctuated => {
                            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v28.clone()
                        }
                        US3::US3_FollowingSentence => {
                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                            v29.clone()
                        }
                        US3::US3_OpeningSentence => {
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v27.clone()
                        }
                    };
                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v32));
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = Rc::<str>::from(v2.replace(&*v34, &*v35));
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<str> = Rc::<str>::from(v36.replace(&*v37, &*v38));
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                    let v42: Rc<str> = Rc::<str>::from(v39.replace(&*v40, &*v41));
                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                    let v45: Rc<str> = Rc::<str>::from(v42.replace(&*v43, &*v44));
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = Rc::<str>::from(v45.replace(&*v46, &*v47));
                    let v50: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v33, v48)), v37));
                    US2::US2_Valid(v50.clone())
                }
                US1::US1_EditDeps => {
                    quoted_field_value_3(v2.clone())
                }
                US1::US1_EditDifficulty => {
                    let v116: bool = &*v2 == "D0";
                    let v120: US4 = if v116 {
                        let v117: US5 = US5::US5_AgileD0;
                        US4::US4_Valid(v117.clone())
                    } else {
                        US4::US4_Invalid
                    };
                    let v121: bool = &*v2 == "D1";
                    let v124: US4 = if v121 {
                        let v122: US5 = US5::US5_AgileD1;
                        US4::US4_Valid(v122.clone())
                    } else {
                        v120.clone()
                    };
                    let v125: bool = &*v2 == "D2";
                    let v128: US4 = if v125 {
                        let v126: US5 = US5::US5_AgileD2;
                        US4::US4_Valid(v126.clone())
                    } else {
                        v124.clone()
                    };
                    let v129: bool = &*v2 == "D3";
                    let v132: US4 = if v129 {
                        let v130: US5 = US5::US5_AgileD3;
                        US4::US4_Valid(v130.clone())
                    } else {
                        v128.clone()
                    };
                    let v133: bool = &*v2 == "D4";
                    let v136: US4 = if v133 {
                        let v134: US5 = US5::US5_AgileD4;
                        US4::US4_Valid(v134.clone())
                    } else {
                        v132.clone()
                    };
                    let v137: bool = &*v2 == "D5";
                    let v140: US4 = if v137 {
                        let v138: US5 = US5::US5_AgileD5;
                        US4::US4_Valid(v138.clone())
                    } else {
                        v136.clone()
                    };
                    let v141: bool = &*v2 == "D6";
                    let v144: US4 = if v141 {
                        let v142: US5 = US5::US5_AgileD6;
                        US4::US4_Valid(v142.clone())
                    } else {
                        v140.clone()
                    };
                    let v145: bool = &*v2 == "D7";
                    let v148: US4 = if v145 {
                        let v146: US5 = US5::US5_AgileD7;
                        US4::US4_Valid(v146.clone())
                    } else {
                        v144.clone()
                    };
                    let v149: bool = &*v2 == "D8";
                    let v152: US4 = if v149 {
                        let v150: US5 = US5::US5_AgileD8;
                        US4::US4_Valid(v150.clone())
                    } else {
                        v148.clone()
                    };
                    match &v152 {
                        US4::US4_Invalid => {
                            US2::US2_Invalid
                        }
                        US4::US4_Valid(v153) => {
                            let v153: US5 = v153.clone();
                            let v163: Rc<str> = match &v153 {
                                US5::US5_AgileD0 => {
                                    let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                                    v154.clone()
                                }
                                US5::US5_AgileD1 => {
                                    let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                                    v155.clone()
                                }
                                US5::US5_AgileD2 => {
                                    let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                                    v156.clone()
                                }
                                US5::US5_AgileD3 => {
                                    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                                    v157.clone()
                                }
                                US5::US5_AgileD4 => {
                                    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                                    v158.clone()
                                }
                                US5::US5_AgileD5 => {
                                    let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                                    v159.clone()
                                }
                                US5::US5_AgileD6 => {
                                    let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                                    v160.clone()
                                }
                                US5::US5_AgileD7 => {
                                    let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                                    v161.clone()
                                }
                                US5::US5_AgileD8 => {
                                    let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                                    v162.clone()
                                }
                            };
                            US2::US2_Valid(v163.clone())
                        }
                    }
                }
                US1::US1_EditKind => {
                    let v52: bool = &*v2 == "Epic";
                    let v56: US6 = if v52 {
                        let v53: US7 = US7::US7_AgileEpic;
                        US6::US6_Valid(v53.clone())
                    } else {
                        US6::US6_Invalid
                    };
                    let v57: bool = &*v2 == "Feature";
                    let v60: US6 = if v57 {
                        let v58: US7 = US7::US7_AgileFeature;
                        US6::US6_Valid(v58.clone())
                    } else {
                        v56.clone()
                    };
                    let v61: bool = &*v2 == "Story";
                    let v64: US6 = if v61 {
                        let v62: US7 = US7::US7_AgileStory;
                        US6::US6_Valid(v62.clone())
                    } else {
                        v60.clone()
                    };
                    let v65: bool = &*v2 == "TaskKind";
                    let v68: US6 = if v65 {
                        let v66: US7 = US7::US7_AgileTaskKind;
                        US6::US6_Valid(v66.clone())
                    } else {
                        v64.clone()
                    };
                    match &v68 {
                        US6::US6_Invalid => {
                            US2::US2_Invalid
                        }
                        US6::US6_Valid(v69) => {
                            let v69: US7 = v69.clone();
                            let v74: Rc<str> = match &v69 {
                                US7::US7_AgileEpic => {
                                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                                    v70.clone()
                                }
                                US7::US7_AgileFeature => {
                                    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                                    v71.clone()
                                }
                                US7::US7_AgileStory => {
                                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                                    v72.clone()
                                }
                                US7::US7_AgileTaskKind => {
                                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                                    v73.clone()
                                }
                            };
                            US2::US2_Valid(v74.clone())
                        }
                    }
                }
                US1::US1_EditPriority => {
                    let v79: bool = &*v2 == "P0";
                    let v83: US8 = if v79 {
                        let v80: US9 = US9::US9_AgileP0;
                        US8::US8_Valid(v80.clone())
                    } else {
                        US8::US8_Invalid
                    };
                    let v84: bool = &*v2 == "P1";
                    let v87: US8 = if v84 {
                        let v85: US9 = US9::US9_AgileP1;
                        US8::US8_Valid(v85.clone())
                    } else {
                        v83.clone()
                    };
                    let v88: bool = &*v2 == "P2";
                    let v91: US8 = if v88 {
                        let v89: US9 = US9::US9_AgileP2;
                        US8::US8_Valid(v89.clone())
                    } else {
                        v87.clone()
                    };
                    let v92: bool = &*v2 == "P3";
                    let v95: US8 = if v92 {
                        let v93: US9 = US9::US9_AgileP3;
                        US8::US8_Valid(v93.clone())
                    } else {
                        v91.clone()
                    };
                    let v96: bool = &*v2 == "P4";
                    let v99: US8 = if v96 {
                        let v97: US9 = US9::US9_AgileP4;
                        US8::US8_Valid(v97.clone())
                    } else {
                        v95.clone()
                    };
                    let v100: bool = &*v2 == "P5";
                    let v103: US8 = if v100 {
                        let v101: US9 = US9::US9_AgileP5;
                        US8::US8_Valid(v101.clone())
                    } else {
                        v99.clone()
                    };
                    match &v103 {
                        US8::US8_Invalid => {
                            US2::US2_Invalid
                        }
                        US8::US8_Valid(v104) => {
                            let v104: US9 = v104.clone();
                            let v111: Rc<str> = match &v104 {
                                US9::US9_AgileP0 => {
                                    let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                                    v105.clone()
                                }
                                US9::US9_AgileP1 => {
                                    let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                                    v106.clone()
                                }
                                US9::US9_AgileP2 => {
                                    let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                                    v107.clone()
                                }
                                US9::US9_AgileP3 => {
                                    let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                                    v108.clone()
                                }
                                US9::US9_AgileP4 => {
                                    let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                                    v109.clone()
                                }
                                US9::US9_AgileP5 => {
                                    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                                    v110.clone()
                                }
                            };
                            US2::US2_Valid(v111.clone())
                        }
                    }
                }
                US1::US1_EditTags => {
                    quoted_field_value_3(v2.clone())
                }
                US1::US1_EditTitle => {
                    quoted_field_value_3(v2.clone())
                }
            };
            match &v174 {
                US2::US2_Invalid => {
                    let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v176.clone()
                }
                US2::US2_Valid(v175) => {
                    let v175: Rc<str> = v175.clone();
                    v175.clone()
                }
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<str> {
        edit_field_token_2(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn edit_field_rule_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: US0 = parse_edit_field_1(v0);
    match &v1 {
        US0::US0_Invalid => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be "); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title, title-append, kind, priority, difficulty, deps, tags"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be title, title-append, kind, priority, difficulty, deps, tags"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be title, title-append, kind, priority, difficulty, deps, tags, estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
            v14
        }
        US0::US0_Valid(v2) => {
            let v2: US1 = v2.clone();
            match &v2 {
                US1::US1_EditDifficulty => {
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                    v5
                }
                US1::US1_EditKind => {
                    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                    v3
                }
                US1::US1_EditPriority => {
                    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                    v4
                }
                _ => {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the value was rejected"); } LIT.with(|lit| lit.clone()) };
                    v6
                }
            }
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        edit_field_rule_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_edit_field_position(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_agile_edit_field_token(v0: &str, v1: &str, v2: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn kit_agile_edit_field_rule(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
