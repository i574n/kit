#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_LiveGui,
    US1_TestSuite,
    US1_HeavyBuild,
    US1_Compile,
    US1_CompilerInstall,
    US1_TreeRead,
    US1_Bench,
    US1_CompileHold,
}
#[derive(Clone)]
enum US2 {
    US2_Held,
    US2_Waiting,
}
#[derive(Clone)]
enum US0 {
    US0_RowKnown(Rc<str>, US1, US2, Rc<str>, u64, u64),
    US0_RowUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_ClassKnown(US1),
    US3_ClassUnknown,
}
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Calm,
    US5_Crowded(u64, u64, u64),
}
struct Mut0 { l0: Rc<str> }
fn free_mb_0() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("KIT_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn other_rows_3(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn first_row_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn tab_field_7(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn class_word_9(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US1::US1_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US1::US1_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US1::US1_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US1::US1_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US1::US1_LiveGui => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US1::US1_TestSuite => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US1::US1_TreeRead => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn class_of_8(mut v0: Rc<str>) -> US3 {
    let v1: US1 = US1::US1_LiveGui;
    let v2: Rc<str> = class_word_9(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US1 = US1::US1_LiveGui;
        US3::US3_ClassKnown(v4)
    } else {
        let v6: US1 = US1::US1_TestSuite;
        let v7: Rc<str> = class_word_9(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: US1 = US1::US1_TestSuite;
            US3::US3_ClassKnown(v9)
        } else {
            let v11: US1 = US1::US1_HeavyBuild;
            let v12: Rc<str> = class_word_9(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US1 = US1::US1_HeavyBuild;
                US3::US3_ClassKnown(v14)
            } else {
                let v16: US1 = US1::US1_Compile;
                let v17: Rc<str> = class_word_9(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US1 = US1::US1_Compile;
                    US3::US3_ClassKnown(v19)
                } else {
                    let v21: US1 = US1::US1_CompilerInstall;
                    let v22: Rc<str> = class_word_9(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US1 = US1::US1_CompilerInstall;
                        US3::US3_ClassKnown(v24)
                    } else {
                        let v26: US1 = US1::US1_TreeRead;
                        let v27: Rc<str> = class_word_9(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: US1 = US1::US1_TreeRead;
                            US3::US3_ClassKnown(v29)
                        } else {
                            let v31: US1 = US1::US1_Bench;
                            let v32: Rc<str> = class_word_9(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US1 = US1::US1_Bench;
                                US3::US3_ClassKnown(v34)
                            } else {
                                let v36: US1 = US1::US1_CompileHold;
                                let v37: Rc<str> = class_word_9(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US1 = US1::US1_CompileHold;
                                    US3::US3_ClassKnown(v39)
                                } else {
                                    US3::US3_ClassUnknown
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method11(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_10(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method11(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US4> = closure1();
    let mut v4: Option<US4> = v2.map(|x| v3(x));
    let v6: US4 = US4::US4_None;
    let v7: US4 = v6.clone();
    let v8: US4 = v4.unwrap_or(v7);
    match &v8 {
        US4::US4_None => {
            0u64
        }
        US4::US4_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn row_of_6(mut v0: Rc<str>) -> US0 {
    let v1: i32 = 1i32;
    let v2: Rc<str> = tab_field_7(v0.clone(), v1);
    let mut v3: US3 = class_of_8(v2);
    match &v3 {
        US3::US3_ClassKnown(v5) => {
            let v5: US1 = v5.clone();
            let v6: i32 = 2i32;
            let v7: Rc<str> = tab_field_7(v0.clone(), v6);
            let v8: bool = &*v7 == "held";
            let v11: US2 = if v8 {
                US2::US2_Held
            } else {
                US2::US2_Waiting
            };
            let v12: i32 = 0i32;
            let v13: Rc<str> = tab_field_7(v0.clone(), v12);
            let v14: i32 = 3i32;
            let v15: Rc<str> = tab_field_7(v0.clone(), v14);
            let v16: i32 = 4i32;
            let v17: Rc<str> = tab_field_7(v0.clone(), v16);
            let v18: u64 = decimal_of_10(v17);
            let v19: i32 = 5i32;
            let v20: Rc<str> = tab_field_7(v0, v19);
            let v21: u64 = decimal_of_10(v20);
            US0::US0_RowKnown(v13, v5, v11, v15, v18, v21)
        }
        US3::US3_ClassUnknown => {
            US0::US0_RowUnknown
        }
    }
}
fn needed_mb_12(mut v0: US1) -> u64 {
    match &v0 {
        US1::US1_Bench => {
            2048u64
        }
        US1::US1_Compile => {
            1024u64
        }
        US1::US1_CompileHold => {
            0u64
        }
        US1::US1_CompilerInstall => {
            2048u64
        }
        US1::US1_HeavyBuild => {
            1024u64
        }
        US1::US1_LiveGui => {
            4096u64
        }
        US1::US1_TestSuite => {
            3072u64
        }
        US1::US1_TreeRead => {
            0u64
        }
    }
}
fn crowd_with_5(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: Rc<str>, mut v4: u64) -> (u64, u64, u64) {
    let v5: US0 = row_of_6(v3);
    let (mut v27, mut v28, mut v29): (u64, u64, u64) = match &v5 {
        US0::US0_RowUnknown => {
            (v0, v1, v2)
        }
        _ => {
            let (mut v24, mut v25, mut v26): (u64, u64, u64) = match &v5 {
                US0::US0_RowKnown(v6, v7, v8, v9, v10, v11) => {
                    let v6: Rc<str> = v6.clone();
                    let v7: US1 = v7.clone();
                    let v8: US2 = v8.clone();
                    let v9: Rc<str> = v9.clone();
                    let v10: u64 = *v10;
                    let v11: u64 = *v11;
                    let v17: bool = match &v8 {
                        US2::US2_Held => {
                            let v12: bool = v4 >= v10;
                            if v12 {
                                let v13: u64 = v4.wrapping_sub(v10);
                                let v14: bool = v13 <= 180000u64;
                                v14
                            } else {
                                false
                            }
                        }
                        US2::US2_Waiting => {
                            false
                        }
                    };
                    if v17 {
                        let v18: u64 = v0.wrapping_add(1u64);
                        let v19: u64 = needed_mb_12(v7);
                        let v20: u64 = v1.wrapping_add(v19);
                        (v18, v20, v2)
                    } else {
                        (v0, v1, v2)
                    }
                }
                _ => unreachable!(),
            };
            (v24, v25, v26)
        }
    };
    (v27, v28, v29)
}
fn crowd_of_2(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64) -> (u64, u64, u64) {
    let v5: bool = &*v0 == "";
    let (mut v14, mut v15, mut v16): (u64, u64, u64) = if v5 {
        (v2, v3, v4)
    } else {
        let v6: Rc<str> = other_rows_3(v0.clone());
        let v7: Rc<str> = first_row_4(v0);
        let (mut v8, mut v9, mut v10): (u64, u64, u64) = crowd_with_5(v2, v3, v4, v7, v1);
        let (mut v11, mut v12, mut v13): (u64, u64, u64) = crowd_of_2(v6, v1, v8, v9, v10);
        (v11, v12, v13)
    };
    (v14, v15, v16)
}
fn method14(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_13(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn machine_crowding_1(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> Rc<str> {
    let mut v3: u64 = 0u64;
    let mut v4: u64 = 0u64;
    let (mut v5, mut v6, mut v7): (u64, u64, u64) = crowd_of_2(v0.clone(), v2, v3, v4, v1);
    let v8: bool = v5 >= 2u64;
    let v10: bool = if v8 {
        let v9: bool = v6 > v7;
        v9
    } else {
        false
    };
    let v13: US5 = if v10 {
        US5::US5_Crowded(v5, v6, v7)
    } else {
        US5::US5_Calm
    };
    match &v13 {
        US5::US5_Calm => {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US5::US5_Crowded(v15, v16, v17) => {
            let v15: u64 = *v15;
            let v16: u64 = *v16;
            let v17: u64 = *v17;
            let v18: Rc<str> = format_real_13(v15);
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== crowding: "); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" windows granted in the last 3 min need "); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
            let v23: Rc<str> = format_real_13(v16);
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB, "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
            let v27: Rc<str> = format_real_13(v17);
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB free: hold new heavy work until they settle"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
            v30.clone()
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> Rc<str> {
        let v2: u64 = free_mb_0();
        machine_crowding_1(v0.clone(), v2, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_machine_crowding(v0: &str, v1: u64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), v1)
}
