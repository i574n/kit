#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_MemoryUser,
    US0_MemoryFeedback,
    US0_MemoryProject,
    US0_MemoryReference,
    US0_MemoryUntyped,
}
#[derive(Clone)]
enum US1 {
    US1_CitesOpen,
    US1_CitesDone,
    US1_CitesMissing,
}
#[derive(Clone)]
enum US2 {
    US2_NoFinding,
    US2_DanglingId(Rc<str>, Rc<str>),
    US2_DoneCited(Rc<str>, Rc<str>),
    US2_UnwrittenLink(Rc<str>, Rc<str>),
    US2_UnknownMemory(Rc<str>, Rc<str>),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_SwitchJson,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US5),
    US4_None,
}
#[derive(Clone)]
enum US7 {
    US7_ValuedMemory,
}
#[derive(Clone)]
enum US6 {
    US6_Some(US7),
    US6_None,
}
fn directory_exists_2(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn names_from_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = &*v1 == "";
        if v3 {
            return v2;
        } else {
            let v4: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(10u8)).next().unwrap_or(""));
            let v5: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v4).to_string_lossy().as_ref());
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v5).unwrap_or_default().as_str());
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("name"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v6; let front = text.strip_prefix("---").and_then(|rest| rest.split_once("
---").map(|(front, _)| front)).unwrap_or(""); front.lines().find_map(|line| line.trim().strip_prefix(&*v7).and_then(|rest| rest.strip_prefix(char::from(58u8))).map(|value| value.trim().trim_matches(char::from(34u8)).to_owned())).unwrap_or_default() }.as_str());
            let v9: bool = &*v8 == "";
            let v11: Rc<str> = if v9 {
                let v10: Rc<str> = std::rc::Rc::<str>::from((v4).trim_end_matches(".md"));
                v10
            } else {
                v8
            };
            let v12: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let v13: bool = &*v2 == "";
            let v16: Rc<str> = if v13 {
                v11
            } else {
                let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }));
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v11));
                v15
            };
            (v1, v2) = (v12, v16);
            continue;
        }
    }
}
fn links_start_4() -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0i32, 0i32, 0i32, 0i32, 0i32, 0i32, v0)
}
fn ids_checked_6(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let v11: bool = &*v3 == "";
    let (mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113): (i32, i32, i32, i32, i32, i32, Rc<str>) = if v11 {
        (v4, v5, v6, v7, v8, v9, v10)
    } else {
        let v12: Rc<str> = std::rc::Rc::<str>::from((v3).split(char::from(10u8)).next().unwrap_or(""));
        let v13: i32 = v5.wrapping_add(1i32);
        let mut v14: Rc<str> = std::rc::Rc::<str>::from((v3).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let v15: Rc<str> = agile_task_census::kit_agile_task_status(&*v0, &*v12);
        let v16: bool = &*v15 == "";
        let v22: US1 = if v16 {
            US1::US1_CitesMissing
        } else {
            let v18: bool = &*v15 == "Done";
            if v18 {
                US1::US1_CitesDone
            } else {
                US1::US1_CitesOpen
            }
        };
        let v32: US2 = match v22 {
            US1::US1_CitesDone => {
                let v25: bool = match &v1 {
                    US0::US0_MemoryFeedback => {
                        true
                    }
                    US0::US0_MemoryProject => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v25 {
                    US2::US2_DoneCited(v2.clone(), v12)
                } else {
                    US2::US2_NoFinding
                }
            }
            US1::US1_CitesMissing => {
                US2::US2_DanglingId(v2.clone(), v12)
            }
            US1::US1_CitesOpen => {
                US2::US2_NoFinding
            }
        };
        let (mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
            US2::US2_NoFinding => {
                (v4, v13, v6, v7, v8, v9, v10)
            }
            _ => {
                let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                    US2::US2_DanglingId(v33, v34) => {
                        let v33: Rc<str> = v33.clone();
                        let v34: Rc<str> = v34.clone();
                        let v35: i32 = v6.wrapping_add(1i32);
                        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dangling  "); } LIT.with(|lit| lit.clone()) }, v33));
                        let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v34));
                        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  no backlog item with this id\n"); } LIT.with(|lit| lit.clone()) }));
                        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v39));
                        (v4, v13, v35, v7, v8, v9, v40)
                    }
                    _ => {
                        let (mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                            US2::US2_DoneCited(v41, v42) => {
                                let v41: Rc<str> = v41.clone();
                                let v42: Rc<str> = v42.clone();
                                let v43: i32 = v7.wrapping_add(1i32);
                                let v44: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done  "); } LIT.with(|lit| lit.clone()) }, v41));
                                let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v42));
                                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  the item is Done; check that the rule still holds\n"); } LIT.with(|lit| lit.clone()) }));
                                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v47));
                                (v4, v13, v6, v43, v8, v9, v48)
                            }
                            _ => {
                                let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                                    US2::US2_UnwrittenLink(v49, v50) => {
                                        let v49: Rc<str> = v49.clone();
                                        let v50: Rc<str> = v50.clone();
                                        let v51: i32 = v8.wrapping_add(1i32);
                                        let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unwritten-link  "); } LIT.with(|lit| lit.clone()) }, v49));
                                        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  [["); } LIT.with(|lit| lit.clone()) }));
                                        let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v50));
                                        let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]]  no memory with this name yet\n"); } LIT.with(|lit| lit.clone()) }));
                                        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v55));
                                        (v4, v13, v6, v7, v51, v9, v56)
                                    }
                                    _ => {
                                        let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (i32, i32, i32, i32, i32, i32, Rc<str>) = match v32 {
                                            US2::US2_UnknownMemory(v57, v58) => {
                                                let v57: Rc<str> = v57;
                                                let v58: Rc<str> = v58;
                                                let v59: i32 = v9.wrapping_add(1i32);
                                                let v60: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown-memory  "); } LIT.with(|lit| lit.clone()) }, v57));
                                                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                                                let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v58));
                                                let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  a backlog note cites this memory, which does not exist\n"); } LIT.with(|lit| lit.clone()) }));
                                                let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v63));
                                                (v4, v13, v6, v7, v8, v59, v64)
                                            }
                                            _ => unreachable!(),
                                        };
                                        (v65, v66, v67, v68, v69, v70, v71)
                                    }
                                };
                                (v72, v73, v74, v75, v76, v77, v78)
                            }
                        };
                        (v79, v80, v81, v82, v83, v84, v85)
                    }
                };
                (v86, v87, v88, v89, v90, v91, v92)
            }
        };
        let (mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106): (i32, i32, i32, i32, i32, i32, Rc<str>) = ids_checked_6(v0, v1, v2, v14, v93, v94, v95, v96, v97, v98, v99);
        (v100, v101, v102, v103, v104, v105, v106)
    };
    (v107, v108, v109, v110, v111, v112, v113)
}
fn method9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method10(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn loop_8(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let v4: bool = v3 >= v2;
        if v4 {
            return false;
        } else {
            let v5: Rc<str> = v1.borrow()[v3 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let v7: i32 = 0i32;
            let v8: i32 = method9(v5.clone(), v6, v7);
            let v9: i32 = v6.wrapping_sub(1i32);
            let v10: Rc<str> = string_slice(&v5, v8 as i64, v9 as i64);
            let v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = method10(v10.clone(), v11);
            let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
            let mut v14: bool = &*v13 == &*v0;
            if v14 {
                return true;
            } else {
                let v15: i32 = v3.wrapping_add(1i32);
                v3 = v15;
                continue;
            }
        }
    }
}
fn links_checked_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let v10: bool = &*v2 == "";
    let (mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101): (i32, i32, i32, i32, i32, i32, Rc<str>) = if v10 {
        (v3, v4, v5, v6, v7, v8, v9)
    } else {
        let v11: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let v12: Rc<str> = std::rc::Rc::<str>::from((v2).split(char::from(10u8)).next().unwrap_or(""));
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v13).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v15: i32 = (v14.clone().borrow().len() as i32);
        let v16: i32 = 0i32;
        let v17: bool = loop_8(v12.clone(), &v14, v15, v16);
        let v20: US2 = if v17 {
            US2::US2_NoFinding
        } else {
            US2::US2_UnwrittenLink(v1.clone(), v12)
        };
        let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
            US2::US2_NoFinding => {
                (v3, v4, v5, v6, v7, v8, v9)
            }
            _ => {
                let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                    US2::US2_DanglingId(v21, v22) => {
                        let v21: Rc<str> = v21.clone();
                        let v22: Rc<str> = v22.clone();
                        let v23: i32 = v5.wrapping_add(1i32);
                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dangling  "); } LIT.with(|lit| lit.clone()) }, v21));
                        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v22));
                        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  no backlog item with this id\n"); } LIT.with(|lit| lit.clone()) }));
                        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v27));
                        (v3, v4, v23, v6, v7, v8, v28)
                    }
                    _ => {
                        let (mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                            US2::US2_DoneCited(v29, v30) => {
                                let v29: Rc<str> = v29.clone();
                                let v30: Rc<str> = v30.clone();
                                let v31: i32 = v6.wrapping_add(1i32);
                                let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done  "); } LIT.with(|lit| lit.clone()) }, v29));
                                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                                let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v30));
                                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  the item is Done; check that the rule still holds\n"); } LIT.with(|lit| lit.clone()) }));
                                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v35));
                                (v3, v4, v5, v31, v7, v8, v36)
                            }
                            _ => {
                                let (mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                                    US2::US2_UnwrittenLink(v37, v38) => {
                                        let v37: Rc<str> = v37.clone();
                                        let v38: Rc<str> = v38.clone();
                                        let v39: i32 = v7.wrapping_add(1i32);
                                        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unwritten-link  "); } LIT.with(|lit| lit.clone()) }, v37));
                                        let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  [["); } LIT.with(|lit| lit.clone()) }));
                                        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v38));
                                        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]]  no memory with this name yet\n"); } LIT.with(|lit| lit.clone()) }));
                                        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v43));
                                        (v3, v4, v5, v6, v39, v8, v44)
                                    }
                                    _ => {
                                        let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59): (i32, i32, i32, i32, i32, i32, Rc<str>) = match v20 {
                                            US2::US2_UnknownMemory(v45, v46) => {
                                                let v45: Rc<str> = v45;
                                                let v46: Rc<str> = v46;
                                                let v47: i32 = v8.wrapping_add(1i32);
                                                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown-memory  "); } LIT.with(|lit| lit.clone()) }, v45));
                                                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                                                let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v46));
                                                let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  a backlog note cites this memory, which does not exist\n"); } LIT.with(|lit| lit.clone()) }));
                                                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v51));
                                                (v3, v4, v5, v6, v7, v47, v52)
                                            }
                                            _ => unreachable!(),
                                        };
                                        (v53, v54, v55, v56, v57, v58, v59)
                                    }
                                };
                                (v60, v61, v62, v63, v64, v65, v66)
                            }
                        };
                        (v67, v68, v69, v70, v71, v72, v73)
                    }
                };
                (v74, v75, v76, v77, v78, v79, v80)
            }
        };
        let (mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94): (i32, i32, i32, i32, i32, i32, Rc<str>) = links_checked_7(v0, v1, v11, v81, v82, v83, v84, v85, v86, v87);
        (v88, v89, v90, v91, v92, v93, v94)
    };
    (v95, v96, v97, v98, v99, v100, v101)
}
fn memories_checked_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let v12: bool = &*v4 == "";
    let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (i32, i32, i32, i32, i32, i32, Rc<str>) = if v12 {
        (v5, v6, v7, v8, v9, v10, v11)
    } else {
        let v13: Rc<str> = std::rc::Rc::<str>::from((v4).split(char::from(10u8)).next().unwrap_or(""));
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v3).join(&*v13).to_string_lossy().as_ref());
        let v15: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v14).unwrap_or_default().as_str());
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("name"); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v15; let front = text.strip_prefix("---").and_then(|rest| rest.split_once("
---").map(|(front, _)| front)).unwrap_or(""); front.lines().find_map(|line| line.trim().strip_prefix(&*v16).and_then(|rest| rest.strip_prefix(char::from(58u8))).map(|value| value.trim().trim_matches(char::from(34u8)).to_owned())).unwrap_or_default() }.as_str());
        let v18: bool = &*v17 == "";
        let v20: Rc<str> = if v18 {
            let v19: Rc<str> = std::rc::Rc::<str>::from((v13).trim_end_matches(".md"));
            v19
        } else {
            v17
        };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v15; let front = text.strip_prefix("---").and_then(|rest| rest.split_once("
---").map(|(front, _)| front)).unwrap_or(""); front.lines().find_map(|line| line.trim().strip_prefix(&*v21).and_then(|rest| rest.strip_prefix(char::from(58u8))).map(|value| value.trim().trim_matches(char::from(34u8)).to_owned())).unwrap_or_default() }.as_str());
        let v23: bool = &*v22 == "user";
        let v38: US0 = if v23 {
            US0::US0_MemoryUser
        } else {
            let v25: bool = &*v22 == "feedback";
            if v25 {
                US0::US0_MemoryFeedback
            } else {
                let v27: bool = &*v22 == "project";
                if v27 {
                    US0::US0_MemoryProject
                } else {
                    let v29: bool = &*v22 == "reference";
                    if v29 {
                        US0::US0_MemoryReference
                    } else {
                        US0::US0_MemoryUntyped
                    }
                }
            }
        };
        let v39: Rc<str> = std::rc::Rc::<str>::from({ let known = (v1).split(char::from(124u8)).collect::<std::collections::HashSet<_>>(); let mut ids = (v15).split(|c: char| !(c.is_ascii_alphanumeric() || c == char::from(45u8) || c == char::from(95u8))).map(|token| token.trim_matches(char::from(45u8))).filter(|token| token.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == char::from(45u8))).filter(|token| token.contains(char::from(45u8)) && token.split(char::from(45u8)).all(|part| !part.is_empty()) && token.starts_with(|c: char| c.is_ascii_uppercase()) && known.contains(token.split(char::from(45u8)).next().unwrap_or(""))).map(str::to_owned).collect::<Vec<_>>(); ids.sort(); ids.dedup(); ids.join("
") }.as_str());
        let v40: i32 = v5.wrapping_add(1i32);
        let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47): (i32, i32, i32, i32, i32, i32, Rc<str>) = ids_checked_6(v0.clone(), v38, v20.clone(), v39, v40, v6, v7, v8, v9, v10, v11);
        let v48: Rc<str> = std::rc::Rc::<str>::from((v4).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let v49: Rc<str> = std::rc::Rc::<str>::from({ let mut links = (v15).split("[[").skip(1).filter_map(|rest| rest.split_once("]]").map(|(target, _)| target.trim().to_owned())).filter(|target| !target.is_empty()).collect::<Vec<_>>(); links.sort(); links.dedup(); links.join("
") }.as_str());
        let (mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (i32, i32, i32, i32, i32, i32, Rc<str>) = links_checked_7(v2.clone(), v20, v49, v41, v42, v43, v44, v45, v46, v47);
        let (mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (i32, i32, i32, i32, i32, i32, Rc<str>) = memories_checked_5(v0, v1, v2, v3, v48, v50, v51, v52, v53, v54, v55, v56);
        (v57, v58, v59, v60, v61, v62, v63)
    };
    (v64, v65, v66, v67, v68, v69, v70)
}
fn cites_in_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let v11: i32 = (v2).get(v3 as usize..).and_then(|rest| rest.find("memory ")).map(|at| at as i32 + v3).unwrap_or(-1);
    let v12: bool = v11 < 0i32;
    let (mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107): (i32, i32, i32, i32, i32, i32, Rc<str>) = if v12 {
        (v4, v5, v6, v7, v8, v9, v10)
    } else {
        let v13: i32 = v11.wrapping_add(7i32);
        let mut v14: Rc<str> = std::rc::Rc::<str>::from((v2).get(v13 as usize..).unwrap_or("").chars().take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == char::from(45u8)).collect::<String>().trim_end_matches(char::from(45u8)));
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v15).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v17: i32 = (v16.clone().borrow().len() as i32);
        let v18: i32 = 0i32;
        let v19: bool = loop_8(v14.clone(), &v16, v17, v18);
        let v26: US2 = if v19 {
            US2::US2_NoFinding
        } else {
            let v21: i32 = (v14).matches(char::from(45u8)).count() as i32;
            let v22: bool = v21 >= 2i32;
            if v22 {
                US2::US2_UnknownMemory(v1.clone(), v14)
            } else {
                US2::US2_NoFinding
            }
        };
        let (mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
            US2::US2_NoFinding => {
                (v4, v5, v6, v7, v8, v9, v10)
            }
            _ => {
                let (mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                    US2::US2_DanglingId(v27, v28) => {
                        let v27: Rc<str> = v27.clone();
                        let v28: Rc<str> = v28.clone();
                        let v29: i32 = v6.wrapping_add(1i32);
                        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dangling  "); } LIT.with(|lit| lit.clone()) }, v27));
                        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v28));
                        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  no backlog item with this id\n"); } LIT.with(|lit| lit.clone()) }));
                        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v33));
                        (v4, v5, v29, v7, v8, v9, v34)
                    }
                    _ => {
                        let (mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                            US2::US2_DoneCited(v35, v36) => {
                                let v35: Rc<str> = v35.clone();
                                let v36: Rc<str> = v36.clone();
                                let v37: i32 = v7.wrapping_add(1i32);
                                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done  "); } LIT.with(|lit| lit.clone()) }, v35));
                                let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                                let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v36));
                                let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  the item is Done; check that the rule still holds\n"); } LIT.with(|lit| lit.clone()) }));
                                let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v41));
                                (v4, v5, v6, v37, v8, v9, v42)
                            }
                            _ => {
                                let (mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                                    US2::US2_UnwrittenLink(v43, v44) => {
                                        let v43: Rc<str> = v43.clone();
                                        let v44: Rc<str> = v44.clone();
                                        let v45: i32 = v8.wrapping_add(1i32);
                                        let v46: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unwritten-link  "); } LIT.with(|lit| lit.clone()) }, v43));
                                        let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  [["); } LIT.with(|lit| lit.clone()) }));
                                        let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v44));
                                        let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]]  no memory with this name yet\n"); } LIT.with(|lit| lit.clone()) }));
                                        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v49));
                                        (v4, v5, v6, v7, v45, v9, v50)
                                    }
                                    _ => {
                                        let (mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65): (i32, i32, i32, i32, i32, i32, Rc<str>) = match v26 {
                                            US2::US2_UnknownMemory(v51, v52) => {
                                                let v51: Rc<str> = v51;
                                                let v52: Rc<str> = v52;
                                                let v53: i32 = v9.wrapping_add(1i32);
                                                let v54: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown-memory  "); } LIT.with(|lit| lit.clone()) }, v51));
                                                let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                                                let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v52));
                                                let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  a backlog note cites this memory, which does not exist\n"); } LIT.with(|lit| lit.clone()) }));
                                                let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v57));
                                                (v4, v5, v6, v7, v8, v53, v58)
                                            }
                                            _ => unreachable!(),
                                        };
                                        (v59, v60, v61, v62, v63, v64, v65)
                                    }
                                };
                                (v66, v67, v68, v69, v70, v71, v72)
                            }
                        };
                        (v73, v74, v75, v76, v77, v78, v79)
                    }
                };
                (v80, v81, v82, v83, v84, v85, v86)
            }
        };
        let (mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100): (i32, i32, i32, i32, i32, i32, Rc<str>) = cites_in_12(v0, v1, v2, v13, v87, v88, v89, v90, v91, v92, v93);
        (v94, v95, v96, v97, v98, v99, v100)
    };
    (v101, v102, v103, v104, v105, v106, v107)
}
fn notes_checked_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let v10: i32 = (v1).get(v2 as usize..).and_then(|rest| rest.find("= Note (")).map(|at| at as i32 + v2).unwrap_or(-1);
    let v11: bool = v10 < 0i32;
    let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (i32, i32, i32, i32, i32, i32, Rc<str>) = if v11 {
        (v3, v4, v5, v6, v7, v8, v9)
    } else {
        let v12: i32 = (v1).get(v10 as usize..).and_then(|rest| rest.find(char::from(10u8))).map(|at| at as i32 + v10).unwrap_or((v1).len() as i32);
        let v13: Rc<str> = std::rc::Rc::<str>::from((v1).get(v10 as usize..v12 as usize).unwrap_or(""));
        let mut v14: Rc<str> = std::rc::Rc::<str>::from((v13).split(char::from(34u8)).nth(1).unwrap_or(""));
        let v15: Rc<str> = std::rc::Rc::<str>::from((v1).get(v10 as usize..v12 as usize).unwrap_or(""));
        let v16: i32 = 0i32;
        let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (i32, i32, i32, i32, i32, i32, Rc<str>) = cites_in_12(v0.clone(), v14, v15, v16, v3, v4, v5, v6, v7, v8, v9);
        let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (i32, i32, i32, i32, i32, i32, Rc<str>) = notes_checked_11(v0, v1, v12, v17, v18, v19, v20, v21, v22, v23);
        (v24, v25, v26, v27, v28, v29, v30)
    };
    (v31, v32, v33, v34, v35, v36, v37)
}
fn method14(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_13(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn links_checked_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> i32 {
    let mut v3: bool = &*v1 == "";
    let mut v6: bool = if v3 {
        true
    } else {
        let v4: bool = directory_exists_2(v1.clone());
        let v5: bool = v4 == false;
        v5
    };
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile links rejected: --memory <dir> must name a memory directory"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v7);
        2i32
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v8).to_string_lossy().as_ref());
        let v10: Rc<str> = agile_note_listing::kit_agile_state_segments(&*v9);
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile.spi"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v9).join(&*v11).to_string_lossy().as_ref());
        let v13: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v12).unwrap_or_default().as_str());
        let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        v14.borrow_mut().push(v13);
        v14.borrow_mut().push(v10);
        let v15: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
        let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v17));
        let v19: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v18);
        let v20: Rc<str> = std::rc::Rc::<str>::from({ let mut names = std::fs::read_dir(&*v1).into_iter().flatten().flatten().map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.ends_with(".md") && name != "MEMORY.md").collect::<Vec<_>>(); names.sort(); names.join("
") }.as_str());
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = names_from_3(v1.clone(), v20.clone(), v21);
        let v23: i32 = 0i32;
        let v24: Rc<str> = std::rc::Rc::<str>::from({ let mut prefixes = (v19).match_indices("= Task (").filter_map(|(at, marker)| (v19)[at + marker.len()..].strip_prefix(char::from(34u8)).and_then(|rest| rest.split(char::from(45u8)).next()).map(str::to_owned)).collect::<Vec<_>>(); prefixes.sort(); prefixes.dedup(); prefixes.join("|") }.as_str());
        let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31): (i32, i32, i32, i32, i32, i32, Rc<str>) = links_start_4();
        let (mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38): (i32, i32, i32, i32, i32, i32, Rc<str>) = memories_checked_5(v19.clone(), v24, v22.clone(), v1, v20, v25, v26, v27, v28, v29, v30, v31);
        let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (i32, i32, i32, i32, i32, i32, Rc<str>) = notes_checked_11(v22, v19, v23, v32, v33, v34, v35, v36, v37, v38);
        let v46: Rc<str> = format_real_13(v39);
        let v47: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memories="); } LIT.with(|lit| lit.clone()) }, v46));
        let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cited="); } LIT.with(|lit| lit.clone()) }));
        let v49: Rc<str> = format_real_13(v40);
        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
        let v51: Rc<str> = format_real_13(v41);
        let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" dangling="); } LIT.with(|lit| lit.clone()) }, v51));
        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" done="); } LIT.with(|lit| lit.clone()) }));
        let v54: Rc<str> = format_real_13(v42);
        let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
        let v56: Rc<str> = format_real_13(v43);
        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unwritten_links="); } LIT.with(|lit| lit.clone()) }, v56));
        let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unknown_memories="); } LIT.with(|lit| lit.clone()) }));
        let v59: Rc<str> = format_real_13(v44);
        let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v60));
        let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v61));
        if v2 {
            let v63: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); (v45).lines().filter_map(|line| { let mut parts = line.splitn(4, "  "); Some(format!("{{{q}kind{q}:{q}{}{q},{q}memory{q}:{q}{}{q},{q}target{q}:{q}{}{q}}}", parts.next()?, parts.next()?, parts.next()?.trim_start_matches("[[").trim_end_matches("]]"))) }).collect::<Vec<_>>().join(",") }.as_str());
            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"findings\":["); } LIT.with(|lit| lit.clone()) }, v63));
            let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v64, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"dangling\":"); } LIT.with(|lit| lit.clone()) }));
            let v66: Rc<str> = format_real_13(v41);
            let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v66));
            let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) }));
            println!("{}", v68);
            ()
        } else {
            let v69: bool = &*v45 != "";
            if v69 {
                let v70: i32 = (v45.clone().len() as i32);
                let v71: i32 = method10(v45.clone(), v70);
                let v72: Rc<str> = string_slice(&v45, 0i32 as i64, v71 as i64);
                println!("{}", v72);
                ()
            };
            let v73: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile links ok "); } LIT.with(|lit| lit.clone()) }, v62));
            println!("{}", v73);
            ()
        };
        let v74: bool = v41 == 0i32;
        if v74 {
            0i32
        } else {
            1i32
        }
    }
}
fn command_line_argument_count_16() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_17(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_15(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_16();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US3 = if v4 {
        let v5: Rc<str> = command_line_argument_17(v0);
        US3::US3_Some(v5)
    } else {
        US3::US3_None
    };
    match v8 {
        US3::US3_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US3::US3_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_15(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn links_flags_read_18(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: bool) -> (Rc<str>, bool) {
    let (mut v36, mut v37): (Rc<str>, bool) = match &*v0 {
        UH0::UH0_Nil => {
            (v1, v2)
        }
        _ => {
            let (mut v34, mut v35): (Rc<str>, bool) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_Cons(v3, v4) => {
                    let mut v3: Rc<str> = v3;
                    let v4: Rc<UH0> = v4;
                    let v5: bool = &*v3 == "--json";
                    let v9: US4 = if v5 {
                        let mut v6: US5 = US5::US5_SwitchJson;
                        US4::US4_Some(v6)
                    } else {
                        US4::US4_None
                    };
                    let v10: bool = &*v3 == "--memory";
                    let mut v14: US6 = if v10 {
                        let v11: US7 = US7::US7_ValuedMemory;
                        US6::US6_Some(v11)
                    } else {
                        US6::US6_None
                    };
                    match v9 {
                        US4::US4_Some(v15) => {
                            let v15: US5 = v15;
                            let v16: bool = true;
                            links_flags_read_18(v4, v1, v16)
                        }
                        _ => {
                            let (mut v30, mut v31): (Rc<str>, bool) = match v14 {
                                US6::US6_Some(v19) => {
                                    let v19: US7 = v19;
                                    match &*v4 {
                                        UH0::UH0_Cons(v20, v21) => {
                                            let v20: Rc<str> = v20.clone();
                                            let v21: Rc<UH0> = v21.clone();
                                            links_flags_read_18(v21, v20, v2)
                                        }
                                        _ => {
                                            let (mut v24, mut v25): (Rc<str>, bool) = links_flags_read_18(v4, v1, v2);
                                            (v24, v25)
                                        }
                                    }
                                }
                                _ => {
                                    let (mut v28, mut v29): (Rc<str>, bool) = links_flags_read_18(v4, v1, v2);
                                    (v28, v29)
                                }
                            };
                            (v30, v31)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v34, v35)
        }
    };
    (v36, v37)
}
fn links_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_15(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v9: bool = false;
                            let (mut v10, mut v11): (Rc<str>, bool) = links_flags_read_18(v7, v8, v9);
                            links_checked_1(v6, v10, v11)
                        }
                        _ => {
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v14: bool = false;
                            links_checked_1(v13.clone(), v13, v14)
                        }
                    }
                }
                _ => {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v18: bool = false;
                    links_checked_1(v17.clone(), v17, v18)
                }
            }
        }
        _ => {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v22: bool = false;
            links_checked_1(v21.clone(), v21, v22)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        links_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_links_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
