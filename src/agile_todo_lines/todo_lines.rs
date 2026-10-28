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
    US0_FsharpFamily,
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
fn directive_line_3(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///>"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.starts_with(&*v3);
        v4
    }
}
fn visible_end_4(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn comment_leaders_5(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_BatchFamily => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("::|REM|rem"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_CFamily => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|/*"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US0::US0_DashFamily => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US0::US0_ErlangFamily => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US0::US0_FsharpFamily => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|(*"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        US0::US0_HashFamily => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US0::US0_HaskellFamily => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|{-"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_LeanFamily => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|/-"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US0::US0_LispFamily => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US0::US0_MarkupFamily => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!--"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US0::US0_MlFamily => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(*"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US0::US0_PascalFamily => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|(*|{"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
    }
}
fn leader_before_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        let v7: i32 = if v5 {
            let mut v6: i32 = (v0.clone().len() as i32);
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
fn first_visible_offset_7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v2 <= v1;
        if v3 {
            return v1;
        } else {
            let v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = v4 == 32i32;
            if v5 {
                let mut v6: i32 = v1.wrapping_add(1i32);
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
fn comment_text_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.trim());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-->"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.ends_with(&*v2);
    let v25: Rc<str> = if v3 {
        let v4: i32 = (v1.clone().len() as i32);
        let v5: i32 = v4.wrapping_sub(3i32);
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v5 as usize).unwrap_or(""));
        v6.clone()
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*/"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v1.ends_with(&*v7);
        if v8 {
            let v9: i32 = (v1.clone().len() as i32);
            let v10: i32 = v9.wrapping_sub(2i32);
            let v11: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v10 as usize).unwrap_or(""));
            v11.clone()
        } else {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*)"); } LIT.with(|lit| lit.clone()) };
            let v13: bool = v1.ends_with(&*v12);
            if v13 {
                let v14: i32 = (v1.clone().len() as i32);
                let v15: i32 = v14.wrapping_sub(2i32);
                let v16: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v15 as usize).unwrap_or(""));
                v16.clone()
            } else {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-}"); } LIT.with(|lit| lit.clone()) };
                let v18: bool = v1.ends_with(&*v17);
                if v18 {
                    let v19: i32 = (v1.clone().len() as i32);
                    let v20: i32 = v19.wrapping_sub(2i32);
                    let v21: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v20 as usize).unwrap_or(""));
                    v21.clone()
                } else {
                    v1.clone()
                }
            }
        }
    };
    let v26: Rc<str> = std::rc::Rc::<str>::from(v25.trim());
    v26.clone()
}
fn leader_run_start_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v1 <= 0i32;
        if v3 {
            return 0i32;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: i32 = v0.as_bytes().get(v4 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v6: bool = v5 == v2;
            if v6 {
                (v0, v1, v2) = (v0.clone(), v4, v2);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn count_of_byte_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        let v5: bool = v3 <= v2;
        if v5 {
            return v4;
        } else {
            let mut v6: i32 = v0.as_bytes().get(v2 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
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
fn outside_quotes_10(mut v0: bool, mut v1: Rc<str>, mut v2: i32) -> bool {
    let v3: i32 = 34i32;
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let mut v6: i32 = count_of_byte_11(v1.clone(), v3, v4, v2, v5);
    let v7: i32 = v6.wrapping_rem(2i32);
    let v8: bool = v7 == 0i32;
    if v0 {
        let v9: i32 = 39i32;
        let v10: i32 = 0i32;
        let v11: i32 = 0i32;
        let mut v12: i32 = count_of_byte_11(v1, v9, v10, v2, v11);
        let v13: i32 = v12.wrapping_rem(2i32);
        let v14: bool = v13 == 0i32;
        let v15: bool = v8 && v14;
        v15
    } else {
        v8
    }
}
fn earlier_comment_12(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> bool {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v1.get(v3 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v3 as usize) as i32).unwrap_or(-1);
        let mut v6: bool = v5 < 0i32;
        let v8: i32 = if v6 {
            let v7: i32 = (v1.clone().len() as i32);
            v7
        } else {
            v5
        };
        let v9: Rc<str> = std::rc::Rc::<str>::from(v1.get(v3 as usize..v8 as usize).unwrap_or(""));
        let v10: i32 = v2.find(&*v9).map(|offset| offset as i32).unwrap_or(-1);
        let v11: bool = 0i32 <= v10;
        let mut v12: bool = outside_quotes_10(v0, v2.clone(), v10);
        let v13: bool = v11 && v12;
        if v13 {
            return true;
        } else {
            if v6 {
                return false;
            } else {
                let v14: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0, v1.clone(), v2.clone(), v14);
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
    let mut v6: i32 = v1.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
    let v7: bool = v6 < 0i32;
    if v7 {
        US2::US2_NoTodo
    } else {
        let v9: bool = directive_line_3(v1.clone());
        if v9 {
            US2::US2_NoTodo
        } else {
            let v11: i32 = v6.wrapping_add(4i32);
            let mut v12: i32 = v1.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v13: bool = 65i32 <= v12;
            let v14: bool = v12 <= 90i32;
            let v15: bool = v13 && v14;
            let v25: bool = if v15 {
                true
            } else {
                let v16: bool = 97i32 <= v12;
                let v17: bool = v12 <= 122i32;
                let v18: bool = v16 && v17;
                if v18 {
                    true
                } else {
                    let v19: bool = 48i32 <= v12;
                    let v20: bool = v12 <= 57i32;
                    let v21: bool = v19 && v20;
                    if v21 {
                        true
                    } else {
                        let v22: bool = v12 == 95i32;
                        v22
                    }
                }
            };
            if v25 {
                US2::US2_NoTodo
            } else {
                let v27: i32 = visible_end_4(v1.clone(), v6);
                let v28: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v27 as usize).unwrap_or(""));
                let v29: Rc<str> = comment_leaders_5(v0.clone());
                let v30: i32 = 0i32;
                let v31: Rc<str> = leader_before_6(v29.clone(), v28.clone(), v30);
                let v35: Rc<str> = match &v0 {
                    US0::US0_CFamily => {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
                        v32.clone()
                    }
                    US0::US0_PascalFamily => {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
                        v33.clone()
                    }
                    _ => {
                        v4.clone()
                    }
                };
                let v36: i32 = (v28.clone().len() as i32);
                let v37: i32 = (v31.clone().len() as i32);
                let v38: i32 = v36.wrapping_sub(v37);
                let v39: i32 = 0i32;
                let v40: i32 = first_visible_offset_7(v1.clone(), v39);
                let v41: bool = v31 == v4 ;
                if v41 {
                    let v42: bool = v35 == v4 ;
                    let v43: bool = v42 == false;
                    let v44: bool = v28.ends_with(&*v35);
                    let v45: i32 = (v35.clone().len() as i32);
                    let v46: i32 = v36.wrapping_sub(v45);
                    let v47: bool = v46 == v40;
                    let v48: bool = v44 && v47;
                    let v49: bool = v43 && v48;
                    if v49 {
                        let v50: i32 = (v1.clone().len() as i32);
                        let v51: Rc<str> = std::rc::Rc::<str>::from(v1.get(v40 as usize..v50 as usize).unwrap_or(""));
                        let v52: Rc<str> = comment_text_8(v51.clone());
                        US2::US2_TodoComment(v52.clone())
                    } else {
                        US2::US2_NoTodo
                    }
                } else {
                    let v56: i32 = v31.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let v57: i32 = leader_run_start_9(v1.clone(), v38, v56);
                    let mut v60: bool = match &v0 {
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
                    let v61: bool = outside_quotes_10(v60, v1.clone(), v57);
                    let v62: bool = v61 == false;
                    if v62 {
                        US2::US2_NoTodo
                    } else {
                        let v64: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v57 as usize).unwrap_or(""));
                        let v65: i32 = 0i32;
                        let v66: bool = earlier_comment_12(v60, v29.clone(), v64.clone(), v65);
                        if v66 {
                            US2::US2_NoTodo
                        } else {
                            let v68: i32 = (v1.clone().len() as i32);
                            let v69: Rc<str> = std::rc::Rc::<str>::from(v1.get(v57 as usize..v68 as usize).unwrap_or(""));
                            let v70: Rc<str> = comment_text_8(v69.clone());
                            US2::US2_TodoComment(v70.clone())
                        }
                    }
                }
            }
        }
    }
}
fn leader_prefix_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        let v7: i32 = if v5 {
            let mut v6: i32 = (v0.clone().len() as i32);
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
fn comment_starts_line_13(mut v0: US0, mut v1: Rc<str>) -> bool {
    let v2: i32 = 0i32;
    let v3: i32 = first_visible_offset_7(v1.clone(), v2);
    let v4: Rc<str> = comment_leaders_5(v0);
    let v5: i32 = (v1.clone().len() as i32);
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.get(v3 as usize..v5 as usize).unwrap_or(""));
    let v7: i32 = 0i32;
    leader_prefix_14(v4, v6, v7)
}
fn todo_free_16(mut v0: US0, mut v1: Rc<str>) -> bool {
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
fn block_from_15(mut v0: US0, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: i32) -> (Rc<str>, i32, i32) {
    loop {
        let mut v6: i32 = (v1.clone().len() as i32);
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
            let v15: i32 = first_visible_offset_7(v13.clone(), v14);
            let v16: bool = v15 == v2;
            let v17: i32 = (v13.clone().len() as i32);
            let v18: Rc<str> = std::rc::Rc::<str>::from(v13.get(v2 as usize..v17 as usize).unwrap_or(""));
            let v19: bool = comment_starts_line_13(v0.clone(), v13.clone());
            let v20: bool = v16 && v19;
            let v21: bool = todo_free_16(v0.clone(), v13.clone());
            let v22: bool = directive_line_3(v18.clone());
            let v23: bool = v22 == false;
            let v24: bool = v21 && v23;
            let v25: bool = v20 && v24;
            if v25 {
                let v26: i32 = v11.wrapping_add(1i32);
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v8, &*v18, &*v27, &*v27].concat());
                let v29: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2, v26, v28.clone(), v29);
                continue;
            } else {
                return (v4.clone(), v3, v5);
            }
        }
    }
}
fn id_end_18(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v2 <= v1;
        if v3 {
            return v1;
        } else {
            let v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = 65i32 <= v4;
            let v11: bool = if v5 {
                let mut v6: bool = v4 <= 90i32;
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
fn todo_id_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TO"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DO"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v4, &*v4, &*v4].concat());
    let mut v6: i32 = v1.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
    let v7: i32 = v6.wrapping_add(4i32);
    let v8: bool = v6 < 0i32;
    if v8 {
        v4.clone()
    } else {
        let v9: i32 = v1.as_bytes().get(v7 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v10: bool = v9 == 58i32;
        if v10 {
            let v11: i32 = v7.wrapping_add(1i32);
            let mut v12: i32 = first_visible_offset_7(v1.clone(), v11);
            let v13: i32 = id_end_18(v1.clone(), v12);
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
fn json_escaped_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v1, &v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v4, &v5));
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
fn finding_json_19(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = todo_id_17(v1.clone(), v4.clone());
    let mut v6: Rc<dyn Fn(Rc<str>) -> Rc<str>> = v0(v5.clone());
    let v7: Rc<str> = v6(v4.clone());
    let v8: bool = &*v5 == "";
    let v18: Rc<str> = if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let v10: Rc<str> = json_escaped_20(v5.clone());
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"id\":\""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10));
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"fields\":"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v13)), v7));
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
        v17.clone()
    };
    let v19: Rc<str> = json_escaped_20(v2.clone());
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":\""); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"line\":"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
    let v24: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v25, v26)), v18));
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"text\":\""); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
    let v31: Rc<str> = json_escaped_20(v4.clone());
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"}"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
    v34.clone()
}
fn findings_from_1(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>>, mut v1: US0, mut v2: US1, mut v3: bool, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: i32, mut v9: i32, mut v10: Rc<str>, mut v11: i32, mut v12: Rc<str>) -> (Rc<str>, i32, Rc<str>) {
    loop {
        let v13: i32 = (v5.clone().len() as i32);
        let v14: bool = v13 <= v8;
        if v14 {
            return (v10.clone(), v11, v12.clone());
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v16: i32 = v5.get(v8 as usize..).and_then(|rest| rest.find(&*v15)).map(|found| (found + v8 as usize) as i32).unwrap_or(-1);
            let v17: bool = v16 < 0i32;
            let v18: i32 = if v17 {
                v13
            } else {
                v16
            };
            let v19: Rc<str> = std::rc::Rc::<str>::from(v5.get(v8 as usize..v18 as usize).unwrap_or(""));
            let v20: i32 = v18.wrapping_add(1i32);
            let v21: US2 = todo_in_line_2(v1.clone(), v19.clone());
            match &v21 {
                US2::US2_NoTodo => {
                    let v22: i32 = v9.wrapping_add(1i32);
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = (v0.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v7, v20, v22, v10.clone(), v11, v12.clone());
                    continue;
                }
                US2::US2_TodoComment(v26) => {
                    let v26: Rc<str> = v26.clone();
                    let v27: bool = comment_starts_line_13(v1.clone(), v19.clone());
                    let (mut v34, mut v35, mut v36): (Rc<str>, i32, i32) = if v27 {
                        let v28: i32 = 0i32;
                        let v29: i32 = first_visible_offset_7(v19.clone(), v28);
                        let v30: i32 = 0i32;
                        block_from_15(v1.clone(), v5.clone(), v29, v20, v26.clone(), v30)
                    } else {
                        (v26.clone(), v20, 0i32)
                    };
                    let v37: i32 = v9.wrapping_add(1i32);
                    let v38: i32 = v37.wrapping_add(v36);
                    let v39: Rc<str> = todo_id_17(v4.clone(), v34.clone());
                    let v47: bool = match &v2 {
                        US1::US1_AllTodos => {
                            true
                        }
                        US1::US1_TaggedTodos => {
                            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v41: bool = v39 == v40 ;
                            let v42: bool = v41 == false;
                            v42
                        }
                        US1::US1_TodosOf(v43) => {
                            let v43: Rc<str> = v43.clone();
                            let v44: bool = v39 == v43 ;
                            v44
                        }
                    };
                    if v47 {
                        let v53: Rc<str> = if v7 {
                            finding_json_19(v0.clone(), v4.clone(), v6.clone(), v9, v34.clone())
                        } else {
                            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let v50: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                            let v52: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v49, &*v50, &*v51, &*v34].concat());
                            v52.clone()
                        };
                        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v55: bool = v10 == v54 ;
                        let mut v60: Rc<str> = if v55 {
                            v53.clone()
                        } else {
                            if v7 {
                                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\n"); } LIT.with(|lit| lit.clone()) };
                                let v57: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v56, &*v53, &*v54, &*v54].concat());
                                v57.clone()
                            } else {
                                let v58: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v15, &*v53, &*v54, &*v54].concat());
                                v58.clone()
                            }
                        };
                        let v61: i32 = v11.wrapping_add(1i32);
                        let v62: Rc<str> = std::rc::Rc::<str>::from(v5.get(v8 as usize..v35 as usize).unwrap_or(""));
                        let v63: i32 = (v62).lines().map(|line| line.trim_end().chars().count()).max().unwrap_or(0) as i32;
                        let v64: bool = 120i32 < v63;
                        let v65: bool = v3 && v64;
                        let v69: Rc<str> = if v65 {
                            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let v67: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
                            let v68: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v6, &*v66, &*v67, &*v15].concat());
                            v68.clone()
                        } else {
                            v12.clone()
                        };
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = (v0.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v7, v35, v38, v60.clone(), v61, v69.clone());
                        continue;
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = (v0.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v7, v35, v38, v10.clone(), v11, v12.clone());
                        continue;
                    }
                }
            }
        }
    }
}
fn linked_marker_fields_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = agile_todo_fields::kit_agile_todo_fields(&*v0, &*v1);
    v2
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<str> {
        linked_marker_fields_21(v0.clone(), v1.clone())
    })
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
        closure2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn last_offset_of_22(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v4 <= v2;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v0.as_bytes().get(v2 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
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
    let mut v6: bool = v0 == 1i32;
    let v42: US0 = if v6 {
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
                                                let v28: bool = v0 == 12i32;
                                                if v28 {
                                                    US0::US0_FsharpFamily
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
        }
    };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: bool = v1 == v43 ;
    let v51: US1 = if v44 {
        US1::US1_AllTodos
    } else {
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
        let v47: bool = v1 == v46 ;
        if v47 {
            US1::US1_TaggedTodos
        } else {
            US1::US1_TodosOf(v1.clone())
        }
    };
    let v52: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>> = closure1();
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spi|spir"); } LIT.with(|lit| lit.clone()) };
    let v54: i32 = 47i32;
    let v55: i32 = 0i32;
    let v56: i32 = -1i32;
    let v57: i32 = last_offset_of_22(v4.clone(), v54, v55, v56);
    let v58: i32 = v57.wrapping_add(1i32);
    let v59: i32 = (v4.clone().len() as i32);
    let mut v60: Rc<str> = std::rc::Rc::<str>::from(v4.get(v58 as usize..v59 as usize).unwrap_or(""));
    let v61: Rc<str> = std::rc::Rc::<str>::from(v60.to_ascii_lowercase());
    let v62: i32 = 46i32;
    let v63: i32 = 0i32;
    let v64: i32 = -1i32;
    let v65: i32 = last_offset_of_22(v61.clone(), v62, v63, v64);
    let v66: bool = v65 < 0i32;
    let v70: Rc<str> = if v66 {
        v43.clone()
    } else {
        let v67: i32 = v65.wrapping_add(1i32);
        let v68: i32 = (v61.clone().len() as i32);
        let v69: Rc<str> = std::rc::Rc::<str>::from(v61.get(v67 as usize..v68 as usize).unwrap_or(""));
        v69.clone()
    };
    let v71: bool = v53.split("|").any(|item| item == &*v70);
    let v72: i32 = 0i32;
    let v73: i32 = 1i32;
    let v74: i32 = 0i32;
    findings_from_1(v52.clone(), v42.clone(), v51.clone(), v71, v2.clone(), v3.clone(), v4.clone(), v5, v72, v73, v43.clone(), v74, v43.clone())
}
fn closure0() -> Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> (Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> (Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool| -> (Rc<str>, i32, Rc<str>) {
        todo_findings_0(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        todo_id_17(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_todo_findings(v0: i32, v1: &str, v2: &str, v3: &str, v4: &str, v5: bool) -> (Rc<str>, i32, Rc<str>) {
    closure0()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), v5)
}
pub fn kit_agile_todo_id(v0: &str, v1: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
