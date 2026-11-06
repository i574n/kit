#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;

pub fn spiral_json_quote(s: &str) -> std::rc::Rc<str> {
    let (q, b) = (char::from(34u8), char::from(92u8));
    let mut out = String::from(q);
    for c in s.chars() {
        match c as u32 {
            34 | 92 => { out.push(b); out.push(c); }
            n @ (8 | 9 | 10 | 12 | 13) => { out.push(b); out.push(char::from(b"btn_fr"[(n - 8) as usize])); }
            n if n < 32 => out.push_str(&format!("{b}u{n:04x}")),
            _ => out.push(c),
        }
    }
    out.push(q);
    std::rc::Rc::from(out.as_str())
}

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
fn concat5_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn directive_line_4(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///>"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    if v2 {
        true
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.starts_with(&*v3);
        v4
    }
}
fn visible_end_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return 0i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let v4: i32 = v0.as_bytes().get(v3 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = v4 == 32i32;
            if v5 {
                v1 = v3;
                continue;
            } else {
                let v7: i32 = v0.as_bytes().get(v3 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v8: bool = v7 == 9i32;
                if v8 {
                    v1 = v3;
                    continue;
                } else {
                    let v10: i32 = v0.as_bytes().get(v3 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let v11: bool = v10 == 13i32;
                    if v11 {
                        v1 = v3;
                        continue;
                    } else {
                        return v1;
                    }
                }
            }
        }
    }
}
fn comment_leaders_6(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_BatchFamily => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("::|REM|rem"); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_CFamily => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|/*"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_DashFamily => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_ErlangFamily => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US0::US0_FsharpFamily => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|(*"); } LIT.with(|lit| lit.clone()) };
            v12
        }
        US0::US0_HashFamily => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_HaskellFamily => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|{-"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_LeanFamily => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--|/-"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US0::US0_LispFamily => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US0::US0_MarkupFamily => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!--"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US0::US0_MlFamily => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(*"); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US0::US0_PascalFamily => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//|(*|{"); } LIT.with(|lit| lit.clone()) };
            v10
        }
    }
}
fn leader_before_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
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
            return v8;
        } else {
            if v5 {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v10;
            } else {
                let v11: i32 = v4.wrapping_add(1i32);
                v2 = v11;
                continue;
            }
        }
    }
}
fn first_visible_offset_8(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v2 <= v1;
        if v3 {
            return v1;
        } else {
            let v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = v4 == 32i32;
            if v5 {
                let mut v6: i32 = v1.wrapping_add(1i32);
                v1 = v6;
                continue;
            } else {
                let v8: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v9: bool = v8 == 9i32;
                if v9 {
                    let v10: i32 = v1.wrapping_add(1i32);
                    v1 = v10;
                    continue;
                } else {
                    return v1;
                }
            }
        }
    }
}
fn comment_text_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.trim());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-->"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.ends_with(&*v2);
    let v25: Rc<str> = if v3 {
        let v4: i32 = (v1.clone().len() as i32);
        let v5: i32 = v4.wrapping_sub(3i32);
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v5 as usize).unwrap_or(""));
        v6
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*/"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v1.ends_with(&*v7);
        if v8 {
            let v9: i32 = (v1.clone().len() as i32);
            let v10: i32 = v9.wrapping_sub(2i32);
            let v11: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v10 as usize).unwrap_or(""));
            v11
        } else {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*)"); } LIT.with(|lit| lit.clone()) };
            let v13: bool = v1.ends_with(&*v12);
            if v13 {
                let v14: i32 = (v1.clone().len() as i32);
                let v15: i32 = v14.wrapping_sub(2i32);
                let v16: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v15 as usize).unwrap_or(""));
                v16
            } else {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-}"); } LIT.with(|lit| lit.clone()) };
                let v18: bool = v1.ends_with(&*v17);
                if v18 {
                    let v19: i32 = (v1.clone().len() as i32);
                    let v20: i32 = v19.wrapping_sub(2i32);
                    let v21: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v20 as usize).unwrap_or(""));
                    v21
                } else {
                    v1
                }
            }
        }
    };
    let v26: Rc<str> = std::rc::Rc::<str>::from(v25.trim());
    v26
}
fn leader_run_start_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v1 <= 0i32;
        if v3 {
            return 0i32;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: i32 = v0.as_bytes().get(v4 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v6: bool = v5 == v2;
            if v6 {
                v1 = v4;
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn count_of_byte_12(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
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
                (v2, v4) = (v8, v9);
                continue;
            } else {
                let v11: i32 = v2.wrapping_add(1i32);
                v2 = v11;
                continue;
            }
        }
    }
}
fn outside_quotes_11(mut v0: bool, mut v1: Rc<str>, mut v2: i32) -> bool {
    let mut v3: i32 = 34i32;
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let mut v6: i32 = count_of_byte_12(v1.clone(), v3, v4, v2, v5);
    let v7: i32 = v6.wrapping_rem(2i32);
    let v8: bool = v7 == 0i32;
    if v0 {
        let v9: i32 = 39i32;
        let v10: i32 = 0i32;
        let v11: i32 = 0i32;
        let mut v12: i32 = count_of_byte_12(v1, v9, v10, v2, v11);
        let v13: i32 = v12.wrapping_rem(2i32);
        let v14: bool = v13 == 0i32;
        let v15: bool = v8 && v14;
        v15
    } else {
        v8
    }
}
fn earlier_comment_13(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> bool {
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
        let mut v12: bool = outside_quotes_11(v0, v2.clone(), v10);
        let v13: bool = v11 && v12;
        if v13 {
            return true;
        } else {
            if v6 {
                return false;
            } else {
                let v14: i32 = v5.wrapping_add(1i32);
                v3 = v14;
                continue;
            }
        }
    }
}
fn todo_in_line_2(mut v0: US0, mut v1: Rc<str>) -> US2 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TO"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DO"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_3(v2, v3, v4.clone(), v4.clone(), v4.clone());
    let mut v6: i32 = v1.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
    let v7: bool = v6 < 0i32;
    if v7 {
        US2::US2_NoTodo
    } else {
        let v9: bool = directive_line_4(v1.clone());
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
                let v27: i32 = visible_end_5(v1.clone(), v6);
                let v28: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v27 as usize).unwrap_or(""));
                let v29: Rc<str> = comment_leaders_6(v0.clone());
                let v30: i32 = 0i32;
                let v31: Rc<str> = leader_before_7(v29.clone(), v28.clone(), v30);
                let v35: Rc<str> = match &v0 {
                    US0::US0_CFamily => {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
                        v32
                    }
                    US0::US0_PascalFamily => {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
                        v33
                    }
                    _ => {
                        v4
                    }
                };
                let v36: i32 = (v28.clone().len() as i32);
                let v37: i32 = (v31.clone().len() as i32);
                let v38: i32 = v36.wrapping_sub(v37);
                let v39: i32 = 0i32;
                let v40: i32 = first_visible_offset_8(v1.clone(), v39);
                let v41: bool = &*v31 == "";
                if v41 {
                    let v42: bool = &*v35 == "";
                    let v43: bool = v42 == false;
                    let v44: bool = v28.ends_with(&*v35);
                    let v45: i32 = (v35.len() as i32);
                    let v46: i32 = v36.wrapping_sub(v45);
                    let v47: bool = v46 == v40;
                    let v48: bool = v44 && v47;
                    let v49: bool = v43 && v48;
                    if v49 {
                        let v50: i32 = (v1.clone().len() as i32);
                        let v51: Rc<str> = std::rc::Rc::<str>::from(v1.get(v40 as usize..v50 as usize).unwrap_or(""));
                        let v52: Rc<str> = comment_text_9(v51);
                        US2::US2_TodoComment(v52)
                    } else {
                        US2::US2_NoTodo
                    }
                } else {
                    let v56: i32 = v31.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let v57: i32 = leader_run_start_10(v1.clone(), v38, v56);
                    let v60: bool = match v0 {
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
                    let v61: bool = outside_quotes_11(v60, v1.clone(), v57);
                    let v62: bool = v61 == false;
                    if v62 {
                        US2::US2_NoTodo
                    } else {
                        let v64: Rc<str> = std::rc::Rc::<str>::from(v1.get(0i32 as usize..v57 as usize).unwrap_or(""));
                        let v65: i32 = 0i32;
                        let v66: bool = earlier_comment_13(v60, v29, v64, v65);
                        if v66 {
                            US2::US2_NoTodo
                        } else {
                            let v68: i32 = (v1.clone().len() as i32);
                            let v69: Rc<str> = std::rc::Rc::<str>::from(v1.get(v57 as usize..v68 as usize).unwrap_or(""));
                            let v70: Rc<str> = comment_text_9(v69);
                            US2::US2_TodoComment(v70)
                        }
                    }
                }
            }
        }
    }
}
fn leader_prefix_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
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
                v2 = v10;
                continue;
            }
        }
    }
}
fn comment_starts_line_14(mut v0: US0, mut v1: Rc<str>) -> bool {
    let mut v2: i32 = 0i32;
    let mut v3: i32 = first_visible_offset_8(v1.clone(), v2);
    let v4: Rc<str> = comment_leaders_6(v0);
    let v5: i32 = (v1.clone().len() as i32);
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.get(v3 as usize..v5 as usize).unwrap_or(""));
    let v7: i32 = 0i32;
    leader_prefix_15(v4, v6, v7)
}
fn todo_free_17(mut v0: US0, mut v1: Rc<str>) -> bool {
    let mut v2: US2 = todo_in_line_2(v0, v1);
    match v2 {
        US2::US2_NoTodo => {
            true
        }
        US2::US2_TodoComment(v3) => {
            let mut v3: Rc<str> = v3;
            false
        }
    }
}
fn block_from_16(mut v0: US0, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: i32) -> (Rc<str>, i32, i32) {
    loop {
        let mut v6: i32 = (v1.clone().len() as i32);
        let v7: bool = v6 <= v3;
        if v7 {
            return (v4, v3, v5);
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
            let v15: i32 = first_visible_offset_8(v13.clone(), v14);
            let v16: bool = v15 == v2;
            let v17: i32 = (v13.clone().len() as i32);
            let v18: Rc<str> = std::rc::Rc::<str>::from(v13.get(v2 as usize..v17 as usize).unwrap_or(""));
            let v19: bool = comment_starts_line_14(v0.clone(), v13.clone());
            let v20: bool = v16 && v19;
            let v21: bool = todo_free_17(v0.clone(), v13);
            let v22: bool = directive_line_4(v18.clone());
            let v23: bool = v22 == false;
            let v24: bool = v21 && v23;
            let v25: bool = v20 && v24;
            if v25 {
                let v26: i32 = v11.wrapping_add(1i32);
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = concat5_3(v4, v8, v18, v27.clone(), v27.clone());
                let v29: i32 = v5.wrapping_add(1i32);
                (v3, v4, v5) = (v26, v28, v29);
                continue;
            } else {
                return (v4, v3, v5);
            }
        }
    }
}
fn id_end_19(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v2 <= v1;
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
                v1 = v12;
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn todo_id_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TO"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DO"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_3(v2, v3, v4.clone(), v4.clone(), v4.clone());
    let mut v6: i32 = v1.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
    let v7: i32 = v6.wrapping_add(4i32);
    let v8: bool = v6 < 0i32;
    if v8 {
        v4
    } else {
        let v9: i32 = v1.as_bytes().get(v7 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v10: bool = v9 == 58i32;
        if v10 {
            let v11: i32 = v7.wrapping_add(1i32);
            let mut v12: i32 = first_visible_offset_8(v1.clone(), v11);
            let v13: i32 = id_end_19(v1.clone(), v12);
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
                let v33: Rc<str> = concat5_3(v31, v14.clone(), v32, v4.clone(), v4.clone());
                let v34: bool = v0.contains(&*v33);
                v34
            };
            let v36: bool = v27 && v35;
            if v36 {
                v14
            } else {
                v4
            }
        } else {
            v4
        }
    }
}
fn json_text_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = spiral_json_quote(&*v0);
    v1
}
fn finding_json_20(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = todo_id_18(v1, v4.clone());
    let mut v6: Rc<dyn Fn(Rc<str>) -> Rc<str>> = v0(v5.clone());
    let v7: Rc<str> = v6(v4.clone());
    let v8: bool = &*v5 == "";
    let v15: Rc<str> = if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let v10: Rc<str> = json_text_21(v5);
        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"id\":"); } LIT.with(|lit| lit.clone()) }, v10));
        let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"fields\":"); } LIT.with(|lit| lit.clone()) }));
        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v7));
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
        v14
    };
    let v16: Rc<str> = json_text_21(v2);
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":"); } LIT.with(|lit| lit.clone()) }, v16));
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"line\":"); } LIT.with(|lit| lit.clone()) }));
    let v19: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v15));
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"text\":"); } LIT.with(|lit| lit.clone()) }));
    let v24: Rc<str> = json_text_21(v4);
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) }));
    v26
}
fn findings_from_1(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>>, mut v1: US0, mut v2: US1, mut v3: bool, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: i32, mut v9: i32, mut v10: Rc<str>, mut v11: i32, mut v12: Rc<str>) -> (Rc<str>, i32, Rc<str>) {
    loop {
        let v13: i32 = (v5.clone().len() as i32);
        let v14: bool = v13 <= v8;
        if v14 {
            return (v10, v11, v12);
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
            match v21 {
                US2::US2_NoTodo => {
                    let v22: i32 = v9.wrapping_add(1i32);
                    (v8, v9) = (v20, v22);
                    continue;
                }
                US2::US2_TodoComment(v26) => {
                    let v26: Rc<str> = v26;
                    let v27: bool = comment_starts_line_14(v1.clone(), v19.clone());
                    let (mut v34, mut v35, mut v36): (Rc<str>, i32, i32) = if v27 {
                        let v28: i32 = 0i32;
                        let v29: i32 = first_visible_offset_8(v19, v28);
                        let v30: i32 = 0i32;
                        block_from_16(v1.clone(), v5.clone(), v29, v20, v26, v30)
                    } else {
                        (v26, v20, 0i32)
                    };
                    let v37: i32 = v9.wrapping_add(1i32);
                    let v38: i32 = v37.wrapping_add(v36);
                    let v39: Rc<str> = todo_id_18(v4.clone(), v34.clone());
                    let v46: bool = match &v2 {
                        US1::US1_AllTodos => {
                            true
                        }
                        US1::US1_TaggedTodos => {
                            let v40: bool = &*v39 == "";
                            let v41: bool = v40 == false;
                            v41
                        }
                        US1::US1_TodosOf(v42) => {
                            let v42: Rc<str> = v42.clone();
                            let v43: bool = &*v39 == &*v42;
                            v43
                        }
                    };
                    if v46 {
                        let v52: Rc<str> = if v7 {
                            finding_json_20(v0.clone(), v4.clone(), v6.clone(), v9, v34)
                        } else {
                            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let v49: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                            concat5_3(v6.clone(), v48, v49, v50, v34)
                        };
                        let v53: bool = &*v10 == "";
                        let v60: Rc<str> = if v53 {
                            v52
                        } else {
                            if v7 {
                                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\n"); } LIT.with(|lit| lit.clone()) };
                                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                concat5_3(v10, v54, v52, v55.clone(), v55.clone())
                            } else {
                                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                concat5_3(v10, v15.clone(), v52, v57.clone(), v57.clone())
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
                            concat5_3(v12, v6.clone(), v66, v67, v15)
                        } else {
                            v12
                        };
                        (v8, v9, v10, v11, v12) = (v35, v38, v60, v61, v69);
                        continue;
                    } else {
                        (v8, v9) = (v35, v38);
                        continue;
                    }
                }
            }
        }
    }
}
fn linked_marker_fields_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = agile_todo_fields::kit_agile_todo_fields(&*v0, &*v1);
    v2
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<str> {
        linked_marker_fields_22(v0.clone(), v1)
    })
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
        closure2(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn last_offset_of_23(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
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
                (v2, v3) = (v8, v2);
                continue;
            } else {
                let v10: i32 = v2.wrapping_add(1i32);
                v2 = v10;
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
    let v43: bool = &*v1 == "";
    let v49: US1 = if v43 {
        US1::US1_AllTodos
    } else {
        let v45: bool = &*v1 == "*";
        if v45 {
            US1::US1_TaggedTodos
        } else {
            US1::US1_TodosOf(v1)
        }
    };
    let v50: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<str>>> = closure1();
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spi|spir"); } LIT.with(|lit| lit.clone()) };
    let v52: i32 = 47i32;
    let v53: i32 = 0i32;
    let v54: i32 = -1i32;
    let v55: i32 = last_offset_of_23(v4.clone(), v52, v53, v54);
    let v56: i32 = v55.wrapping_add(1i32);
    let v57: i32 = (v4.clone().len() as i32);
    let mut v58: Rc<str> = std::rc::Rc::<str>::from(v4.get(v56 as usize..v57 as usize).unwrap_or(""));
    let v59: Rc<str> = std::rc::Rc::<str>::from(v58.to_ascii_lowercase());
    let v60: i32 = 46i32;
    let v61: i32 = 0i32;
    let v62: i32 = -1i32;
    let v63: i32 = last_offset_of_23(v59.clone(), v60, v61, v62);
    let v64: bool = v63 < 0i32;
    let v69: Rc<str> = if v64 {
        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v65
    } else {
        let v66: i32 = v63.wrapping_add(1i32);
        let v67: i32 = (v59.clone().len() as i32);
        let v68: Rc<str> = std::rc::Rc::<str>::from(v59.get(v66 as usize..v67 as usize).unwrap_or(""));
        v68
    };
    let v70: bool = v51.split("|").any(|item| item == &*v69);
    let v71: i32 = 0i32;
    let v72: i32 = 1i32;
    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v74: i32 = 0i32;
    findings_from_1(v50, v42, v49, v70, v2, v3, v4, v5, v71, v72, v73.clone(), v74, v73.clone())
}
fn closure0() -> Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> (Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> (Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool| -> (Rc<str>, i32, Rc<str>) {
        todo_findings_0(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        todo_id_18(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_todo_findings(v0: i32, v1: &str, v2: &str, v3: &str, v4: &str, v5: bool) -> (Rc<str>, i32, Rc<str>) {
    closure0()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), v5)
}
pub fn kit_agile_todo_id(v0: &str, v1: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
