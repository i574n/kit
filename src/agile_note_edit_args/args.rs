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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_SwitchAppend,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_ValuedAs,
    US4_ValuedExpectAt,
    US4_ValuedExpectPrefix,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US4),
    US3_None,
}
fn command_line_argument_count_2() -> i32 {
    let v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_3(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn dropped_4(mut v0: i32, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = v0 > 0i32;
                if v4 {
                    let v5: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return v1;
                }
            }
            _ => {
                return v1;
            }
        }
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn method9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn valued_applied_7(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: bool) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) {
    let v11: i32 = (v1.clone().len() as i32);
    let v12: i32 = 0i32;
    let v13: i32 = method8(v1.clone(), v11, v12);
    let v14: i32 = v11.wrapping_sub(1i32);
    let v15: Rc<str> = string_slice(&v1.clone(), v13 as i64, v14 as i64);
    let v16: i32 = (v15.clone().len() as i32);
    let v17: i32 = method9(v15.clone(), v16);
    let v18: Rc<str> = string_slice(&v15, 0i32 as i64, v17 as i64);
    let (mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = match &v0 {
        US4::US4_ValuedAs => {
            let v19: bool = &*v18 == "";
            if v19 {
                (v2, v3, v4, v5, v6, v7, v8, v9, false)
            } else {
                (v2, v3, v4, v5, v18, v7, v8, v9, v10)
            }
        }
        _ => {
            let (mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = match &v0 {
                US4::US4_ValuedExpectAt => {
                    let v29: bool = &*v18 == "";
                    if v29 {
                        (v2, v3, v4, v5, v6, v7, v8, v9, false)
                    } else {
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        (v2, v3, v4, v5, v6, v7, v18, v30, v10)
                    }
                }
                _ => {
                    let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = match v0 {
                        US4::US4_ValuedExpectPrefix => {
                            let v40: bool = &*v1 == "";
                            if v40 {
                                (v2, v3, v4, v5, v6, v7, v8, v9, false)
                            } else {
                                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                (v2, v3, v4, v5, v6, v7, v41, v1, v10)
                            }
                        }
                        _ => unreachable!(),
                    };
                    (v51, v52, v53, v54, v55, v56, v57, v58, v59)
                }
            };
            (v60, v61, v62, v63, v64, v65, v66, v67, v68)
        }
    };
    (v69, v70, v71, v72, v73, v74, v75, v76, v77)
}
fn flags_read_6(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) {
    let (mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = match &*v0 {
        UH0::UH0_Nil => {
            (v1, v2, v3, v4, v5, v6, v7, v8, v9)
        }
        _ => {
            let (mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_Cons(v10, v11) => {
                    let v10: Rc<str> = v10;
                    let v11: Rc<UH0> = v11;
                    let v12: bool = &*v10 == "--append";
                    let v16: US1 = if v12 {
                        let v13: US2 = US2::US2_SwitchAppend;
                        US1::US1_Some(v13)
                    } else {
                        US1::US1_None
                    };
                    let v17: bool = &*v10 == "--as";
                    let v29: US3 = if v17 {
                        let v18: US4 = US4::US4_ValuedAs;
                        US3::US3_Some(v18)
                    } else {
                        let v20: bool = &*v10 == "--expect-at";
                        if v20 {
                            let v21: US4 = US4::US4_ValuedExpectAt;
                            US3::US3_Some(v21)
                        } else {
                            let v23: bool = &*v10 == "--expect-prefix";
                            if v23 {
                                let v24: US4 = US4::US4_ValuedExpectPrefix;
                                US3::US3_Some(v24)
                            } else {
                                US3::US3_None
                            }
                        }
                    };
                    let v30: bool = &*v10 == "";
                    if v30 {
                        (v1, v2, v3, v4, v5, v6, v7, v8, v9)
                    } else {
                        let (mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = match v16 {
                            US1::US1_Some(v31) => {
                                let v31: US2 = v31;
                                let v32: bool = true;
                                flags_read_6(v11, v1, v2, v3, v4, v5, v32, v7, v8, v9)
                            }
                            _ => {
                                let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = match v29 {
                                    US3::US3_Some(v42) => {
                                        let v42: US4 = v42;
                                        match Rc::unwrap_or_clone(v11) {
                                            UH0::UH0_Cons(v43, v44) => {
                                                let v43: Rc<str> = v43;
                                                let v44: Rc<UH0> = v44;
                                                let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = valued_applied_7(v42, v43, v1, v2, v3, v4, v5, v6, v7, v8, v9);
                                                if v53 {
                                                    let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = flags_read_6(v44, v45, v46, v47, v48, v49, v50, v51, v52, v53);
                                                    (v54, v55, v56, v57, v58, v59, v60, v61, v62)
                                                } else {
                                                    (v45, v46, v47, v48, v49, v50, v51, v52, v53)
                                                }
                                            }
                                            _ => {
                                                (v1, v2, v3, v4, v5, v6, v7, v8, false)
                                            }
                                        }
                                    }
                                    _ => {
                                        (v1, v2, v3, v4, v5, v6, v7, v8, false)
                                    }
                                };
                                (v81, v82, v83, v84, v85, v86, v87, v88, v89)
                            }
                        };
                        (v90, v91, v92, v93, v94, v95, v96, v97, v98)
                    }
                }
                _ => unreachable!(),
            };
            (v108, v109, v110, v111, v112, v113, v114, v115, v116)
        }
    };
    (v117, v118, v119, v120, v121, v122, v123, v124, v125)
}
fn args_of_5(mut v0: &Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) {
    let (mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = match &**v0 {
        UH0::UH0_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<UH0> = v2.clone();
            match Rc::unwrap_or_clone(v2) {
                UH0::UH0_Cons(v3, v4) => {
                    let v3: Rc<str> = v3;
                    let v4: Rc<UH0> = v4;
                    match &*v4 {
                        UH0::UH0_Cons(v5, v6) => {
                            let v5: Rc<str> = v5.clone();
                            let v6: Rc<UH0> = v6.clone();
                            match &*v6 {
                                UH0::UH0_Cons(v7, v8) => {
                                    let v7: Rc<str> = v7.clone();
                                    let v8: Rc<UH0> = v8.clone();
                                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v10: bool = false;
                                    let v11: bool = true;
                                    flags_read_6(v8, v1, v3, v5, v7, v9.clone(), v10, v9.clone(), v9, v11)
                                }
                                _ => {
                                    let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = match Rc::unwrap_or_clone(v6) {
                                        UH0::UH0_Nil => {
                                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            (v1, v3, v5, v21.clone(), v21.clone(), false, v21.clone(), v21, true)
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v22, v23, v24, v25, v26, v27, v28, v29, v30)
                                }
                            }
                        }
                        _ => {
                            let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = match Rc::unwrap_or_clone(v4) {
                                UH0::UH0_Nil => {
                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    (v1, v3, v40.clone(), v40.clone(), v40.clone(), false, v40.clone(), v40, true)
                                }
                                _ => unreachable!(),
                            };
                            (v41, v42, v43, v44, v45, v46, v47, v48, v49)
                        }
                    }
                }
                _ => {
                    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (v59.clone(), v59.clone(), v59.clone(), v59.clone(), v59.clone(), false, v59.clone(), v59, true)
                }
            }
        }
        _ => {
            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v69.clone(), v69.clone(), v69.clone(), v69.clone(), v69.clone(), false, v69.clone(), v69, true)
        }
    };
    (v70, v71, v72, v73, v74, v75, v76, v77, v78)
}
fn note_args_read_0(mut v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool)>>> {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = dropped_4(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = args_of_5(&v3);
    Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v8, v9, v10, v11, v12)]))
}
fn closure0() -> Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool)>>>> = Rc::new(move |mut v0: i32| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool)>>> {
        note_args_read_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn agile_note_edit_args_read(v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool)>>> {
    closure0()(v0)
}
