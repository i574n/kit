#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_KindUser,
    US0_KindFeedback,
    US0_KindProject,
    US0_KindReference,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoMemories,
    UH0_MemoryThen(Rc<str>, Rc<str>, Rc<str>, US0, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_EntryRead(Rc<str>, Rc<str>, Rc<str>, US0, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US1_EntryRefused(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_KindKnown(US0),
    US2_KindUnknown,
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum UH1 {
    UH1_NoRanked,
    UH1_RankedThen(i32, Rc<str>, Rc<str>, Rc<str>, US0, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
fn file_present_1(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn memories_flipped_4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_MemoryThen(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: US0 = v5;
                let mut v6: Rc<str> = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let v10: Rc<UH0> = v10;
                let v11: Rc<UH0> = Rc::new(UH0::UH0_MemoryThen(v2, v3, v4, v5, v6, v7, v8, v9, v1));
                (v0, v1) = (v10, v11);
                continue;
            }
            UH0::UH0_NoMemories => {
                return v1;
            }
        }
    }
}
fn kind_of_case_6(mut v0: Rc<str>) -> US2 {
    let mut v1: bool = &*v0 == "User";
    if v1 {
        let v2: US0 = US0::US0_KindUser;
        US2::US2_KindKnown(v2)
    } else {
        let v4: bool = &*v0 == "Feedback";
        if v4 {
            let v5: US0 = US0::US0_KindFeedback;
            US2::US2_KindKnown(v5)
        } else {
            let v7: bool = &*v0 == "Project";
            if v7 {
                let v8: US0 = US0::US0_KindProject;
                US2::US2_KindKnown(v8)
            } else {
                let v10: bool = &*v0 == "Reference";
                if v10 {
                    let v11: US0 = US0::US0_KindReference;
                    US2::US2_KindKnown(v11)
                } else {
                    US2::US2_KindUnknown
                }
            }
        }
    }
}
fn unescaped_from_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> (Rc<str>, i32) {
    loop {
        let v4: i32 = (v0).as_bytes().get(v1 as usize).map(|byte| *byte as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        let v7: bool = if v5 {
            true
        } else {
            let mut v6: bool = v4 == 34i32;
            v6
        };
        if v7 {
            let v8: Rc<str> = std::rc::Rc::<str>::from((v0).get(v2 as usize..v1 as usize).unwrap_or(""));
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = concat5_2(v3, v8, v9.clone(), v9.clone(), v9.clone());
            let v11: i32 = v1.wrapping_add(1i32);
            return (v10, v11);
        } else {
            let v12: bool = v4 == 92i32;
            if v12 {
                let v13: i32 = v1.wrapping_add(1i32);
                let v14: i32 = (v0).as_bytes().get(v13 as usize).map(|byte| *byte as i32).unwrap_or(-1);
                let v15: Rc<str> = std::rc::Rc::<str>::from((v0).get(v2 as usize..v1 as usize).unwrap_or(""));
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = concat5_2(v3, v15, v16.clone(), v16.clone(), v16.clone());
                let v18: bool = v14 == 110i32;
                if v18 {
                    let v19: i32 = v1.wrapping_add(2i32);
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<str> = concat5_2(v17, v20, v16.clone(), v16.clone(), v16.clone());
                    (v1, v2, v3) = (v19, v19, v21);
                    continue;
                } else {
                    let v24: bool = v14 == 114i32;
                    if v24 {
                        let v25: i32 = v1.wrapping_add(2i32);
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = concat5_2(v17, v26, v16.clone(), v16.clone(), v16.clone());
                        (v1, v2, v3) = (v25, v25, v27);
                        continue;
                    } else {
                        let v30: bool = v14 == 116i32;
                        if v30 {
                            let v31: i32 = v1.wrapping_add(2i32);
                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let v33: Rc<str> = concat5_2(v17, v32, v16.clone(), v16.clone(), v16.clone());
                            (v1, v2, v3) = (v31, v31, v33);
                            continue;
                        } else {
                            let v36: i32 = v1.wrapping_add(2i32);
                            (v1, v2, v3) = (v36, v13, v17);
                            continue;
                        }
                    }
                }
            } else {
                let v45: i32 = v1.wrapping_add(1i32);
                v1 = v45;
                continue;
            }
        }
    }
}
fn quoted_at_7(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, i32) {
    let v2: bool = v1 < 0i32;
    let v5: i32 = if v2 {
        -1i32
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = (v0).get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|at| at as i32 + v1).unwrap_or(-1);
        v4
    };
    let mut v6: bool = v5 < 0i32;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v7, -1i32)
    } else {
        let v8: i32 = v5.wrapping_add(1i32);
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        unescaped_from_8(v0, v8, v8, v9)
    }
}
fn entry_of_line_5(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= Memory ("); } LIT.with(|lit| lit.clone()) };
    let v2: i32 = (v0).get(0i32 as usize..).and_then(|rest| rest.find(&*v1)).map(|at| at as i32 + 0i32).unwrap_or(-1);
    let v3: i32 = v2.wrapping_add(10i32);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = (v0).get(v3 as usize..).and_then(|rest| rest.find(&*v4)).map(|at| at as i32 + v3).unwrap_or(-1);
    let mut v6: Rc<str> = std::rc::Rc::<str>::from((v0).get(v3 as usize..v5 as usize).unwrap_or(""));
    let v7: US2 = kind_of_case_6(v6);
    match v7 {
        US2::US2_KindKnown(v13) => {
            let v13: US0 = v13;
            let (mut v14, mut v15): (Rc<str>, i32) = quoted_at_7(v0.clone(), v5);
            let (mut v16, mut v17): (Rc<str>, i32) = quoted_at_7(v0.clone(), v15);
            let (mut v18, mut v19): (Rc<str>, i32) = quoted_at_7(v0.clone(), v17);
            let (mut v20, mut v21): (Rc<str>, i32) = quoted_at_7(v0.clone(), v19);
            let (mut v22, mut v23): (Rc<str>, i32) = quoted_at_7(v0.clone(), v21);
            let (mut v24, mut v25): (Rc<str>, i32) = quoted_at_7(v0.clone(), v23);
            let (mut v26, mut v27): (Rc<str>, i32) = quoted_at_7(v0.clone(), v25);
            let v28: bool = v27 < 0i32;
            if v28 {
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed memory line: "); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = { let text = &*v0; if text.chars().count() <= 80i32 as usize { std::rc::Rc::<str>::from(text) } else { std::rc::Rc::<str>::from(text.chars().take((80i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = concat5_2(v29, v30, v31.clone(), v31.clone(), v31.clone());
                US1::US1_EntryRefused(v32)
            } else {
                US1::US1_EntryRead(v14, v16, v18, v13, v20, v22, v24, v26)
            }
        }
        US2::US2_KindUnknown => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown memory kind in: "); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { let text = &*v0; if text.chars().count() <= 80i32 as usize { std::rc::Rc::<str>::from(text) } else { std::rc::Rc::<str>::from(text.chars().take((80i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = concat5_2(v8, v9, v10.clone(), v10.clone(), v10.clone());
            US1::US1_EntryRefused(v11)
        }
    }
}
fn store_from_3(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>) -> (Rc<UH0>, i32, Rc<str>, Rc<str>) {
    let mut v6: i32 = (v0.clone().len() as i32);
    let v7: bool = v1 >= v6;
    let (mut v60, mut v61, mut v62, mut v63): (Rc<UH0>, i32, Rc<str>, Rc<str>) = if v7 {
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
        let v9: Rc<UH0> = memories_flipped_4(v2, v8);
        (v9, v3, v4, v5)
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v11: i32 = (v0).get(v1 as usize..).and_then(|rest| rest.find(&*v10)).map(|at| at as i32 + v1).unwrap_or(-1);
        let v12: bool = v11 < 0i32;
        let v13: i32 = if v12 {
            v6
        } else {
            v11
        };
        let v14: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize..v13 as usize).unwrap_or(""));
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl memory_"); } LIT.with(|lit| lit.clone()) };
        let v16: bool = v14.starts_with(&*v15);
        let v19: bool = if v16 {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= Memory ("); } LIT.with(|lit| lit.clone()) };
            let v18: bool = v14.contains(&*v17);
            v18
        } else {
            false
        };
        let (mut v51, mut v52, mut v53, mut v54): (Rc<UH0>, i32, Rc<str>, Rc<str>) = if v19 {
            let v20: US1 = entry_of_line_5(v14);
            let (mut v38, mut v39, mut v40, mut v41): (Rc<UH0>, i32, Rc<str>, Rc<str>) = match &v20 {
                US1::US1_EntryRead(v21, v22, v23, v24, v25, v26, v27, v28) => {
                    let v21: Rc<str> = v21.clone();
                    let v22: Rc<str> = v22.clone();
                    let v23: Rc<str> = v23.clone();
                    let v24: US0 = v24.clone();
                    let v25: Rc<str> = v25.clone();
                    let v26: Rc<str> = v26.clone();
                    let v27: Rc<str> = v27.clone();
                    let v28: Rc<str> = v28.clone();
                    let v29: i32 = v3.wrapping_add(1i32);
                    let v30: Rc<UH0> = Rc::new(UH0::UH0_MemoryThen(v21, v22, v23, v24, v25, v26, v27, v28, v2));
                    (v30, v29, v4, v5)
                }
                _ => {
                    let (mut v34, mut v35, mut v36, mut v37): (Rc<UH0>, i32, Rc<str>, Rc<str>) = match v20 {
                        US1::US1_EntryRefused(v31) => {
                            let v31: Rc<str> = v31;
                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v33: Rc<str> = concat5_2(v5, v31, v10, v32.clone(), v32.clone());
                            (v2, v3, v4, v33)
                        }
                        _ => unreachable!(),
                    };
                    (v34, v35, v36, v37)
                }
            };
            (v38, v39, v40, v41)
        } else {
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl index_of_memory () = MemoryIndex ("); } LIT.with(|lit| lit.clone()) };
            let v43: bool = v14.starts_with(&*v42);
            let (mut v47, mut v48, mut v49, mut v50): (Rc<UH0>, i32, Rc<str>, Rc<str>) = if v43 {
                let v44: i32 = 0i32;
                let (mut v45, mut v46): (Rc<str>, i32) = quoted_at_7(v14, v44);
                (v2, v3, v45, v5)
            } else {
                (v2, v3, v4, v5)
            };
            (v47, v48, v49, v50)
        };
        let v55: i32 = v13.wrapping_add(1i32);
        let (mut v56, mut v57, mut v58, mut v59): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_3(v0, v55, v51, v52, v53, v54);
        (v56, v57, v58, v59)
    };
    (v60, v61, v62, v63)
}
fn slug_lines_9(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_MemoryThen(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: US0 = v5;
                let mut v6: Rc<str> = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let v10: Rc<UH0> = v10;
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = concat5_2(v1, v3, v11, v12.clone(), v12.clone());
                (v0, v1) = (v10, v13);
                continue;
            }
            UH0::UH0_NoMemories => {
                return v1;
            }
        }
    }
}
fn dangling_in_11(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[["); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = (v0).get(v1 as usize..).and_then(|rest| rest.find(&*v5)).map(|at| at as i32 + v1).unwrap_or(-1);
        let v7: bool = v6 < 0i32;
        let v10: i32 = if v7 {
            -1i32
        } else {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]]"); } LIT.with(|lit| lit.clone()) };
            let v9: i32 = (v0).get(v6 as usize..).and_then(|rest| rest.find(&*v8)).map(|at| at as i32 + v6).unwrap_or(-1);
            v9
        };
        let v11: bool = v10 < 0i32;
        if v11 {
            return v4;
        } else {
            let v12: i32 = v6.wrapping_add(2i32);
            let v13: Rc<str> = std::rc::Rc::<str>::from((v0).get(v12 as usize..v10 as usize).unwrap_or(""));
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = concat5_2(v14.clone(), v13.clone(), v14.clone(), v15.clone(), v15.clone());
            let v17: bool = v2.contains(&*v16);
            let v18: i32 = v10.wrapping_add(2i32);
            let v22: Rc<str> = if v17 {
                v4
            } else {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": dangling [["); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]]\n"); } LIT.with(|lit| lit.clone()) };
                concat5_2(v4, v3.clone(), v19, v13, v20)
            };
            (v1, v4) = (v18, v22);
            continue;
        }
    }
}
fn checked_10(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: i32) -> (Rc<str>, i32, i32, i32) {
    let (mut v61, mut v62, mut v63, mut v64): (Rc<str>, i32, i32, i32) = match &*v0 {
        UH0::UH0_NoMemories => {
            (v3, v4, v5, v6)
        }
        _ => {
            let (mut v57, mut v58, mut v59, mut v60): (Rc<str>, i32, i32, i32) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_MemoryThen(v7, v8, v9, v10, v11, v12, v13, v14, v15) => {
                    let v7: Rc<str> = v7;
                    let v8: Rc<str> = v8;
                    let v9: Rc<str> = v9;
                    let v10: US0 = v10;
                    let v11: Rc<str> = v11;
                    let v12: Rc<str> = v12;
                    let v13: Rc<str> = v13;
                    let v14: Rc<str> = v14;
                    let v15: Rc<UH0> = v15;
                    let v16: i32 = 0i32;
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<str> = dangling_in_11(v14.clone(), v16, v1.clone(), v7.clone(), v17.clone());
                    let v19: i32 = (v18).lines().count() as i32;
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**Why:**"); } LIT.with(|lit| lit.clone()) };
                    let v21: bool = v14.contains(&*v20);
                    let v24: bool = if v21 {
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**How to apply:**"); } LIT.with(|lit| lit.clone()) };
                        let v23: bool = v14.contains(&*v22);
                        v23
                    } else {
                        false
                    };
                    let v26: bool = match &v10 {
                        US0::US0_KindFeedback => {
                            true
                        }
                        US0::US0_KindProject => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v28: bool = if v26 {
                        let v27: bool = v24 == false;
                        v27
                    } else {
                        false
                    };
                    let v38: Rc<str> = if v28 {
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
                        let v34: Rc<str> = match v10 {
                            US0::US0_KindFeedback => {
                                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                                v31
                            }
                            US0::US0_KindProject => {
                                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                                v32
                            }
                            US0::US0_KindReference => {
                                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                                v33
                            }
                            US0::US0_KindUser => {
                                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                                v30
                            }
                        };
                        let v35: Rc<str> = std::rc::Rc::<str>::from((v34).to_ascii_lowercase().as_str());
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" without **Why:** and **How to apply:**\n"); } LIT.with(|lit| lit.clone()) };
                        concat5_2(v7.clone(), v29, v35, v36, v17.clone())
                    } else {
                        v17.clone()
                    };
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]("); } LIT.with(|lit| lit.clone()) };
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                    let v41: Rc<str> = concat5_2(v39, v7.clone(), v40, v17.clone(), v17.clone());
                    let v42: bool = v2.contains(&*v41);
                    let v45: Rc<str> = if v42 {
                        v17.clone()
                    } else {
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": missing from MEMORY.md\n"); } LIT.with(|lit| lit.clone()) };
                        concat5_2(v7, v43, v17.clone(), v17.clone(), v17.clone())
                    };
                    let v46: Rc<str> = concat5_2(v3, v18, v38.clone(), v45, v17);
                    let v47: i32 = v4.wrapping_add(v19);
                    let v48: bool = &*v38 == "";
                    let v49: i32 = if v48 {
                        0i32
                    } else {
                        1i32
                    };
                    let v50: i32 = v5.wrapping_add(v49);
                    let v51: i32 = if v42 {
                        0i32
                    } else {
                        1i32
                    };
                    let v52: i32 = v6.wrapping_add(v51);
                    checked_10(v15, v1, v2, v46, v47, v50, v52)
                }
                _ => unreachable!(),
            };
            (v57, v58, v59, v60)
        }
    };
    (v61, v62, v63, v64)
}
fn file_lines_12(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_MemoryThen(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: US0 = v5;
                let mut v6: Rc<str> = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let v10: Rc<UH0> = v10;
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = concat5_2(v1, v2, v11, v12.clone(), v12.clone());
                (v0, v1) = (v10, v13);
                continue;
            }
            UH0::UH0_NoMemories => {
                return v1;
            }
        }
    }
}
fn index_targets_missing_13(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]("); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = (v0).get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|at| at as i32 + v1).unwrap_or(-1);
        let mut v6: bool = v5 < 0i32;
        let v9: i32 = if v6 {
            -1i32
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = (v0).get(v5 as usize..).and_then(|rest| rest.find(&*v7)).map(|at| at as i32 + v5).unwrap_or(-1);
            v8
        };
        let v10: bool = v9 < 0i32;
        if v10 {
            return v3;
        } else {
            let v11: i32 = v5.wrapping_add(2i32);
            let v12: Rc<str> = std::rc::Rc::<str>::from((v0).get(v11 as usize..v9 as usize).unwrap_or(""));
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = concat5_2(v13.clone(), v12.clone(), v13.clone(), v14.clone(), v14.clone());
            let v16: bool = v2.contains(&*v15);
            let v17: i32 = v9.wrapping_add(1i32);
            let v21: bool = if v16 {
                true
            } else {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".md"); } LIT.with(|lit| lit.clone()) };
                let v19: bool = v12.ends_with(&*v18);
                let v20: bool = v19 == false;
                v20
            };
            let v24: Rc<str> = if v21 {
                v3
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MEMORY.md: entry for missing "); } LIT.with(|lit| lit.clone()) };
                concat5_2(v3, v22, v12, v13, v14)
            };
            (v1, v3) = (v17, v24);
            continue;
        }
    }
}
fn method15(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_14(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn memory_check_0(mut v0: Rc<str>) -> i32 {
    let mut v1: bool = file_present_1(v0.clone());
    let v2: bool = v1 == false;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check needs a readable data file: "); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = concat5_2(v3, v0, v4.clone(), v4.clone(), v4.clone());
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = concat5_2(v6, v5, v4.clone(), v4.clone(), v4);
        eprintln!("{}", v7);
        2i32
    } else {
        let v8: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
        let v9: i32 = 0i32;
        let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
        let v11: i32 = 0i32;
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let (mut v13, mut v14, mut v15, mut v16): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_3(v8, v9, v10, v11, v12.clone(), v12.clone());
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = slug_lines_9(v13.clone(), v17.clone());
        let v19: i32 = 0i32;
        let v20: i32 = 0i32;
        let v21: i32 = 0i32;
        let (mut v22, mut v23, mut v24, mut v25): (Rc<str>, i32, i32, i32) = checked_10(v13.clone(), v18, v15.clone(), v12.clone(), v19, v20, v21);
        let v26: i32 = 0i32;
        let v27: Rc<str> = file_lines_12(v13, v17);
        let v28: Rc<str> = index_targets_missing_13(v15, v26, v27, v12.clone());
        let v29: i32 = (v28).lines().count() as i32;
        let v30: i32 = v24.wrapping_add(v25);
        let v31: i32 = v30.wrapping_add(v29);
        let v32: Rc<str> = concat5_2(v22, v28, v16, v12.clone(), v12.clone());
        print!("{}", &*v32);
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" memories="); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = format_real_14(v14);
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" dangling_links="); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = format_real_14(v23);
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" missing_reasons="); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<str> = concat5_2(v33, v34, v35, v36, v37);
        let v39: bool = v31 == 0i32;
        let v42: Rc<str> = if v39 {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit memory check ok"); } LIT.with(|lit| lit.clone()) };
            v40
        } else {
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit memory check failed"); } LIT.with(|lit| lit.clone()) };
            v41
        };
        let v43: Rc<str> = format_real_14(v24);
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unindexed="); } LIT.with(|lit| lit.clone()) };
        let v45: Rc<str> = format_real_14(v25);
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" index_missing="); } LIT.with(|lit| lit.clone()) };
        let v47: Rc<str> = format_real_14(v29);
        let v48: Rc<str> = concat5_2(v44, v45, v46, v47, v12.clone());
        let v49: Rc<str> = concat5_2(v42, v38, v43, v48, v12);
        println!("{}", v49);
        if v39 {
            0i32
        } else {
            1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        memory_check_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn kind_of_word_17(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = std::rc::Rc::<str>::from((v1).to_ascii_lowercase().as_str());
    let v3: bool = &*v0 == &*v2;
    if v3 {
        let v4: US0 = US0::US0_KindUser;
        US2::US2_KindKnown(v4)
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = std::rc::Rc::<str>::from((v6).to_ascii_lowercase().as_str());
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: US0 = US0::US0_KindFeedback;
            US2::US2_KindKnown(v9)
        } else {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = std::rc::Rc::<str>::from((v11).to_ascii_lowercase().as_str());
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US0 = US0::US0_KindProject;
                US2::US2_KindKnown(v14)
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = std::rc::Rc::<str>::from((v16).to_ascii_lowercase().as_str());
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US0 = US0::US0_KindReference;
                    US2::US2_KindKnown(v19)
                } else {
                    US2::US2_KindUnknown
                }
            }
        }
    }
}
fn listed_18(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_MemoryThen(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: US0 = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<UH0> = v11;
                let v12: bool = &*v1 == "";
                let v20: bool = if v12 {
                    true
                } else {
                    let v17: Rc<str> = match &v6 {
                        US0::US0_KindFeedback => {
                            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                            v14
                        }
                        US0::US0_KindProject => {
                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                            v15
                        }
                        US0::US0_KindReference => {
                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                            v16
                        }
                        US0::US0_KindUser => {
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                            v13
                        }
                    };
                    let v18: Rc<str> = std::rc::Rc::<str>::from((v17).to_ascii_lowercase().as_str());
                    let v19: bool = &*v1 == &*v18;
                    v19
                };
                if v20 {
                    let v25: Rc<str> = match v6 {
                        US0::US0_KindFeedback => {
                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                            v22
                        }
                        US0::US0_KindProject => {
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                            v23
                        }
                        US0::US0_KindReference => {
                            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                            v24
                        }
                        US0::US0_KindUser => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                            v21
                        }
                    };
                    let v26: Rc<str> = std::rc::Rc::<str>::from((v25).to_ascii_lowercase().as_str());
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                    let v28: Rc<str> = { let text = &*v5; if text.chars().count() <= 100i32 as usize { std::rc::Rc::<str>::from(text) } else { std::rc::Rc::<str>::from(text.chars().take((100i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
                    let v29: Rc<str> = concat5_2(v26, v27.clone(), v4, v27.clone(), v28);
                    println!("{}", v29);
                    let v30: i32 = v2.wrapping_add(1i32);
                    (v0, v2) = (v11, v30);
                    continue;
                } else {
                    v0 = v11;
                    continue;
                }
            }
            UH0::UH0_NoMemories => {
                return v2;
            }
        }
    }
}
fn memory_list_run_16(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: bool = file_present_1(v1.clone());
    let v3: bool = v2 == false;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list needs a readable data file: "); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = concat5_2(v4, v1, v5.clone(), v5.clone(), v5.clone());
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = concat5_2(v7, v6, v5.clone(), v5.clone(), v5);
        eprintln!("{}", v8);
        2i32
    } else {
        let v9: bool = &*v0 == "";
        let v13: US2 = if v9 {
            let v10: US0 = US0::US0_KindUser;
            US2::US2_KindKnown(v10)
        } else {
            kind_of_word_17(v0.clone())
        };
        match v13 {
            US2::US2_KindKnown(v36) => {
                let v36: US0 = v36;
                let v37: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v1).unwrap_or_default().as_str());
                let v38: i32 = 0i32;
                let v39: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
                let v40: i32 = 0i32;
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let (mut v42, mut v43, mut v44, mut v45): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_3(v37, v38, v39, v40, v41.clone(), v41.clone());
                let v46: i32 = 0i32;
                let v47: i32 = listed_18(v42, v0.clone(), v46);
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit memory list ok shown="); } LIT.with(|lit| lit.clone()) };
                let v49: Rc<str> = format_real_14(v47);
                let v52: Rc<str> = if v9 {
                    v41.clone()
                } else {
                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" kind="); } LIT.with(|lit| lit.clone()) };
                    concat5_2(v50, v0, v41.clone(), v41.clone(), v41.clone())
                };
                let v53: Rc<str> = concat5_2(v48, v49, v52, v41.clone(), v41);
                println!("{}", v53);
                0i32
            }
            US2::US2_KindUnknown => {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list --kind expects "); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = std::rc::Rc::<str>::from((v15).to_ascii_lowercase().as_str());
                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v16));
                let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v17));
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = std::rc::Rc::<str>::from((v19).to_ascii_lowercase().as_str());
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v20));
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v21));
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = std::rc::Rc::<str>::from((v23).to_ascii_lowercase().as_str());
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v24));
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v25));
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = std::rc::Rc::<str>::from((v27).to_ascii_lowercase().as_str());
                let v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v28));
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v29));
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", got "); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = concat5_2(v14, v30, v31, v0, v32.clone());
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = concat5_2(v34, v33, v32.clone(), v32.clone(), v32);
                eprintln!("{}", v35);
                2i32
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        memory_list_run_16(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn query_score_21(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US0, mut v5: &Rc<str>, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: Rc<str>, mut v9: i32) -> i32 {
    loop {
        let v10: bool = &*v0 == "";
        if v10 {
            return v9;
        } else {
            let v11: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let v12: Rc<str> = std::rc::Rc::<str>::from((v2).to_lowercase().as_str());
            let v13: i32 = (v12).matches(&*v11).count() as i32;
            let v14: i32 = 5i32.wrapping_mul(v13);
            let v15: Rc<str> = std::rc::Rc::<str>::from((v3).to_lowercase().as_str());
            let v16: i32 = (v15).matches(&*v11).count() as i32;
            let v17: i32 = 3i32.wrapping_mul(v16);
            let v18: i32 = v14.wrapping_add(v17);
            let v19: Rc<str> = std::rc::Rc::<str>::from((v8).to_lowercase().as_str());
            let v20: i32 = (v19).matches(&*v11).count() as i32;
            let v21: i32 = v18.wrapping_add(v20);
            let v22: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let v23: i32 = v9.wrapping_add(v21);
            (v0, v9) = (v22, v23);
            continue;
        }
    }
}
fn ranked_inserted_22(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<UH1>) -> Rc<UH1> {
    match &*v9 {
        UH1::UH1_NoRanked => {
            let v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRanked); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_RankedThen(v0, v1, v2, v3, v4, v5, v6, v7, v8, v10))
        }
        UH1::UH1_RankedThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21) => {
            let v12: i32 = *v12;
            let v13: Rc<str> = v13.clone();
            let v14: Rc<str> = v14.clone();
            let v15: Rc<str> = v15.clone();
            let v16: US0 = v16.clone();
            let v17: Rc<str> = v17.clone();
            let v18: Rc<str> = v18.clone();
            let v19: Rc<str> = v19.clone();
            let v20: Rc<str> = v20.clone();
            let v21: Rc<UH1> = v21.clone();
            let v22: bool = v0 > v12;
            let v26: bool = if v22 {
                true
            } else {
                let v23: bool = v0 == v12;
                if v23 {
                    let v24: bool = v2 < v14;
                    v24
                } else {
                    false
                }
            };
            if v26 {
                Rc::new(UH1::UH1_RankedThen(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
            } else {
                let v28: Rc<UH1> = ranked_inserted_22(v0, v1, v2, v3, v4, v5, v6, v7, v8, v21);
                Rc::new(UH1::UH1_RankedThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v28))
            }
        }
    }
}
fn ranked_from_20(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_MemoryThen(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: US0 = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<UH0> = v11;
                let v12: i32 = 0i32;
                let v13: i32 = query_score_21(v1.clone(), &v3, v4.clone(), v5.clone(), v6.clone(), &v7, &v8, &v9, v10.clone(), v12);
                let v14: bool = v13 > 0i32;
                let v16: Rc<UH1> = if v14 {
                    ranked_inserted_22(v13, v3, v4, v5, v6, v7, v8, v9, v10, v2)
                } else {
                    v2
                };
                (v0, v2) = (v11, v16);
                continue;
            }
            UH0::UH0_NoMemories => {
                return v2;
            }
        }
    }
}
fn ranked_printed_23(mut v0: Rc<UH1>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_NoRanked => {
                return v2;
            }
            UH1::UH1_RankedThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let v3: i32 = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<str> = v6;
                let v7: US0 = v7;
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let v12: Rc<UH1> = v12;
                let v13: bool = v2 < v1;
                if v13 {
                    let v14: Rc<str> = format_real_14(v3);
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                    let v20: Rc<str> = match v7 {
                        US0::US0_KindFeedback => {
                            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                            v17
                        }
                        US0::US0_KindProject => {
                            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                            v18
                        }
                        US0::US0_KindReference => {
                            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                            v19
                        }
                        US0::US0_KindUser => {
                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                            v16
                        }
                    };
                    let v21: Rc<str> = std::rc::Rc::<str>::from((v20).to_ascii_lowercase().as_str());
                    let v22: Rc<str> = { let text = &*v6; if text.chars().count() <= 100i32 as usize { std::rc::Rc::<str>::from(text) } else { std::rc::Rc::<str>::from(text.chars().take((100i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = concat5_2(v5, v15.clone(), v22, v23.clone(), v23.clone());
                    let v25: Rc<str> = concat5_2(v14, v15.clone(), v21, v15.clone(), v24);
                    println!("{}", v25);
                    let v26: i32 = v2.wrapping_add(1i32);
                    (v0, v2) = (v12, v26);
                    continue;
                } else {
                    return v2;
                }
            }
        }
    }
}
fn memory_recall_19(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: bool = file_present_1(v2.clone());
    let v4: bool = v3 == false;
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("recall needs a readable data file: "); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = concat5_2(v5, v2, v6.clone(), v6.clone(), v6.clone());
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = concat5_2(v8, v7, v6.clone(), v6.clone(), v6);
        eprintln!("{}", v9);
        2i32
    } else {
        let v10: Rc<str> = std::rc::Rc::<str>::from((v1).to_lowercase().split_whitespace().collect::<Vec<_>>().join("
").as_str());
        let v11: bool = &*v10 == "";
        if v11 {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("recall needs query words"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = concat5_2(v13, v12, v14.clone(), v14.clone(), v14);
            eprintln!("{}", v15);
            2i32
        } else {
            let v16: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v2).unwrap_or_default().as_str());
            let v17: i32 = 0i32;
            let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
            let v19: i32 = 0i32;
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let (mut v21, mut v22, mut v23, mut v24): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_3(v16, v17, v18, v19, v20.clone(), v20.clone());
            let v25: Rc<str> = std::rc::Rc::<str>::from((v1).to_lowercase().split_whitespace().collect::<Vec<_>>().join("
").as_str());
            let v26: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRanked); } CASE.with(|case| case.clone()) };
            let v27: Rc<UH1> = ranked_from_20(v21, v25, v26);
            let v28: i32 = 0i32;
            let v29: i32 = ranked_printed_23(v27, v0, v28);
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit memory recall ok shown="); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = format_real_14(v29);
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of="); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = format_real_14(v22);
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" query="); } LIT.with(|lit| lit.clone()) };
            let v35: Rc<str> = concat5_2(v34, v1, v20.clone(), v20.clone(), v20);
            let v36: Rc<str> = concat5_2(v30, v31, v32, v33, v35);
            println!("{}", v36);
            0i32
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32| -> i32 {
        memory_recall_19(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_memory_check(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_memory_list(v0: &str, v1: &str) -> i32 {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_memory_recall(v0: &str, v1: &str, v2: i32) -> i32 {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2)
}
pub type SpiralShape_97df63ebabc3 = Mut0;
