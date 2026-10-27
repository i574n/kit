#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_CFamily,
    US0_HashFamily,
    US0_DashFamily,
    US0_HaskellFamily,
    US0_LeanFamily,
    US0_LispFamily,
    US0_ErlangFamily,
    US0_MarkupFamily,
    US0_MlFamily,
    US0_PascalFamily,
    US0_BatchFamily,
}
#[derive(Clone)]
enum US1 {
    US1_AllTodos,
    US1_TaggedTodos,
    US1_TodosOf(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_NoTodo,
    US2_TodoComment(Rc<str>),
}
fn visible_end_3(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return 0i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: i32 = v0.as_bytes().get(v3 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = v4 == 32i32;
            if v5 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                let v7: i32 = v0.as_bytes().get(v3 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v8: bool = v7 == 9i32;
                if v8 {
                    (v0, v1) = (v0.clone(), v3);
                    continue;
                } else {
                    let v10: i32 = v0.as_bytes().get(v3 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let v11: bool = v10 == 13i32;
                    if v11 {
                        (v0, v1) = (v0.clone(), v3);
                        continue;
                    } else {
                        return v1;
                    }
                }
            }
        }
    }
}
fn leader_before_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        let v7: i32 = if v5 {
            let v6: i32 = (v0.clone().len() as i32);
            v6
        } else {
            v4
        };
        let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v7 as usize).unwrap_or(""));
        let v9: bool = v1.ends_with(&*v8);
        if v9 {
            return v8.clone();
        } else {
            if v5 {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v10.clone();
            } else {
                let v11: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v11);
                continue;
            }
        }
    }
}
fn first_visible_offset_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v2 <= v1;
        if v3 {
            return v1;
        } else {
            let v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = v4 == 32i32;
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                let v8: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v9: bool = v8 == 9i32;
                if v9 {
                    let v10: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v10);
                    continue;
                } else {
                    return v1;
                }
            }
        }
    }
}
fn leader_run_start_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v1 <= 0i32;
        if v3 {
            return 0i32;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: i32 = v0.as_bytes().get(v4 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v6: bool = v5 == v2;
            if v6 {
                (v0, v1, v2) = (v0.clone(), v4, v2);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn count_of_byte_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        let v5: bool = v3 <= v2;
        if v5 {
            return v4;
        } else {
            let v6: i32 = v0.as_bytes().get(v2 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v7: bool = v6 == v1;
            if v7 {
                let v8: i32 = v2.wrapping_add(1i32);
                let v9: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v1, v8, v3, v9);
                continue;
            } else {
                let v11: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v1, v11, v3, v4);
                continue;
            }
        }
    }
}
fn earlier_comment_8(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> bool {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v1.get(v3 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v3 as usize) as i32).unwrap_or(-1);
        let v6: bool = v5 < 0i32;
        let v8: i32 = if v6 {
            let v7: i32 = (v1.clone().len() as i32);
            v7
        } else {
            v5
        };
        let v9: Rc<str> = std::rc::Rc::<str>::from(v1.get(v3 as usize..v8 as usize).unwrap_or(""));
        let v10: i32 = v2.find(&*v9).map(|offset| offset as i32).unwrap_or(-1);
        let v11: bool = 0i32 <= v10;
        let mut v12: i32 = 34i32;
        let v13: i32 = 0i32;
        let v14: i32 = 0i32;
        let v15: i32 = count_of_byte_7(v2.clone(), v12, v13, v10, v14);
        let v16: i32 = v15.wrapping_rem(2i32);
        let v17: bool = v16 == 0i32;
        let v25: bool = if v0 {
            let v18: i32 = 39i32;
            let v19: i32 = 0i32;
            let v20: i32 = 0i32;
            let v21: i32 = count_of_byte_7(v2.clone(), v18, v19, v10, v20);
            let v22: i32 = v21.wrapping_rem(2i32);
            let v23: bool = v22 == 0i32;
            let v24: bool = v17 && v23;
            v24
        } else {
            v17
        };
        let v26: bool = v11 && v25;
        if v26 {
            return true;
        } else {
            if v6 {
                return false;
            } else {
                let v27: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0, v1.clone(), v2.clone(), v27);
                continue;
            }
        }
    }
}
fn todo_in_line_2(mut v0: US0, mut v1: Rc<str>) -> US2 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TO"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DO"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v4, &*v4, &*v4].concat());
    let v6: i32 = v1.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
    let v7: bool = v6 < 0i32;
    if v7 {
        US2::US2_NoTodo
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///>"); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v1.starts_with(&*v9);
        let v13: bool = if v10 {
            true
        } else {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
            let mut v12: bool = v1.starts_with(&*v11);
            v12
        };
        if v13 {
            US2::US2_NoTodo
        } else {
            let v15: i32 = v6.wrapping_add(4i32);
            let v16: i32 = v1.as_bytes().get(v15 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v17: bool = 65i32 <= v16;
            let v18: bool = v16 <= 90i32;
            let v19: bool = v17 && v18;
            let v29: bool = if v19 {
                true
            } else {
                let v20: bool = 97i32 <= v16;
                let v21: bool = v16 <= 122i32;
                let v22: bool = v20 && v21;
                if v22 {
                    true
                } else {
                    let v23: bool = 48i32 <= v16;
                    let v24: bool = v16 <= 57i32;
                    let v25: bool = v23 && v24;
                    if v25 {
                        true
                    } else {
                        let v26: bool = v16 == 95i32;
                        v26
                    }
                }
            };
            if v29 {
                US2::US2_NoTodo
            } else {
                let v31: i32 = visible_end_3(v1.clone(), v6);
                let v32: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v31 as usize).unwrap_or(""));
                let v54: Rc<str> = match &v0 {
                    US0::US0_BatchFamily => {
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("::|REM|rem"); } LIT.with(|lit| lit.clone()) };
                        v43.clone()
                    }
                    US0::US0_CFamily => {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|/*"); } LIT.with(|lit| lit.clone()) };
                        v33.clone()
                    }
                    US0::US0_DashFamily => {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                        v35.clone()
                    }
                    US0::US0_ErlangFamily => {
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
                        v39.clone()
                    }
                    US0::US0_HashFamily => {
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
                        v34.clone()
                    }
                    US0::US0_HaskellFamily => {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|{-"); } LIT.with(|lit| lit.clone()) };
                        v36.clone()
                    }
                    US0::US0_LeanFamily => {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|/-"); } LIT.with(|lit| lit.clone()) };
                        v37.clone()
                    }
                    US0::US0_LispFamily => {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                        v38.clone()
                    }
                    US0::US0_MarkupFamily => {
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!--"); } LIT.with(|lit| lit.clone()) };
                        v40.clone()
                    }
                    US0::US0_MlFamily => {
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(*"); } LIT.with(|lit| lit.clone()) };
                        v41.clone()
                    }
                    US0::US0_PascalFamily => {
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|(*|{"); } LIT.with(|lit| lit.clone()) };
                        v42.clone()
                    }
                };
                let v55: i32 = 0i32;
                let mut v56: Rc<str> = leader_before_4(v54.clone(), v32.clone(), v55);
                let v60: Rc<str> = match &v0 {
                    US0::US0_CFamily => {
                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
                        v57.clone()
                    }
                    US0::US0_PascalFamily => {
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
                        v58.clone()
                    }
                    _ => {
                        v4.clone()
                    }
                };
                let v61: i32 = (v32.clone().len() as i32);
                let v62: i32 = (v56.clone().len() as i32);
                let v63: i32 = v61.wrapping_sub(v62);
                let v64: i32 = 0i32;
                let v65: i32 = first_visible_offset_5(v1.clone(), v64);
                let v66: bool = v56 == v4 ;
                if v66 {
                    let v67: bool = v60 == v4 ;
                    let v68: bool = v67 == false;
                    let v69: bool = v32.ends_with(&*v60);
                    let v70: i32 = (v60.clone().len() as i32);
                    let v71: i32 = v61.wrapping_sub(v70);
                    let v72: bool = v71 == v65;
                    let v73: bool = v69 && v72;
                    let v74: bool = v68 && v73;
                    if v74 {
                        let v75: i32 = (v1.clone().len() as i32);
                        let v76: Rc<str> = std::rc::Rc::<str>::from(v1.get(v65 as usize..v75 as usize).unwrap_or(""));
                        let v77: Rc<str> = std::rc::Rc::<str>::from(v76.trim());
                        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-->"); } LIT.with(|lit| lit.clone()) };
                        let v79: bool = v77.ends_with(&*v78);
                        let v101: Rc<str> = if v79 {
                            let v80: i32 = (v77.clone().len() as i32);
                            let v81: i32 = v80.wrapping_sub(3i32);
                            let v82: Rc<str> = std::rc::Rc::<str>::from(v77.get(0i32 as usize..v81 as usize).unwrap_or(""));
                            v82.clone()
                        } else {
                            let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*/"); } LIT.with(|lit| lit.clone()) };
                            let v84: bool = v77.ends_with(&*v83);
                            if v84 {
                                let v85: i32 = (v77.clone().len() as i32);
                                let v86: i32 = v85.wrapping_sub(2i32);
                                let v87: Rc<str> = std::rc::Rc::<str>::from(v77.get(0i32 as usize..v86 as usize).unwrap_or(""));
                                v87.clone()
                            } else {
                                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*)"); } LIT.with(|lit| lit.clone()) };
                                let v89: bool = v77.ends_with(&*v88);
                                if v89 {
                                    let v90: i32 = (v77.clone().len() as i32);
                                    let v91: i32 = v90.wrapping_sub(2i32);
                                    let v92: Rc<str> = std::rc::Rc::<str>::from(v77.get(0i32 as usize..v91 as usize).unwrap_or(""));
                                    v92.clone()
                                } else {
                                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-}"); } LIT.with(|lit| lit.clone()) };
                                    let v94: bool = v77.ends_with(&*v93);
                                    if v94 {
                                        let v95: i32 = (v77.clone().len() as i32);
                                        let v96: i32 = v95.wrapping_sub(2i32);
                                        let v97: Rc<str> = std::rc::Rc::<str>::from(v77.get(0i32 as usize..v96 as usize).unwrap_or(""));
                                        v97.clone()
                                    } else {
                                        v77.clone()
                                    }
                                }
                            }
                        };
                        let v102: Rc<str> = std::rc::Rc::<str>::from(v101.trim());
                        US2::US2_TodoComment(v102.clone())
                    } else {
                        US2::US2_NoTodo
                    }
                } else {
                    let v106: i32 = v56.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let v107: i32 = leader_run_start_6(v1.clone(), v63, v106);
                    let v110: bool = match &v0 {
                        US0::US0_DashFamily => {
                            true
                        }
                        US0::US0_ErlangFamily => {
                            true
                        }
                        US0::US0_HashFamily => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v111: i32 = 34i32;
                    let v112: i32 = 0i32;
                    let v113: i32 = 0i32;
                    let v114: i32 = count_of_byte_7(v1.clone(), v111, v112, v107, v113);
                    let v115: i32 = v114.wrapping_rem(2i32);
                    let v116: bool = v115 == 0i32;
                    let v124: bool = if v110 {
                        let v117: i32 = 39i32;
                        let v118: i32 = 0i32;
                        let v119: i32 = 0i32;
                        let v120: i32 = count_of_byte_7(v1.clone(), v117, v118, v107, v119);
                        let v121: i32 = v120.wrapping_rem(2i32);
                        let v122: bool = v121 == 0i32;
                        let v123: bool = v116 && v122;
                        v123
                    } else {
                        v116
                    };
                    let v125: bool = v124 == false;
                    if v125 {
                        US2::US2_NoTodo
                    } else {
                        let v127: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v107 as usize).unwrap_or(""));
                        let v128: i32 = 0i32;
                        let v129: bool = earlier_comment_8(v110, v54.clone(), v127.clone(), v128);
                        if v129 {
                            US2::US2_NoTodo
                        } else {
                            let v131: i32 = (v1.clone().len() as i32);
                            let v132: Rc<str> = std::rc::Rc::<str>::from(v1.get(v107 as usize..v131 as usize).unwrap_or(""));
                            let v133: Rc<str> = std::rc::Rc::<str>::from(v132.trim());
                            let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-->"); } LIT.with(|lit| lit.clone()) };
                            let v135: bool = v133.ends_with(&*v134);
                            let v157: Rc<str> = if v135 {
                                let v136: i32 = (v133.clone().len() as i32);
                                let v137: i32 = v136.wrapping_sub(3i32);
                                let v138: Rc<str> = std::rc::Rc::<str>::from(v133.get(0i32 as usize..v137 as usize).unwrap_or(""));
                                v138.clone()
                            } else {
                                let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*/"); } LIT.with(|lit| lit.clone()) };
                                let v140: bool = v133.ends_with(&*v139);
                                if v140 {
                                    let v141: i32 = (v133.clone().len() as i32);
                                    let v142: i32 = v141.wrapping_sub(2i32);
                                    let v143: Rc<str> = std::rc::Rc::<str>::from(v133.get(0i32 as usize..v142 as usize).unwrap_or(""));
                                    v143.clone()
                                } else {
                                    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*)"); } LIT.with(|lit| lit.clone()) };
                                    let v145: bool = v133.ends_with(&*v144);
                                    if v145 {
                                        let v146: i32 = (v133.clone().len() as i32);
                                        let v147: i32 = v146.wrapping_sub(2i32);
                                        let v148: Rc<str> = std::rc::Rc::<str>::from(v133.get(0i32 as usize..v147 as usize).unwrap_or(""));
                                        v148.clone()
                                    } else {
                                        let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-}"); } LIT.with(|lit| lit.clone()) };
                                        let v150: bool = v133.ends_with(&*v149);
                                        if v150 {
                                            let v151: i32 = (v133.clone().len() as i32);
                                            let v152: i32 = v151.wrapping_sub(2i32);
                                            let v153: Rc<str> = std::rc::Rc::<str>::from(v133.get(0i32 as usize..v152 as usize).unwrap_or(""));
                                            v153.clone()
                                        } else {
                                            v133.clone()
                                        }
                                    }
                                }
                            };
                            let v158: Rc<str> = std::rc::Rc::<str>::from(v157.trim());
                            US2::US2_TodoComment(v158.clone())
                        }
                    }
                }
            }
        }
    }
}
fn leader_prefix_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        let v7: i32 = if v5 {
            let v6: i32 = (v0.clone().len() as i32);
            v6
        } else {
            v4
        };
        let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v7 as usize).unwrap_or(""));
        let v9: bool = v1.starts_with(&*v8);
        if v9 {
            return true;
        } else {
            if v5 {
                return false;
            } else {
                let v10: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v10);
                continue;
            }
        }
    }
}
fn comment_starts_line_9(mut v0: US0, mut v1: Rc<str>) -> bool {
    let v2: i32 = 0i32;
    let v3: i32 = first_visible_offset_5(v1.clone(), v2);
    let v25: Rc<str> = match &v0 {
        US0::US0_BatchFamily => {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("::|REM|rem"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US0::US0_CFamily => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|/*"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_DashFamily => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US0::US0_ErlangFamily => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_HashFamily => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US0::US0_HaskellFamily => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|{-"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US0::US0_LeanFamily => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|/-"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US0::US0_LispFamily => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US0::US0_MarkupFamily => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!--"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_MlFamily => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(*"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        US0::US0_PascalFamily => {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|(*|{"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
    };
    let v26: i32 = (v1.clone().len() as i32);
    let v27: Rc<str> = std::rc::Rc::<str>::from(v1.get(v3 as usize..v26 as usize).unwrap_or(""));
    let v28: i32 = 0i32;
    leader_prefix_10(v25.clone(), v27.clone(), v28)
}
fn todo_free_12(mut v0: US0, mut v1: Rc<str>) -> bool {
    let v2: US2 = todo_in_line_2(v0, v1);
    match &v2 {
        US2::US2_NoTodo => {
            true
        }
        US2::US2_TodoComment(v3) => {
            let v3: Rc<str> = v3.clone();
            false
        }
    }
}
fn block_from_11(mut v0: US0, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: i32) -> (Rc<str>, i32, i32) {
    loop {
        let v6: i32 = (v1.clone().len() as i32);
        let v7: bool = v6 <= v3;
        if v7 {
            return (v4.clone(), v3, v5);
        } else {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v9: i32 = v1.get(v3 as usize..).and_then(|rest| rest.find(&*v8)).map(|found| (found + v3 as usize) as i32).unwrap_or(-1);
            let v10: bool = v9 < 0i32;
            let v11: i32 = if v10 {
                v6
            } else {
                v9
            };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from(v1.get(v3 as usize..v11 as usize).unwrap_or(""));
            let v13: Rc<str> = std::rc::Rc::<str>::from(v12.trim_end());
            let v14: i32 = 0i32;
            let v15: i32 = first_visible_offset_5(v13.clone(), v14);
            let v16: bool = v15 == v2;
            let v17: i32 = (v13.clone().len() as i32);
            let v18: Rc<str> = std::rc::Rc::<str>::from(v13.get(v2 as usize..v17 as usize).unwrap_or(""));
            let v19: bool = comment_starts_line_9(v0.clone(), v13.clone());
            let v20: bool = v16 && v19;
            let v21: bool = todo_free_12(v0.clone(), v13.clone());
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///>"); } LIT.with(|lit| lit.clone()) };
            let v23: bool = v18.starts_with(&*v22);
            let v26: bool = if v23 {
                true
            } else {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
                let v25: bool = v18.starts_with(&*v24);
                v25
            };
            let v27: bool = v26 == false;
            let v28: bool = v21 && v27;
            let v29: bool = v20 && v28;
            if v29 {
                let v30: i32 = v11.wrapping_add(1i32);
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v8, &*v18, &*v31, &*v31].concat());
                let v33: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2, v30, v32.clone(), v33);
                continue;
            } else {
                return (v4.clone(), v3, v5);
            }
        }
    }
}
fn id_end_14(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v2 <= v1;
        if v3 {
            return v1;
        } else {
            let v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = 65i32 <= v4;
            let v11: bool = if v5 {
                let v6: bool = v4 <= 90i32;
                v6
            } else {
                let v7: bool = 48i32 <= v4;
                if v7 {
                    let v8: bool = v4 <= 57i32;
                    v8
                } else {
                    let v9: bool = v4 == 45i32;
                    v9
                }
            };
            if v11 {
                let mut v12: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v12);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn todo_id_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TO"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DO"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v4, &*v4, &*v4].concat());
    let v6: i32 = v1.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
    let v7: i32 = v6.wrapping_add(4i32);
    let v8: bool = v6 < 0i32;
    if v8 {
        v4.clone()
    } else {
        let v9: i32 = v1.as_bytes().get(v7 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v10: bool = v9 == 58i32;
        if v10 {
            let v11: i32 = v7.wrapping_add(1i32);
            let mut v12: i32 = first_visible_offset_5(v1.clone(), v11);
            let v13: i32 = id_end_14(v1.clone(), v12);
            let v14: Rc<str> = std::rc::Rc::<str>::from(v1.get(v12 as usize..v13 as usize).unwrap_or(""));
            let v15: i32 = (v1.clone().len() as i32);
            let v16: bool = v15 <= v13;
            let v19: bool = if v16 {
                true
            } else {
                let v17: i32 = v1.as_bytes().get(v13 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v18: bool = v17 == 32i32;
                v18
            };
            let v20: bool = v13 <= v12;
            let v26: bool = if v20 {
                false
            } else {
                let v21: i32 = v1.as_bytes().get(v12 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v22: bool = 65i32 <= v21;
                let v23: i32 = v1.as_bytes().get(v12 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v24: bool = v23 <= 90i32;
                let v25: bool = v22 && v24;
                v25
            };
            let v27: bool = v19 && v26;
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v29: i32 = v14.find(&*v28).map(|offset| offset as i32).unwrap_or(-1);
            let v30: bool = 0i32 <= v29;
            let v35: bool = if v30 {
                true
            } else {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v14, &*v32, &*v4, &*v4].concat());
                let v34: bool = v0.contains(&*v33);
                v34
            };
            let v36: bool = v27 && v35;
            if v36 {
                v14.clone()
            } else {
                v4.clone()
            }
        } else {
            v4.clone()
        }
    }
}
fn json_escaped_16(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v1, &v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v4, &v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = std::rc::Rc::<str>::from(v6.replace(&*v7, &v8));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(v9.replace(&*v10, &v11));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = std::rc::Rc::<str>::from(v12.replace(&*v13, &v14));
    v15.clone()
}
fn finding_json_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = todo_id_13(v0.clone(), v3.clone());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: bool = v4 == v5 ;
    let v11: Rc<str> = if v6 {
        v5.clone()
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"id\":\""); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = json_escaped_16(v4.clone());
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v8, &*v9, &*v5, &*v5].concat());
        v10.clone()
    };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":\""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = json_escaped_16(v1.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"line\":"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v14, &*v15, &*v16].concat());
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"text\":\""); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = json_escaped_16(v3.clone());
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"}"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v11, &*v18, &*v19, &*v20].concat());
    v21.clone()
}
fn findings_from_1(mut v0: US0, mut v1: US1, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool, mut v7: i32, mut v8: i32, mut v9: Rc<str>, mut v10: i32, mut v11: Rc<str>) -> (Rc<str>, i32, Rc<str>) {
    loop {
        let mut v12: i32 = (v4.clone().len() as i32);
        let v13: bool = v12 <= v7;
        if v13 {
            return (v9.clone(), v10, v11.clone());
        } else {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v15: i32 = v4.get(v7 as usize..).and_then(|rest| rest.find(&*v14)).map(|found| (found + v7 as usize) as i32).unwrap_or(-1);
            let v16: bool = v15 < 0i32;
            let v17: i32 = if v16 {
                v12
            } else {
                v15
            };
            let v18: Rc<str> = std::rc::Rc::<str>::from(v4.get(v7 as usize..v17 as usize).unwrap_or(""));
            let v19: i32 = v17.wrapping_add(1i32);
            let v20: US2 = todo_in_line_2(v0.clone(), v18.clone());
            match &v20 {
                US2::US2_NoTodo => {
                    let v21: i32 = v8.wrapping_add(1i32);
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5.clone(), v6, v19, v21, v9.clone(), v10, v11.clone());
                    continue;
                }
                US2::US2_TodoComment(v25) => {
                    let v25: Rc<str> = v25.clone();
                    let v26: bool = comment_starts_line_9(v0.clone(), v18.clone());
                    let (mut v33, mut v34, mut v35): (Rc<str>, i32, i32) = if v26 {
                        let v27: i32 = 0i32;
                        let v28: i32 = first_visible_offset_5(v18.clone(), v27);
                        let v29: i32 = 0i32;
                        block_from_11(v0.clone(), v4.clone(), v28, v19, v25.clone(), v29)
                    } else {
                        (v25.clone(), v19, 0i32)
                    };
                    let v36: i32 = v8.wrapping_add(1i32);
                    let v37: i32 = v36.wrapping_add(v35);
                    let v38: Rc<str> = todo_id_13(v3.clone(), v33.clone());
                    let v46: bool = match &v1 {
                        US1::US1_AllTodos => {
                            true
                        }
                        US1::US1_TaggedTodos => {
                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v40: bool = v38 == v39 ;
                            let v41: bool = v40 == false;
                            v41
                        }
                        US1::US1_TodosOf(v42) => {
                            let v42: Rc<str> = v42.clone();
                            let v43: bool = v38 == v42 ;
                            v43
                        }
                    };
                    if v46 {
                        let v52: Rc<str> = if v6 {
                            finding_json_15(v3.clone(), v5.clone(), v8, v33.clone())
                        } else {
                            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let v49: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                            let v51: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v48, &*v49, &*v50, &*v33].concat());
                            v51.clone()
                        };
                        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v54: bool = v9 == v53 ;
                        let v59: Rc<str> = if v54 {
                            v52.clone()
                        } else {
                            if v6 {
                                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v55, &*v52, &*v53, &*v53].concat());
                                v56.clone()
                            } else {
                                let v57: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v14, &*v52, &*v53, &*v53].concat());
                                v57.clone()
                            }
                        };
                        let v60: i32 = v10.wrapping_add(1i32);
                        let v61: Rc<str> = std::rc::Rc::<str>::from(v4.get(v7 as usize..v34 as usize).unwrap_or(""));
                        let v62: i32 = (v61).lines().map(|line| line.trim_end().chars().count()).max().unwrap_or(0) as i32;
                        let v63: bool = 120i32 < v62;
                        let v64: bool = v2 && v63;
                        let v68: Rc<str> = if v64 {
                            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let v66: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
                            let v67: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v5, &*v65, &*v66, &*v14].concat());
                            v67.clone()
                        } else {
                            v11.clone()
                        };
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5.clone(), v6, v34, v37, v59.clone(), v60, v68.clone());
                        continue;
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5.clone(), v6, v34, v37, v9.clone(), v10, v11.clone());
                        continue;
                    }
                }
            }
        }
    }
}
fn last_offset_of_17(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v4 <= v2;
        if v5 {
            return v3;
        } else {
            let v6: i32 = v0.as_bytes().get(v2 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v7: bool = v6 == v1;
            if v7 {
                let v8: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1, v8, v2);
                continue;
            } else {
                let v10: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1, v10, v3);
                continue;
            }
        }
    }
}
fn todo_findings_0(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool) -> (Rc<str>, i32, Rc<str>) {
    let v6: bool = v0 == 1i32;
    let v39: US0 = if v6 {
        US0::US0_CFamily
    } else {
        let v8: bool = v0 == 2i32;
        if v8 {
            US0::US0_HashFamily
        } else {
            let v10: bool = v0 == 3i32;
            if v10 {
                US0::US0_DashFamily
            } else {
                let mut v12: bool = v0 == 4i32;
                if v12 {
                    US0::US0_HaskellFamily
                } else {
                    let v14: bool = v0 == 5i32;
                    if v14 {
                        US0::US0_LeanFamily
                    } else {
                        let v16: bool = v0 == 6i32;
                        if v16 {
                            US0::US0_LispFamily
                        } else {
                            let v18: bool = v0 == 7i32;
                            if v18 {
                                US0::US0_ErlangFamily
                            } else {
                                let v20: bool = v0 == 8i32;
                                if v20 {
                                    US0::US0_MarkupFamily
                                } else {
                                    let v22: bool = v0 == 9i32;
                                    if v22 {
                                        US0::US0_MlFamily
                                    } else {
                                        let v24: bool = v0 == 10i32;
                                        if v24 {
                                            US0::US0_PascalFamily
                                        } else {
                                            let v26: bool = v0 == 11i32;
                                            if v26 {
                                                US0::US0_BatchFamily
                                            } else {
                                                US0::US0_CFamily
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
    };
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v41: bool = v1 == v40 ;
    let v48: US1 = if v41 {
        US1::US1_AllTodos
    } else {
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
        let v44: bool = v1 == v43 ;
        if v44 {
            US1::US1_TaggedTodos
        } else {
            US1::US1_TodosOf(v1.clone())
        }
    };
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spi|spir"); } LIT.with(|lit| lit.clone()) };
    let v50: i32 = 47i32;
    let v51: i32 = 0i32;
    let v52: i32 = -1i32;
    let v53: i32 = last_offset_of_17(v4.clone(), v50, v51, v52);
    let v54: i32 = v53.wrapping_add(1i32);
    let v55: i32 = (v4.clone().len() as i32);
    let mut v56: Rc<str> = std::rc::Rc::<str>::from(v4.get(v54 as usize..v55 as usize).unwrap_or(""));
    let v57: Rc<str> = std::rc::Rc::<str>::from(v56.to_ascii_lowercase());
    let v58: i32 = 46i32;
    let v59: i32 = 0i32;
    let v60: i32 = -1i32;
    let v61: i32 = last_offset_of_17(v57.clone(), v58, v59, v60);
    let v62: bool = v61 < 0i32;
    let v66: Rc<str> = if v62 {
        v40.clone()
    } else {
        let v63: i32 = v61.wrapping_add(1i32);
        let v64: i32 = (v57.clone().len() as i32);
        let v65: Rc<str> = std::rc::Rc::<str>::from(v57.get(v63 as usize..v64 as usize).unwrap_or(""));
        v65.clone()
    };
    let v67: bool = v49.split("|").any(|item| item == &*v66);
    let v68: i32 = 0i32;
    let v69: i32 = 1i32;
    let v70: i32 = 0i32;
    findings_from_1(v39.clone(), v48.clone(), v67, v2.clone(), v3.clone(), v4.clone(), v5, v68, v69, v40.clone(), v70, v40.clone())
}
fn closure0() -> Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> (Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> (Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool| -> (Rc<str>, i32, Rc<str>) {
        todo_findings_0(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        todo_id_13(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_todo_findings(v0: i32, v1: &str, v2: &str, v3: &str, v4: &str, v5: bool) -> (Rc<str>, i32, Rc<str>) {
    closure0()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), v5)
}
pub fn kit_agile_todo_id(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
