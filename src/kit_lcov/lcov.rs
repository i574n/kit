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
    US0_BetweenSources,
    US0_InSource(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_SourceBeforeEnd,
    US2_EmptySource,
    US2_HitBeforeSource,
    US2_InvalidHitFields,
    US2_InvalidHitLine,
    US2_InvalidHitCount,
    US2_EndBeforeSource,
    US2_MalformedRecord,
    US2_UnterminatedSource,
    US2_NoLineRecords,
}
#[derive(Clone)]
enum US1 {
    US1_LcovRecords(Rc<str>),
    US1_LcovRejected(US2),
}
#[derive(Clone)]
enum US3 {
    US3_SourceLine(Rc<str>),
    US3_HitLine(Rc<str>),
    US3_EndLine,
    US3_MalformedLine,
    US3_OtherLine,
}
#[derive(Clone)]
enum US4 {
    US4_HitRead(Rc<str>),
    US4_HitRejected(US2),
}
struct Mut0 { l0: Rc<str> }
fn method5(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn hit_reading_of_3(mut v0: Rc<str>) -> US4 {
    let v1: bool = { let parts = v0.split(char::from(44u8)).collect::<Vec<_>>(); (2..=3).contains(&parts.len()) && parts[..2].iter().all(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())) };
    if v1 {
        let v2: u64 = v0.split(char::from(44u8)).next().and_then(|line| line.parse::<u32>().ok()).filter(|line| *line > 0).map(|line| line as u64).unwrap_or(0);
        let v3: bool = v2 == 0u64;
        if v3 {
            let v4: US2 = US2::US2_InvalidHitLine;
            US4::US4_HitRejected(v4.clone())
        } else {
            let v6: bool = v0.split(char::from(44u8)).nth(1).map(|hits| hits.parse::<u64>().is_ok()).unwrap_or(false);
            if v6 {
                let v7: Rc<str> = format_real_4(v2);
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
                let v10: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(44u8)).nth(1).unwrap_or(""));
                let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
                US4::US4_HitRead(v11.clone())
            } else {
                let v13: US2 = US2::US2_InvalidHitCount;
                US4::US4_HitRejected(v13.clone())
            }
        }
    } else {
        let v17: US2 = US2::US2_InvalidHitFields;
        US4::US4_HitRejected(v17.clone())
    }
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn lcov_from_2(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: US0, mut v4: Rc<str>) -> US1 {
    loop {
        let v5: bool = v1 < v2;
        if v5 {
            let v6: u64 = v1.wrapping_add(1u64);
            let v7: i32 = (v1 as i32);
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v8).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v10: bool = v7 >= 0i32;
            let v20: bool = if v10 {
                let v11: i32 = (v9.clone().borrow().len() as i32);
                let v12: bool = v11 > 0i32;
                let v16: bool = if v12 {
                    let v13: i32 = v11.wrapping_sub(1i32);
                    let v14: Rc<str> = v9.clone().borrow()[v13 as usize].clone();
                    let v15: bool = &*v14 == "";
                    v15
                } else {
                    false
                };
                let v18: i32 = if v16 {
                    let v17: i32 = v11.wrapping_sub(1i32);
                    v17
                } else {
                    v11
                };
                let v19: bool = v7 < v18;
                v19
            } else {
                false
            };
            let mut v33: Rc<str> = if v20 {
                let v21: Rc<str> = v9.clone().borrow()[v7 as usize].clone();
                let v22: i32 = (v21.clone().len() as i32);
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v24: bool = v21.ends_with(&*v23);
                let v25: bool = v24 == false;
                if v25 {
                    v21.clone()
                } else {
                    let v26: bool = v22 == 1i32;
                    if v26 {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v27.clone()
                    } else {
                        let v28: i32 = v22.wrapping_sub(2i32);
                        let v29: Rc<str> = string_slice(&v21.clone(), 0i32 as i64, v28 as i64);
                        v29.clone()
                    }
                }
            } else {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v32.clone()
            };
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SF:"); } LIT.with(|lit| lit.clone()) };
            let v35: bool = v33.starts_with(&*v34);
            let v54: US3 = if v35 {
                let v36: Rc<str> = std::rc::Rc::<str>::from(v33.strip_prefix(&*v34).unwrap_or(""));
                US3::US3_SourceLine(v36.clone())
            } else {
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DA:"); } LIT.with(|lit| lit.clone()) };
                let v39: bool = v33.starts_with(&*v38);
                if v39 {
                    let v40: Rc<str> = std::rc::Rc::<str>::from(v33.strip_prefix(&*v38).unwrap_or(""));
                    US3::US3_HitLine(v40.clone())
                } else {
                    let v42: bool = &*v33 == "end_of_record";
                    if v42 {
                        US3::US3_EndLine
                    } else {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DA"); } LIT.with(|lit| lit.clone()) };
                        let v45: bool = v33.starts_with(&*v44);
                        let v48: bool = if v45 {
                            true
                        } else {
                            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SF"); } LIT.with(|lit| lit.clone()) };
                            let v47: bool = v33.starts_with(&*v46);
                            v47
                        };
                        if v48 {
                            US3::US3_MalformedLine
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
            };
            match &v54 {
                US3::US3_EndLine => {
                    match &v3 {
                        US0::US0_BetweenSources => {
                            let v100: US2 = US2::US2_EndBeforeSource;
                            return US1::US1_LcovRejected(v100.clone());
                        }
                        US0::US0_InSource(v102) => {
                            let v102: Rc<str> = v102.clone();
                            let v103: US0 = US0::US0_BetweenSources;
                            (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v103.clone(), v4.clone());
                            continue;
                        }
                    }
                }
                US3::US3_HitLine(v80) => {
                    let v80: Rc<str> = v80.clone();
                    match &v3 {
                        US0::US0_BetweenSources => {
                            let v81: US2 = US2::US2_HitBeforeSource;
                            return US1::US1_LcovRejected(v81.clone());
                        }
                        US0::US0_InSource(v83) => {
                            let v83: Rc<str> = v83.clone();
                            let v84: US4 = hit_reading_of_3(v80.clone());
                            match &v84 {
                                US4::US4_HitRead(v85) => {
                                    let v85: Rc<str> = v85.clone();
                                    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                    let v88: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v83, v86)), v85));
                                    let v89: bool = &*v4 == "";
                                    let v92: Rc<str> = if v89 {
                                        v88.clone()
                                    } else {
                                        let v91: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v8)), v88));
                                        v91.clone()
                                    };
                                    (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v3.clone(), v92.clone());
                                    continue;
                                }
                                US4::US4_HitRejected(v94) => {
                                    let v94: US2 = v94.clone();
                                    return US1::US1_LcovRejected(v94.clone());
                                }
                            }
                        }
                    }
                }
                US3::US3_MalformedLine => {
                    let v107: US2 = US2::US2_MalformedRecord;
                    return US1::US1_LcovRejected(v107.clone());
                }
                US3::US3_OtherLine => {
                    (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v3.clone(), v4.clone());
                    continue;
                }
                US3::US3_SourceLine(v55) => {
                    let v55: Rc<str> = v55.clone();
                    match &v3 {
                        US0::US0_BetweenSources => {
                            let v59: i32 = (v55.clone().len() as i32);
                            let v60: i32 = 0i32;
                            let v61: i32 = method6(v55.clone(), v59, v60);
                            let v62: i32 = v59.wrapping_sub(1i32);
                            let v63: Rc<str> = string_slice(&v55.clone(), v61 as i64, v62 as i64);
                            let v64: i32 = (v63.clone().len() as i32);
                            let v65: i32 = method7(v63.clone(), v64);
                            let v66: Rc<str> = string_slice(&v63.clone(), 0i32 as i64, v65 as i64);
                            let v67: bool = &*v66 == "";
                            if v67 {
                                let v68: US2 = US2::US2_EmptySource;
                                return US1::US1_LcovRejected(v68.clone());
                            } else {
                                let v70: Rc<str> = std::rc::Rc::<str>::from(v55.replace(char::from(92u8), "/"));
                                let v71: US0 = US0::US0_InSource(v70.clone());
                                let v72: bool = &*v4 == "";
                                let v75: Rc<str> = if v72 {
                                    v70.clone()
                                } else {
                                    let v74: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v8)), v70));
                                    v74.clone()
                                };
                                (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v71.clone(), v75.clone());
                                continue;
                            }
                        }
                        US0::US0_InSource(v56) => {
                            let v56: Rc<str> = v56.clone();
                            let v57: US2 = US2::US2_SourceBeforeEnd;
                            return US1::US1_LcovRejected(v57.clone());
                        }
                    }
                }
            }
        } else {
            match &v3 {
                US0::US0_BetweenSources => {
                    let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v119: bool = v4.contains(&*v118);
                    if v119 {
                        return US1::US1_LcovRecords(v4.clone());
                    } else {
                        let v121: US2 = US2::US2_NoLineRecords;
                        return US1::US1_LcovRejected(v121.clone());
                    }
                }
                US0::US0_InSource(v115) => {
                    let v115: Rc<str> = v115.clone();
                    let v116: US2 = US2::US2_UnterminatedSource;
                    return US1::US1_LcovRejected(v116.clone());
                }
            }
        }
    }
}
fn lcov_reading_of_1(mut v0: Rc<str>) -> US1 {
    let v1: u64 = 0u64;
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v4: i32 = (v3.clone().borrow().len() as i32);
    let v5: bool = v4 > 0i32;
    let v9: bool = if v5 {
        let v6: i32 = v4.wrapping_sub(1i32);
        let v7: Rc<str> = v3.clone().borrow()[v6 as usize].clone();
        let v8: bool = &*v7 == "";
        v8
    } else {
        false
    };
    let v11: i32 = if v9 {
        let v10: i32 = v4.wrapping_sub(1i32);
        v10
    } else {
        v4
    };
    let v12: u64 = (v11 as u64);
    let v13: US0 = US0::US0_BetweenSources;
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    lcov_from_2(v0.clone(), v1, v12, v13.clone(), v14.clone())
}
fn lcov_error_0(mut v0: Rc<str>) -> Rc<str> {
    let v1: US1 = lcov_reading_of_1(v0);
    match &v1 {
        US1::US1_LcovRecords(v2) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US1::US1_LcovRejected(v4) => {
            let v4: US2 = v4.clone();
            match &v4 {
                US2::US2_EmptySource => {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has an empty SF record"); } LIT.with(|lit| lit.clone()) };
                    v6
                }
                US2::US2_EndBeforeSource => {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has end_of_record before SF"); } LIT.with(|lit| lit.clone()) };
                    v11
                }
                US2::US2_HitBeforeSource => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has DA before SF"); } LIT.with(|lit| lit.clone()) };
                    v7
                }
                US2::US2_InvalidHitCount => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has invalid DA hits"); } LIT.with(|lit| lit.clone()) };
                    v10
                }
                US2::US2_InvalidHitFields => {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has invalid DA fields"); } LIT.with(|lit| lit.clone()) };
                    v8
                }
                US2::US2_InvalidHitLine => {
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has invalid DA line"); } LIT.with(|lit| lit.clone()) };
                    v9
                }
                US2::US2_MalformedRecord => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has a malformed source or line record"); } LIT.with(|lit| lit.clone()) };
                    v12
                }
                US2::US2_NoLineRecords => {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage input has no line records"); } LIT.with(|lit| lit.clone()) };
                    v14
                }
                US2::US2_SourceBeforeEnd => {
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has SF before end_of_record"); } LIT.with(|lit| lit.clone()) };
                    v5
                }
                US2::US2_UnterminatedSource => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has an unterminated source record"); } LIT.with(|lit| lit.clone()) };
                    v13
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        lcov_error_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn lcov_rows_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: US1 = lcov_reading_of_1(v0);
    match &v1 {
        US1::US1_LcovRecords(v2) => {
            let v2: Rc<str> = v2.clone();
            v2
        }
        US1::US1_LcovRejected(v3) => {
            let v3: US2 = v3.clone();
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        lcov_rows_8(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lcov_error(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_lcov_rows(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
