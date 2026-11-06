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
    US0_LiveGui,
    US0_TestSuite,
    US0_HeavyBuild,
    US0_Compile,
    US0_CompilerInstall,
    US0_TreeRead,
    US0_Bench,
    US0_CompileHold,
    US0_TreeWrite,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, US0, u64, u64, u64, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_StartNow,
    US1_WouldOverlap(Rc<str>, u64),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_ClassKnown(US0),
    US2_ClassUnknown,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US3 {
    US3_AnnouncedLine(Rc<str>, US0, u64, u64, u64),
    US3_OtherAnnouncedLine,
}
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
fn compiles_4(mut v0: US0) -> bool {
    match v0 {
        US0::US0_Compile => {
            true
        }
        _ => {
            false
        }
    }
}
fn lease_only_5(mut v0: US0) -> bool {
    match v0 {
        US0::US0_TreeRead => {
            true
        }
        US0::US0_TreeWrite => {
            true
        }
        _ => {
            false
        }
    }
}
fn exclusive_6(mut v0: US0) -> bool {
    match v0 {
        US0::US0_CompilerInstall => {
            true
        }
        US0::US0_LiveGui => {
            true
        }
        _ => {
            false
        }
    }
}
fn quiet_7(mut v0: US0) -> bool {
    match v0 {
        US0::US0_Bench => {
            true
        }
        _ => {
            false
        }
    }
}
fn loud_8(mut v0: US0) -> bool {
    match v0 {
        US0::US0_Bench => {
            true
        }
        US0::US0_HeavyBuild => {
            true
        }
        US0::US0_TestSuite => {
            true
        }
        _ => {
            false
        }
    }
}
fn holds_compiles_9(mut v0: US0) -> bool {
    match v0 {
        US0::US0_CompileHold => {
            true
        }
        _ => {
            false
        }
    }
}
fn clashes_3(mut v0: US0, mut v1: US0) -> bool {
    let mut v2: bool = lease_only_5(v0.clone());
    let v4: bool = if v2 {
        true
    } else {
        lease_only_5(v1.clone())
    };
    if v4 {
        false
    } else {
        let v5: bool = exclusive_6(v0.clone());
        let v7: bool = if v5 {
            true
        } else {
            exclusive_6(v1.clone())
        };
        if v7 {
            true
        } else {
            let v8: bool = quiet_7(v0.clone());
            let v10: bool = if v8 {
                loud_8(v1.clone())
            } else {
                false
            };
            let v14: bool = if v10 {
                true
            } else {
                let v11: bool = quiet_7(v1.clone());
                if v11 {
                    loud_8(v0.clone())
                } else {
                    false
                }
            };
            let v18: bool = if v14 {
                true
            } else {
                let v15: bool = holds_compiles_9(v0.clone());
                if v15 {
                    compiles_4(v1.clone())
                } else {
                    false
                }
            };
            if v18 {
                true
            } else {
                let v19: bool = holds_compiles_9(v1);
                if v19 {
                    compiles_4(v0)
                } else {
                    false
                }
            }
        }
    }
}
fn minutes_left_10(mut v0: u64, mut v1: u64) -> u64 {
    let mut v2: bool = v0 <= v1;
    if v2 {
        0u64
    } else {
        let v3: u64 = v0.wrapping_sub(v1);
        let v4: u64 = v3.wrapping_add(59999u64);
        let v5: u64 = v4.wrapping_div(60000u64);
        v5
    }
}
fn method2(mut v0: US0, mut v1: u64, mut v2: u64, mut v3: Rc<UH0>, mut v4: US1) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v5, v6, v7, v8, v9, v10) => {
                let v5: Rc<str> = v5;
                let mut v6: US0 = v6;
                let v7: u64 = v7;
                let v8: u64 = v8;
                let mut v9: u64 = v9;
                let v10: Rc<UH0> = v10;
                let v25: US1 = match &v4 {
                    US1::US1_StartNow => {
                        let v13: u64 = v2.wrapping_mul(60000u64);
                        let v14: u64 = v1.wrapping_add(v13);
                        let v15: bool = clashes_3(v6, v0.clone());
                        let v17: bool = if v15 {
                            let v16: bool = v8 > v1;
                            v16
                        } else {
                            false
                        };
                        let v19: bool = if v17 {
                            let v18: bool = v7 < v14;
                            v18
                        } else {
                            false
                        };
                        if v19 {
                            let v20: u64 = minutes_left_10(v7, v1);
                            US1::US1_WouldOverlap(v5, v20)
                        } else {
                            US1::US1_StartNow
                        }
                    }
                    US1::US1_WouldOverlap(v11, v12) => {
                        let v11: Rc<str> = v11.clone();
                        let v12: u64 = *v12;
                        v4
                    }
                };
                (v3, v4) = (v10, v25);
                continue;
            }
            UH0::UH0_Nil => {
                return v4;
            }
        }
    }
}
fn method12(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_11(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method12(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn refusal_of_blocks_1(mut v0: US0, mut v1: u64, mut v2: Rc<UH0>, mut v3: u64) -> Rc<str> {
    let v4: US1 = US1::US1_StartNow;
    let v5: US1 = method2(v0, v3, v1, v2, v4);
    match v5 {
        US1::US1_StartNow => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US1::US1_WouldOverlap(v7, v8) => {
            let v7: Rc<str> = v7;
            let v8: u64 = v8;
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("would overlap a block announced by "); } LIT.with(|lit| lit.clone()) }, v7));
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" that starts in "); } LIT.with(|lit| lit.clone()) }));
            let v11: Rc<str> = format_real_11(v8);
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min"); } LIT.with(|lit| lit.clone()) }));
            v13
        }
    }
}
fn class_word_14(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US0::US0_Compile => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US0::US0_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US0::US0_HeavyBuild => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_TestSuite => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US0::US0_TreeWrite => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-write"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn class_of_13(mut v0: Rc<str>) -> US2 {
    let mut v1: US0 = US0::US0_LiveGui;
    let mut v2: Rc<str> = class_word_14(v1);
    let v3: bool = &*v0 == &*v2;
    if v3 {
        let v4: US0 = US0::US0_LiveGui;
        US2::US2_ClassKnown(v4)
    } else {
        let mut v6: US0 = US0::US0_TestSuite;
        let v7: Rc<str> = class_word_14(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let mut v9: US0 = US0::US0_TestSuite;
            US2::US2_ClassKnown(v9)
        } else {
            let v11: US0 = US0::US0_HeavyBuild;
            let v12: Rc<str> = class_word_14(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US0 = US0::US0_HeavyBuild;
                US2::US2_ClassKnown(v14)
            } else {
                let v16: US0 = US0::US0_Compile;
                let v17: Rc<str> = class_word_14(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US0 = US0::US0_Compile;
                    US2::US2_ClassKnown(v19)
                } else {
                    let v21: US0 = US0::US0_CompilerInstall;
                    let mut v22: Rc<str> = class_word_14(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US0 = US0::US0_CompilerInstall;
                        US2::US2_ClassKnown(v24)
                    } else {
                        let v26: US0 = US0::US0_TreeRead;
                        let v27: Rc<str> = class_word_14(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: US0 = US0::US0_TreeRead;
                            US2::US2_ClassKnown(v29)
                        } else {
                            let v31: US0 = US0::US0_Bench;
                            let v32: Rc<str> = class_word_14(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US0 = US0::US0_Bench;
                                US2::US2_ClassKnown(v34)
                            } else {
                                let v36: US0 = US0::US0_CompileHold;
                                let v37: Rc<str> = class_word_14(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US0 = US0::US0_CompileHold;
                                    US2::US2_ClassKnown(v39)
                                } else {
                                    let v41: US0 = US0::US0_TreeWrite;
                                    let v42: Rc<str> = class_word_14(v41);
                                    let v43: bool = &*v0 == &*v42;
                                    if v43 {
                                        let mut v44: US0 = US0::US0_TreeWrite;
                                        US2::US2_ClassKnown(v44)
                                    } else {
                                        US2::US2_ClassUnknown
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
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6, v7) => {
                let mut v2: Rc<str> = v2;
                let v3: US0 = v3;
                let v4: u64 = v4;
                let v5: u64 = v5;
                let mut v6: u64 = v6;
                let v7: Rc<UH0> = v7;
                let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v6, v1));
                (v0, v1) = (v7, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn file_exists_17(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn method18(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method20(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method21(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method21(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method20(v16, v17)
    }
}
fn method24(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method25(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method23(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method23(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let v7: i32 = method24(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2.clone(), v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method25(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            let v13: bool = &*v12 != "";
            if v13 {
                Rc::new(UH1::UH1_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method26(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let mut v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method22(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl announced_"); } LIT.with(|lit| lit.clone()) };
                let v7: bool = v2.starts_with(&*v6);
                let v8: bool = v7 == false;
                let v11: bool = if v8 {
                    true
                } else {
                    let mut v9: i32 = (v5.clone().borrow().len() as i32);
                    let v10: bool = v9 < 5i32;
                    v10
                };
                let v104: US3 = if v11 {
                    US3::US3_OtherAnnouncedLine
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v14: Rc<str> = v5.borrow()[4i32 as usize].clone();
                    let v15: Rc<UH1> = split_list_19(v13, v14);
                    let v16: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v17: Rc<UH1> = method23(v15, v16);
                    let v18: Rc<str> = v5.borrow()[3i32 as usize].clone();
                    let v19: US2 = class_of_13(v18);
                    match v19 {
                        US2::US2_ClassKnown(v21) => {
                            let v21: US0 = v21;
                            match Rc::unwrap_or_clone(v17) {
                                UH1::UH1_Cons(v22, v23) => {
                                    let mut v22: Rc<str> = v22;
                                    let v23: Rc<UH1> = v23;
                                    match Rc::unwrap_or_clone(v23) {
                                        UH1::UH1_Cons(v24, v25) => {
                                            let v24: Rc<str> = v24;
                                            let v25: Rc<UH1> = v25;
                                            match Rc::unwrap_or_clone(v25) {
                                                UH1::UH1_Cons(v26, v27) => {
                                                    let v26: Rc<str> = v26;
                                                    let v27: Rc<UH1> = v27;
                                                    match Rc::unwrap_or_clone(v27) {
                                                        UH1::UH1_Nil => {
                                                            let v28: Rc<str> = v5.borrow()[1i32 as usize].clone();
                                                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                                                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                            let v31: Rc<str> = Rc::<str>::from(v22.replace(&*v29, &*v30));
                                                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
                                                            let v33: Rc<str> = Rc::<str>::from(v31.replace(&*v32, &*v30));
                                                            let v34: i32 = (v33.clone().len() as i32);
                                                            let v35: i32 = 0i32;
                                                            let v36: i32 = method24(v33.clone(), v34, v35);
                                                            let v37: i32 = v34.wrapping_sub(1i32);
                                                            let v38: Rc<str> = string_slice(&v33, v36 as i64, v37 as i64);
                                                            let v39: i32 = (v38.clone().len() as i32);
                                                            let v40: i32 = method25(v38.clone(), v39);
                                                            let v41: Rc<str> = string_slice(&v38, 0i32 as i64, v40 as i64);
                                                            let v42: Option<u64> = (v41).trim().parse::<u64>().ok();
                                                            let v43: Option<u64> = method26(v42);
                                                            let mut v44: Rc<dyn Fn((u64)) -> US4> = closure1();
                                                            let mut v45: Option<US4> = v43.map(|x| v44(x));
                                                            let v47: US4 = US4::US4_None;
                                                            let v48: US4 = v47.clone();
                                                            let v49: US4 = v45.unwrap_or(v48);
                                                            let v52: u64 = match v49 {
                                                                US4::US4_None => {
                                                                    0u64
                                                                }
                                                                US4::US4_Some(v50) => {
                                                                    let v50: u64 = v50;
                                                                    v50
                                                                }
                                                            };
                                                            let v53: Rc<str> = Rc::<str>::from(v24.replace(&*v29, &*v30));
                                                            let v54: Rc<str> = Rc::<str>::from(v53.replace(&*v32, &*v30));
                                                            let v55: i32 = (v54.clone().len() as i32);
                                                            let v56: i32 = 0i32;
                                                            let v57: i32 = method24(v54.clone(), v55, v56);
                                                            let v58: i32 = v55.wrapping_sub(1i32);
                                                            let v59: Rc<str> = string_slice(&v54, v57 as i64, v58 as i64);
                                                            let v60: i32 = (v59.clone().len() as i32);
                                                            let v61: i32 = method25(v59.clone(), v60);
                                                            let v62: Rc<str> = string_slice(&v59, 0i32 as i64, v61 as i64);
                                                            let v63: Option<u64> = (v62).trim().parse::<u64>().ok();
                                                            let v64: Option<u64> = method26(v63);
                                                            let mut v65: Option<US4> = v64.map(|x| v44(x));
                                                            let v67: US4 = US4::US4_None;
                                                            let v68: US4 = v67.clone();
                                                            let v69: US4 = v65.unwrap_or(v68);
                                                            let v72: u64 = match v69 {
                                                                US4::US4_None => {
                                                                    0u64
                                                                }
                                                                US4::US4_Some(v70) => {
                                                                    let v70: u64 = v70;
                                                                    v70
                                                                }
                                                            };
                                                            let v73: Rc<str> = Rc::<str>::from(v26.replace(&*v29, &*v30));
                                                            let v74: Rc<str> = Rc::<str>::from(v73.replace(&*v32, &*v30));
                                                            let v75: i32 = (v74.clone().len() as i32);
                                                            let v76: i32 = 0i32;
                                                            let v77: i32 = method24(v74.clone(), v75, v76);
                                                            let v78: i32 = v75.wrapping_sub(1i32);
                                                            let v79: Rc<str> = string_slice(&v74, v77 as i64, v78 as i64);
                                                            let v80: i32 = (v79.clone().len() as i32);
                                                            let v81: i32 = method25(v79.clone(), v80);
                                                            let v82: Rc<str> = string_slice(&v79, 0i32 as i64, v81 as i64);
                                                            let v83: Option<u64> = (v82).trim().parse::<u64>().ok();
                                                            let v84: Option<u64> = method26(v83);
                                                            let mut v85: Option<US4> = v84.map(|x| v44(x));
                                                            let v87: US4 = US4::US4_None;
                                                            let v88: US4 = v87.clone();
                                                            let v89: US4 = v85.unwrap_or(v88);
                                                            let v92: u64 = match v89 {
                                                                US4::US4_None => {
                                                                    0u64
                                                                }
                                                                US4::US4_Some(v90) => {
                                                                    let v90: u64 = v90;
                                                                    v90
                                                                }
                                                            };
                                                            US3::US3_AnnouncedLine(v28, v21, v52, v72, v92)
                                                        }
                                                        _ => {
                                                            US3::US3_OtherAnnouncedLine
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    US3::US3_OtherAnnouncedLine
                                                }
                                            }
                                        }
                                        _ => {
                                            US3::US3_OtherAnnouncedLine
                                        }
                                    }
                                }
                                _ => {
                                    US3::US3_OtherAnnouncedLine
                                }
                            }
                        }
                        US2::US2_ClassUnknown => {
                            US3::US3_OtherAnnouncedLine
                        }
                    }
                };
                let v112: Rc<UH0> = match v104 {
                    US3::US3_AnnouncedLine(v105, v106, v107, v108, v109) => {
                        let v105: Rc<str> = v105;
                        let v106: US0 = v106;
                        let v107: u64 = v107;
                        let v108: u64 = v108;
                        let v109: u64 = v109;
                        Rc::new(UH0::UH0_Cons(v105, v106, v107, v108, v109, v1))
                    }
                    US3::US3_OtherAnnouncedLine => {
                        v1
                    }
                };
                (v0, v1) = (v3, v112);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn blocks_at_15(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/announced.spi"); } LIT.with(|lit| lit.clone()) }));
    let v5: bool = file_exists_17(v4.clone());
    if v5 {
        let v7: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v4);
        let mut v9: Vec<u8> = v7.unwrap();
        let v10: Vec<u8> = method18(v9);
        let v12: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v10);
        let v14: std::string::String = v12.unwrap();
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(v16.replace(&*v18, &*v19));
        let v21: Rc<UH1> = split_list_19(v17, v20);
        let mut v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v23: Rc<UH0> = method22(v21, v22);
        let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method16(v23, v24)
    } else {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    }
}
fn announced_refusal_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> Rc<str> {
    let v3: US2 = class_of_13(v1);
    match v3 {
        US2::US2_ClassKnown(v5) => {
            let v5: US0 = v5;
            let mut v6: Rc<UH0> = blocks_at_15(v0);
            let v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
            refusal_of_blocks_1(v5, v2, v6, v7)
        }
        US2::US2_ClassUnknown => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64| -> Rc<str> {
        announced_refusal_0(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method29(mut v0: u64, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6, v7, v8) => {
            let v3: Rc<str> = v3;
            let v4: US0 = v4;
            let v5: u64 = v5;
            let mut v6: u64 = v6;
            let v7: u64 = v7;
            let v8: Rc<UH0> = v8;
            let mut v9: Rc<UH0> = method29(v0, v8, v2);
            let v10: bool = v6 > v0;
            if v10 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v6, v7, v9))
            } else {
                v9
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method30(mut v0: u64, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6, v7, v8) => {
            let v3: Rc<str> = v3;
            let v4: US0 = v4;
            let v5: u64 = v5;
            let mut v6: u64 = v6;
            let v7: u64 = v7;
            let v8: Rc<UH0> = v8;
            let mut v9: Rc<UH1> = method30(v0, v8, v2);
            let v10: Rc<str> = class_word_14(v4);
            let v11: u64 = minutes_left_10(v5, v0);
            let v12: Rc<str> = format_real_11(v11);
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("starts_in="); } LIT.with(|lit| lit.clone()) }, v12));
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
            let v15: u64 = v6.wrapping_sub(v5);
            let v16: u64 = v15.wrapping_div(60000u64);
            let v17: Rc<str> = format_real_11(v16);
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("minutes="); } LIT.with(|lit| lit.clone()) }, v17));
            let v19: Rc<str> = format_real_11(v7);
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("work="); } LIT.with(|lit| lit.clone()) }, v19));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
            let mut v22: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("announced"); } LIT.with(|lit| lit.clone()) };
            v22.borrow_mut().push(v23);
            v22.borrow_mut().push(v10);
            v22.borrow_mut().push(v3);
            v22.borrow_mut().push(v14);
            v22.borrow_mut().push(v18);
            v22.borrow_mut().push(v21);
            let v24: Rc<Vec<Rc<str>>> = Rc::new(v22.borrow().clone());
            let v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v24).as_ref().clone()));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from((*v25.borrow()).join(&*v26));
            Rc::new(UH1::UH1_Cons(v27, v9))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method31(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn upcoming_lines_28(mut v0: Rc<UH0>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = method29(v1, v0, v2);
    let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH1> = method30(v1, v3, v4);
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v7: i32 = 0i32;
    let v8: i32 = method31(v6.clone(), v5, v7);
    let mut v9: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v9).as_ref().clone()));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from((*v10.borrow()).join(&*v11));
    v12
}
fn announced_lines_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<UH0> = blocks_at_15(v0);
    let mut v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    upcoming_lines_28(v1, v2)
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        announced_lines_27(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_window_announced_refusal(v0: &str, v1: &str, v2: u64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2)
}
pub fn kit_window_announced_lines(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
pub type SpiralShape_accbf6de6842 = Mut1;
