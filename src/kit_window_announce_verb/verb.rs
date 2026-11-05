#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
    US2_LiveGui,
    US2_TestSuite,
    US2_HeavyBuild,
    US2_Compile,
    US2_CompilerInstall,
    US2_TreeRead,
    US2_Bench,
    US2_CompileHold,
    US2_TreeWrite,
}
#[derive(Clone)]
enum US1 {
    US1_ClassKnown(US2),
    US1_ClassUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_StartAt(u64),
    US3_StartRefused(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
struct Mut1 { l0: Rc<str> }
fn refused_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: window announce: "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v0));
    eprintln!("{}", v2);
    2i32
}
fn command_line_argument_count_4() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_5(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_3(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_4();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_5(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_3(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn verb_words_2() -> Rc<UH0> {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_3(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4.clone();
                    let v5: Rc<UH0> = v5.clone();
                    v5
                }
                _ => {
                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                }
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn class_word_7(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US2::US2_Compile => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US2::US2_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US2::US2_HeavyBuild => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US2::US2_LiveGui => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US2::US2_TestSuite => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US2::US2_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US2::US2_TreeWrite => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-write"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn class_of_6(mut v0: Rc<str>) -> US1 {
    let v1: US2 = US2::US2_LiveGui;
    let v2: Rc<str> = class_word_7(v1);
    let v3: bool = &*v0 == &*v2;
    if v3 {
        let v4: US2 = US2::US2_LiveGui;
        US1::US1_ClassKnown(v4)
    } else {
        let mut v6: US2 = US2::US2_TestSuite;
        let v7: Rc<str> = class_word_7(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let mut v9: US2 = US2::US2_TestSuite;
            US1::US1_ClassKnown(v9)
        } else {
            let v11: US2 = US2::US2_HeavyBuild;
            let v12: Rc<str> = class_word_7(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let mut v14: US2 = US2::US2_HeavyBuild;
                US1::US1_ClassKnown(v14)
            } else {
                let v16: US2 = US2::US2_Compile;
                let mut v17: Rc<str> = class_word_7(v16);
                let mut v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US2 = US2::US2_Compile;
                    US1::US1_ClassKnown(v19)
                } else {
                    let mut v21: US2 = US2::US2_CompilerInstall;
                    let v22: Rc<str> = class_word_7(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US2 = US2::US2_CompilerInstall;
                        US1::US1_ClassKnown(v24)
                    } else {
                        let v26: US2 = US2::US2_TreeRead;
                        let v27: Rc<str> = class_word_7(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: US2 = US2::US2_TreeRead;
                            US1::US1_ClassKnown(v29)
                        } else {
                            let v31: US2 = US2::US2_Bench;
                            let v32: Rc<str> = class_word_7(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US2 = US2::US2_Bench;
                                US1::US1_ClassKnown(v34)
                            } else {
                                let mut v36: US2 = US2::US2_CompileHold;
                                let v37: Rc<str> = class_word_7(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US2 = US2::US2_CompileHold;
                                    US1::US1_ClassKnown(v39)
                                } else {
                                    let v41: US2 = US2::US2_TreeWrite;
                                    let mut v42: Rc<str> = class_word_7(v41);
                                    let v43: bool = &*v0 == &*v42;
                                    if v43 {
                                        let v44: US2 = US2::US2_TreeWrite;
                                        US1::US1_ClassKnown(v44)
                                    } else {
                                        US1::US1_ClassUnknown
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
fn local_offset_minutes_8() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn method10(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method13(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method13(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method12(v16.clone(), v17.clone())
    }
}
fn clock_start_14(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: i64) -> u64 {
    let v4: bool = v3 < 0i64;
    let mut v6: i64 = if v4 {
        let v5: i64 = v3.wrapping_neg();
        v5
    } else {
        v3
    };
    let v7: u64 = (v6 as u64);
    let v8: u64 = v7.wrapping_mul(60000u64);
    let v11: u64 = if v4 {
        let mut v9: u64 = v2.wrapping_sub(v8);
        v9
    } else {
        let v10: u64 = v2.wrapping_add(v8);
        v10
    };
    let v12: u64 = v11.wrapping_div(86400000u64);
    let v13: u64 = v12.wrapping_mul(86400000u64);
    let mut v14: u64 = v0.wrapping_mul(60u64);
    let mut v15: u64 = v14.wrapping_add(v1);
    let v16: u64 = v15.wrapping_mul(60000u64);
    let mut v17: u64 = v13.wrapping_add(v16);
    let v20: u64 = if v4 {
        let mut v18: u64 = v17.wrapping_add(v8);
        v18
    } else {
        let v19: u64 = v17.wrapping_sub(v8);
        v19
    };
    let mut v21: bool = v20 < v2;
    if v21 {
        let v22: u64 = v20.wrapping_add(86400000u64);
        v22
    } else {
        v20
    }
}
fn start_of_9(mut v0: Rc<str>, mut v1: u64, mut v2: i64) -> US3 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start must be HH:MM or +minutes, got '"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v0));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+"); } LIT.with(|lit| lit.clone()) };
    let v8: bool = v0.starts_with(&*v7);
    let v26: u64 = if v8 {
        let mut v9: i32 = (v0.clone().len() as i32);
        let v10: i32 = v9.wrapping_sub(1i32);
        let v11: usize = ((1i32) as usize);
        let v12: i32 = 1i32.wrapping_add(v10);
        let v13: usize = ((v12) as usize);
        let mut v14: Rc<str> = Rc::<str>::from(&(v0)[v11..v13]);
        let mut v15: Option<u64> = (v14).trim().parse::<u64>().ok();
        let v16: Option<u64> = method10(v15.clone());
        let mut v17: Rc<dyn Fn((u64)) -> US4> = closure1();
        let mut v18: Option<US4> = v16.map(|x| v17(x));
        let v20: US4 = US4::US4_None;
        let mut v21: US4 = v20.clone();
        let v22: US4 = v18.unwrap_or(v21);
        match &v22 {
            US4::US4_None => {
                99999u64
            }
            US4::US4_Some(v23) => {
                let v23: u64 = *v23;
                v23
            }
        }
    } else {
        99999u64
    };
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<UH0> = split_list_11(v27.clone(), v0.clone());
    match &*v28 {
        UH0::UH0_Cons(v29, v30) => {
            let v29: Rc<str> = v29.clone();
            let v30: Rc<UH0> = v30.clone();
            match &*v30 {
                UH0::UH0_Cons(v31, v32) => {
                    let v31: Rc<str> = v31.clone();
                    let v32: Rc<UH0> = v32.clone();
                    match &*v32 {
                        UH0::UH0_Nil => {
                            let v33: Option<u64> = (v29).trim().parse::<u64>().ok();
                            let v34: Option<u64> = method10(v33.clone());
                            let mut v35: Rc<dyn Fn((u64)) -> US4> = closure1();
                            let mut v36: Option<US4> = v34.map(|x| v35(x));
                            let v38: US4 = US4::US4_None;
                            let v39: US4 = v38.clone();
                            let v40: US4 = v36.unwrap_or(v39);
                            let v43: u64 = match &v40 {
                                US4::US4_None => {
                                    99999u64
                                }
                                US4::US4_Some(v41) => {
                                    let v41: u64 = *v41;
                                    v41
                                }
                            };
                            let v44: Option<u64> = (v31).trim().parse::<u64>().ok();
                            let v45: Option<u64> = method10(v44.clone());
                            let mut v46: Option<US4> = v45.map(|x| v35(x));
                            let v48: US4 = US4::US4_None;
                            let v49: US4 = v48.clone();
                            let v50: US4 = v46.unwrap_or(v49);
                            let v53: u64 = match &v50 {
                                US4::US4_None => {
                                    99999u64
                                }
                                US4::US4_Some(v51) => {
                                    let v51: u64 = *v51;
                                    v51
                                }
                            };
                            let v54: bool = v43 > 23u64;
                            let v56: bool = if v54 {
                                true
                            } else {
                                let v55: bool = v53 > 59u64;
                                v55
                            };
                            if v56 {
                                US3::US3_StartRefused(v6.clone())
                            } else {
                                let v58: u64 = clock_start_14(v43, v53, v1, v2);
                                US3::US3_StartAt(v58)
                            }
                        }
                        _ => {
                            let v61: bool = v26 < 99999u64;
                            if v61 {
                                let v62: u64 = v26.wrapping_mul(60000u64);
                                let v63: u64 = v1.wrapping_add(v62);
                                US3::US3_StartAt(v63)
                            } else {
                                US3::US3_StartRefused(v6.clone())
                            }
                        }
                    }
                }
                _ => {
                    let v68: bool = v26 < 99999u64;
                    if v68 {
                        let v69: u64 = v26.wrapping_mul(60000u64);
                        let v70: u64 = v1.wrapping_add(v69);
                        US3::US3_StartAt(v70)
                    } else {
                        US3::US3_StartRefused(v6.clone())
                    }
                }
            }
        }
        _ => {
            let v75: bool = v26 < 99999u64;
            if v75 {
                let v76: u64 = v26.wrapping_mul(60000u64);
                let v77: u64 = v1.wrapping_add(v76);
                US3::US3_StartAt(v77)
            } else {
                US3::US3_StartRefused(v6.clone())
            }
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: bool) -> (Rc<str>, bool) {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let v7: bool = if v3 {
                    let mut v6: bool = &*v2 == "";
                    v6
                } else {
                    false
                };
                let (mut v9, mut v10): (Rc<str>, bool) = if v7 {
                    (v4.clone(), false)
                } else {
                    let v8: bool = &*v4 == &*v0;
                    (v2.clone(), v8)
                };
                (v1, v2, v3) = (v5.clone(), v9.clone(), v10);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2.clone(), v3);
            }
        }
    }
}
fn option_after_15(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: bool = false;
    let (mut v4, mut v5): (Rc<str>, bool) = method16(v0, v1, v2, v3);
    v4
}
fn announced_path_17(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/announced.spi"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    v5
}
fn file_exists_18(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method19(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method21(v3, v1.clone());
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl announced_"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            if v6 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method22(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method24(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_23(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_25(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn appended_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US2, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: bool = &*v0 == "";
    let v8: Rc<str> = if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion announced_row =\n    | Announced :: string * string * u64 * u64 * u64 -> announced_row\n"); } LIT.with(|lit| lit.clone()) };
        v7
    } else {
        v0
    };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<UH0> = split_list_11(v9.clone(), v8.clone());
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH0> = method21(v10, v11);
    let v13: i32 = 0i32;
    let mut v14: i32 = method22(v12, v13);
    let mut v15: bool = v8.ends_with(&*v9);
    let mut v17: Rc<str> = if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v16
    } else {
        v9.clone()
    };
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v17));
    let v19: i32 = v14.wrapping_add(1i32);
    let v20: Rc<str> = format_real_23(v19);
    let mut v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl announced_"); } LIT.with(|lit| lit.clone()) };
    v21.borrow_mut().push(v22);
    v21.borrow_mut().push(v20);
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Announced (\""); } LIT.with(|lit| lit.clone()) };
    v21.borrow_mut().push(v23);
    v21.borrow_mut().push(v1);
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    v21.borrow_mut().push(v24);
    let v25: Rc<Vec<Rc<str>>> = Rc::new(v21.borrow().clone());
    let v26: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v25).as_ref().clone()));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = Rc::<str>::from((*v26.borrow()).join(&*v27));
    let v29: Rc<str> = format_real_25(v3);
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
    let v32: Rc<str> = format_real_25(v4);
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v30));
    let v34: Rc<str> = format_real_25(v5);
    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v30));
    let mut v36: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v36.borrow_mut().push(v31);
    v36.borrow_mut().push(v33);
    v36.borrow_mut().push(v35);
    let v37: Rc<Vec<Rc<str>>> = Rc::new(v36.borrow().clone());
    let v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v37).as_ref().clone()));
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = Rc::<str>::from((*v38.borrow()).join(&*v39));
    let v41: Rc<str> = class_word_7(v2);
    let mut v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v42.borrow_mut().push(v28);
    v42.borrow_mut().push(v41);
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
    v42.borrow_mut().push(v43);
    v42.borrow_mut().push(v40);
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    v42.borrow_mut().push(v44);
    let v45: Rc<Vec<Rc<str>>> = Rc::new(v42.borrow().clone());
    let mut v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v45).as_ref().clone()));
    let v47: Rc<str> = Rc::<str>::from((*v46.borrow()).join(&*v27));
    let v49: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v47)), v9));
    v49
}
fn minutes_left_26(mut v0: u64, mut v1: u64) -> u64 {
    let v2: bool = v0 <= v1;
    if v2 {
        0u64
    } else {
        let v3: u64 = v0.wrapping_sub(v1);
        let v4: u64 = v3.wrapping_add(59999u64);
        let v5: u64 = v4.wrapping_div(60000u64);
        v5
    }
}
fn announce_run_0(mut v0: Rc<str>) -> i32 {
    let v1: Rc<UH0> = verb_words_2();
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4.clone();
                    let v5: Rc<UH0> = v5.clone();
                    match &*v5 {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6.clone();
                            let v7: Rc<UH0> = v7.clone();
                            let v8: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                            let mut v9: US1 = class_of_6(v2.clone());
                            let v10: i64 = local_offset_minutes_8();
                            let v11: US3 = start_of_9(v4.clone(), v8, v10);
                            let v12: Option<u64> = (v6).trim().parse::<u64>().ok();
                            let v13: Option<u64> = method10(v12.clone());
                            let mut v14: Rc<dyn Fn((u64)) -> US4> = closure1();
                            let mut v15: Option<US4> = v13.map(|x| v14(x));
                            let mut v17: US4 = US4::US4_None;
                            let mut v18: US4 = v17.clone();
                            let v19: US4 = v15.unwrap_or(v18);
                            match &v9 {
                                US1::US1_ClassUnknown => {
                                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown class '"); } LIT.with(|lit| lit.clone()) };
                                    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v2));
                                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                                    refused_1(v23.clone())
                                }
                                _ => {
                                    match &v11 {
                                        US3::US3_StartRefused(v25) => {
                                            let v25: Rc<str> = v25.clone();
                                            refused_1(v25.clone())
                                        }
                                        _ => {
                                            match &v9 {
                                                US1::US1_ClassKnown(v27) => {
                                                    let v27: US2 = v27.clone();
                                                    match &v11 {
                                                        US3::US3_StartAt(v28) => {
                                                            let v28: u64 = *v28;
                                                            match &v19 {
                                                                US4::US4_None => {
                                                                    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("minutes must be a whole number, got '"); } LIT.with(|lit| lit.clone()) };
                                                                    let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v6));
                                                                    let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                                                    let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
                                                                    refused_1(v92.clone())
                                                                }
                                                                US4::US4_Some(v29) => {
                                                                    let v29: u64 = *v29;
                                                                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--label"); } LIT.with(|lit| lit.clone()) };
                                                                    let v31: Rc<str> = option_after_15(v30.clone(), v7.clone());
                                                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--work"); } LIT.with(|lit| lit.clone()) };
                                                                    let v33: Rc<str> = option_after_15(v32.clone(), v7.clone());
                                                                    let v34: Option<u64> = (v33).trim().parse::<u64>().ok();
                                                                    let mut v35: Option<u64> = method10(v34.clone());
                                                                    let mut v36: Option<US4> = v35.map(|x| v14(x));
                                                                    let v38: US4 = US4::US4_None;
                                                                    let v39: US4 = v38.clone();
                                                                    let v40: US4 = v36.unwrap_or(v39);
                                                                    let v43: u64 = match &v40 {
                                                                        US4::US4_None => {
                                                                            v29
                                                                        }
                                                                        US4::US4_Some(v41) => {
                                                                            let v41: u64 = *v41;
                                                                            v41
                                                                        }
                                                                    };
                                                                    let v44: bool = &*v31 == "";
                                                                    let mut v46: Rc<str> = if v44 {
                                                                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unnamed"); } LIT.with(|lit| lit.clone()) };
                                                                        v45.clone()
                                                                    } else {
                                                                        v31.clone()
                                                                    };
                                                                    let v47: u64 = v29.wrapping_mul(60000u64);
                                                                    let v48: u64 = v28.wrapping_add(v47);
                                                                    let v49: Rc<str> = announced_path_17(v0.clone());
                                                                    let v50: bool = file_exists_18(v49.clone());
                                                                    let v63: Rc<str> = if v50 {
                                                                        let v52: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v49);
                                                                        let v54: Vec<u8> = v52.unwrap();
                                                                        let v55: Vec<u8> = method19(v54.clone());
                                                                        let v57: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v55);
                                                                        let v59: std::string::String = v57.unwrap();
                                                                        let v61: Rc<str> = Rc::<str>::from(String::as_str(&v59));
                                                                        v61.clone()
                                                                    } else {
                                                                        let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                        v62.clone()
                                                                    };
                                                                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                                                    let v66: Rc<str> = Rc::<str>::from(v0.replace(&*v64, &*v65));
                                                                    let v68: Result<(), std::io::Error> = std::fs::create_dir_all(&*v66);
                                                                    let v69: Rc<str> = appended_20(v63.clone(), v46.clone(), v27.clone(), v28, v48, v43);
                                                                    std::fs::write(&*v49, &*v69).unwrap();
                                                                    let v71: Rc<str> = class_word_7(v27.clone());
                                                                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window announce ok class="); } LIT.with(|lit| lit.clone()) };
                                                                    let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v71));
                                                                    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" holder="); } LIT.with(|lit| lit.clone()) };
                                                                    let v76: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v73, v74)), v46));
                                                                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" starts_in_min="); } LIT.with(|lit| lit.clone()) };
                                                                    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
                                                                    let v79: u64 = minutes_left_26(v28, v8);
                                                                    let v80: Rc<str> = format_real_25(v79);
                                                                    let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v80));
                                                                    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" minutes="); } LIT.with(|lit| lit.clone()) };
                                                                    let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                                                                    let v84: Rc<str> = format_real_25(v29);
                                                                    let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v84));
                                                                    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
                                                                    let v88: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v85, v86)), v49));
                                                                    println!("{}", v88);
                                                                    0i32
                                                                }
                                                            }
                                                        }
                                                        _ => unreachable!(),
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window announce <class> <HH:MM|+minutes> <minutes> [--work <minutes>] [--label <holder>]"); } LIT.with(|lit| lit.clone()) };
                            refused_1(v100.clone())
                        }
                    }
                }
                _ => {
                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window announce <class> <HH:MM|+minutes> <minutes> [--work <minutes>] [--label <holder>]"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v103.clone())
                }
            }
        }
        _ => {
            let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window announce <class> <HH:MM|+minutes> <minutes> [--work <minutes>] [--label <holder>]"); } LIT.with(|lit| lit.clone()) };
            refused_1(v106.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        announce_run_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_window_announce_run(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
