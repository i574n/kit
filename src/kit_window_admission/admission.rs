#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Admitted,
    US0_HeldBack(Rc<str>),
    US0_LowMemory(u64, u64),
}
struct Mut0 { l0: Rc<str> }
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
}
#[derive(Clone)]
enum US1 {
    US1_ClassKnown(US2),
    US1_ClassUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_Held,
    US5_Waiting,
}
#[derive(Clone)]
enum US4 {
    US4_RowKnown(Rc<str>, US2, US5, Rc<str>, u64, u64),
    US4_RowUnknown,
}
fn method4(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_3(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method4(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn number_text_2(mut v0: u64) -> Rc<str> {
    format_real_3(v0)
}
fn admission_terms_1(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Admitted => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US0::US0_HeldBack(v2) => {
            let v2: Rc<str> = v2.clone();
            v2.clone()
        }
        US0::US0_LowMemory(v3, v4) => {
            let mut v3: u64 = *v3;
            let mut v4: u64 = *v4;
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason=low-memory free_mb="); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = number_text_2(v3);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" need_mb="); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = number_text_2(v4);
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v6, &*v7, &*v8, &*v9].concat());
            v10.clone()
        }
    }
}
fn class_word_6(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US2::US2_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US2::US2_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US2::US2_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US2::US2_LiveGui => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US2::US2_TestSuite => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US2::US2_TreeRead => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn class_of_5(mut v0: Rc<str>) -> US1 {
    let v1: US2 = US2::US2_LiveGui;
    let v2: Rc<str> = class_word_6(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US2 = US2::US2_LiveGui;
        US1::US1_ClassKnown(v4)
    } else {
        let v6: US2 = US2::US2_TestSuite;
        let v7: Rc<str> = class_word_6(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: US2 = US2::US2_TestSuite;
            US1::US1_ClassKnown(v9)
        } else {
            let v11: US2 = US2::US2_HeavyBuild;
            let v12: Rc<str> = class_word_6(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US2 = US2::US2_HeavyBuild;
                US1::US1_ClassKnown(v14)
            } else {
                let v16: US2 = US2::US2_Compile;
                let v17: Rc<str> = class_word_6(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US2 = US2::US2_Compile;
                    US1::US1_ClassKnown(v19)
                } else {
                    let v21: US2 = US2::US2_CompilerInstall;
                    let v22: Rc<str> = class_word_6(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US2 = US2::US2_CompilerInstall;
                        US1::US1_ClassKnown(v24)
                    } else {
                        let v26: US2 = US2::US2_TreeRead;
                        let v27: Rc<str> = class_word_6(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: US2 = US2::US2_TreeRead;
                            US1::US1_ClassKnown(v29)
                        } else {
                            let v31: US2 = US2::US2_Bench;
                            let v32: Rc<str> = class_word_6(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US2 = US2::US2_Bench;
                                US1::US1_ClassKnown(v34)
                            } else {
                                let v36: US2 = US2::US2_CompileHold;
                                let v37: Rc<str> = class_word_6(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US2 = US2::US2_CompileHold;
                                    US1::US1_ClassKnown(v39)
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
fn first_row_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn tab_field_9(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn method11(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_10(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method11(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn other_rows_12(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn held_until_latest_7(mut v0: Rc<str>, mut v1: u64) -> u64 {
    loop {
        let v2: bool = &*v0 == "";
        if v2 {
            return v1;
        } else {
            let mut v3: Rc<str> = first_row_8(v0.clone());
            let mut v4: i32 = 2i32;
            let v5: Rc<str> = tab_field_9(v3.clone(), v4);
            let v6: bool = &*v5 == "held";
            let v10: u64 = if v6 {
                let v7: i32 = 5i32;
                let v8: Rc<str> = tab_field_9(v3.clone(), v7);
                decimal_of_10(v8.clone())
            } else {
                0u64
            };
            let v11: Rc<str> = other_rows_12(v0.clone());
            let v12: bool = v10 > v1;
            let v13: u64 = if v12 {
                v10
            } else {
                v1
            };
            (v0, v1) = (v11.clone(), v13);
            continue;
        }
    }
}
fn needed_mb_14(mut v0: US2) -> u64 {
    match &v0 {
        US2::US2_Bench => {
            2048u64
        }
        US2::US2_Compile => {
            1024u64
        }
        US2::US2_CompileHold => {
            0u64
        }
        US2::US2_CompilerInstall => {
            2048u64
        }
        US2::US2_HeavyBuild => {
            1024u64
        }
        US2::US2_LiveGui => {
            4096u64
        }
        US2::US2_TestSuite => {
            3072u64
        }
        US2::US2_TreeRead => {
            0u64
        }
    }
}
fn floor_of_13(mut v0: US2, mut v1: u64) -> u64 {
    let v2: u64 = needed_mb_14(v0.clone());
    let mut v3: bool = v1 > v2;
    if v3 {
        v1
    } else {
        needed_mb_14(v0)
    }
}
fn compiles_18(mut v0: US2) -> bool {
    match &v0 {
        US2::US2_Compile => {
            true
        }
        _ => {
            false
        }
    }
}
fn lease_only_19(mut v0: US2) -> bool {
    match &v0 {
        US2::US2_TreeRead => {
            true
        }
        _ => {
            false
        }
    }
}
fn exclusive_20(mut v0: US2) -> bool {
    match &v0 {
        US2::US2_CompilerInstall => {
            true
        }
        US2::US2_LiveGui => {
            true
        }
        _ => {
            false
        }
    }
}
fn quiet_21(mut v0: US2) -> bool {
    match &v0 {
        US2::US2_Bench => {
            true
        }
        _ => {
            false
        }
    }
}
fn loud_22(mut v0: US2) -> bool {
    match &v0 {
        US2::US2_Bench => {
            true
        }
        US2::US2_HeavyBuild => {
            true
        }
        US2::US2_TestSuite => {
            true
        }
        _ => {
            false
        }
    }
}
fn holds_compiles_23(mut v0: US2) -> bool {
    match &v0 {
        US2::US2_CompileHold => {
            true
        }
        _ => {
            false
        }
    }
}
fn clashes_17(mut v0: US2, mut v1: US2) -> bool {
    let v2: bool = lease_only_19(v0.clone());
    let mut v4: bool = if v2 {
        true
    } else {
        lease_only_19(v1.clone())
    };
    if v4 {
        false
    } else {
        let v5: bool = exclusive_20(v0.clone());
        let v7: bool = if v5 {
            true
        } else {
            exclusive_20(v1.clone())
        };
        if v7 {
            true
        } else {
            let v8: bool = quiet_21(v0.clone());
            let v10: bool = if v8 {
                loud_22(v1.clone())
            } else {
                false
            };
            let v14: bool = if v10 {
                true
            } else {
                let v11: bool = quiet_21(v1.clone());
                if v11 {
                    loud_22(v0.clone())
                } else {
                    false
                }
            };
            let v18: bool = if v14 {
                true
            } else {
                let v15: bool = holds_compiles_23(v0.clone());
                if v15 {
                    compiles_18(v1.clone())
                } else {
                    false
                }
            };
            if v18 {
                true
            } else {
                let v19: bool = holds_compiles_23(v1);
                if v19 {
                    compiles_18(v0)
                } else {
                    false
                }
            }
        }
    }
}
fn row_of_24(mut v0: Rc<str>) -> US4 {
    let v1: i32 = 1i32;
    let v2: Rc<str> = tab_field_9(v0.clone(), v1);
    let mut v3: US1 = class_of_5(v2);
    match &v3 {
        US1::US1_ClassKnown(v5) => {
            let v5: US2 = v5.clone();
            let v6: i32 = 2i32;
            let v7: Rc<str> = tab_field_9(v0.clone(), v6);
            let v8: bool = &*v7 == "held";
            let v11: US5 = if v8 {
                US5::US5_Held
            } else {
                US5::US5_Waiting
            };
            let v12: i32 = 0i32;
            let v13: Rc<str> = tab_field_9(v0.clone(), v12);
            let v14: i32 = 3i32;
            let v15: Rc<str> = tab_field_9(v0.clone(), v14);
            let v16: i32 = 4i32;
            let v17: Rc<str> = tab_field_9(v0.clone(), v16);
            let v18: u64 = decimal_of_10(v17);
            let v19: i32 = 5i32;
            let v20: Rc<str> = tab_field_9(v0, v19);
            let v21: u64 = decimal_of_10(v20);
            US4::US4_RowKnown(v13, v5, v11, v15, v18, v21)
        }
        US1::US1_ClassUnknown => {
            US4::US4_RowUnknown
        }
    }
}
fn row_blocks_16(mut v0: US2, mut v1: Rc<str>, mut v2: u64, mut v3: bool, mut v4: Rc<str>) -> bool {
    let v5: US4 = row_of_24(v4);
    match &v5 {
        US4::US4_RowKnown(v6, v7, v8, v9, v10, v11) => {
            let v6: Rc<str> = v6.clone();
            let v7: US2 = v7.clone();
            let v8: US5 = v8.clone();
            let v9: Rc<str> = v9.clone();
            let v10: u64 = *v10;
            let v11: u64 = *v11;
            let v12: bool = &*v6 == &*v1;
            if v12 {
                false
            } else {
                match &v8 {
                    US5::US5_Held => {
                        clashes_17(v7.clone(), v0.clone())
                    }
                    US5::US5_Waiting => {
                        let v14: bool = exclusive_20(v7.clone());
                        let v16: bool = if v14 {
                            true
                        } else {
                            quiet_21(v7.clone())
                        };
                        let v18: bool = if v16 {
                            true
                        } else {
                            holds_compiles_23(v7.clone())
                        };
                        let v20: bool = if v18 {
                            let v19: bool = v10 < v2;
                            v19
                        } else {
                            false
                        };
                        let v22: bool = if v20 {
                            clashes_17(v7, v0)
                        } else {
                            false
                        };
                        if v22 {
                            let v23: bool = v3 == false;
                            v23
                        } else {
                            false
                        }
                    }
                }
            }
        }
        US4::US4_RowUnknown => {
            false
        }
    }
}
fn first_blocker_15(mut v0: US2, mut v1: Rc<str>, mut v2: u64, mut v3: bool, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = &*v4 == "";
        if v5 {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v6.clone();
        } else {
            let v7: Rc<str> = first_row_8(v4.clone());
            let v8: bool = row_blocks_16(v0.clone(), v1.clone(), v2, v3, v7.clone());
            if v8 {
                return v7.clone();
            } else {
                let v9: Rc<str> = other_rows_12(v4.clone());
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2, v3, v9.clone());
                continue;
            }
        }
    }
}
fn settle_ms_27(mut v0: US2) -> u64 {
    match &v0 {
        US2::US2_Bench => {
            90000u64
        }
        US2::US2_Compile => {
            20000u64
        }
        US2::US2_HeavyBuild => {
            90000u64
        }
        US2::US2_TestSuite => {
            90000u64
        }
        _ => {
            0u64
        }
    }
}
fn settling_mb_26(mut v0: Rc<str>, mut v1: u64) -> u64 {
    let v2: US4 = row_of_24(v0);
    match &v2 {
        US4::US4_RowKnown(v3, v4, v5, v6, v7, v8) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: US2 = v4.clone();
            let v5: US5 = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: u64 = *v7;
            let v8: u64 = *v8;
            match &v5 {
                US5::US5_Held => {
                    let v9: u64 = settle_ms_27(v4.clone());
                    let v10: u64 = v7.wrapping_add(v9);
                    let v11: bool = v1 < v10;
                    if v11 {
                        needed_mb_14(v4)
                    } else {
                        0u64
                    }
                }
                US5::US5_Waiting => {
                    0u64
                }
            }
        }
        US4::US4_RowUnknown => {
            0u64
        }
    }
}
fn settling_total_25(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> u64 {
    loop {
        let mut v3: bool = &*v0 == "";
        if v3 {
            return v2;
        } else {
            let mut v4: Rc<str> = other_rows_12(v0.clone());
            let v5: Rc<str> = first_row_8(v0.clone());
            let v6: u64 = settling_mb_26(v5.clone(), v1);
            let v7: u64 = v2.wrapping_add(v6);
            (v0, v1, v2) = (v4.clone(), v1, v7);
            continue;
        }
    }
}
fn free_after_settling_28(mut v0: u64, mut v1: u64) -> u64 {
    let v2: bool = v0 > v1;
    if v2 {
        let mut v3: u64 = v0.wrapping_sub(v1);
        v3
    } else {
        0u64
    }
}
fn minutes_left_31(mut v0: u64, mut v1: u64) -> u64 {
    let v2: bool = v0 <= v1;
    if v2 {
        0u64
    } else {
        let mut v3: u64 = v0.wrapping_sub(v1);
        let mut v4: u64 = v3.wrapping_add(59999u64);
        let v5: u64 = v4.wrapping_div(60000u64);
        v5
    }
}
fn blocker_terms_30(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by="); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = 0i32;
    let mut v4: Rc<str> = tab_field_9(v0.clone(), v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" holder="); } LIT.with(|lit| lit.clone()) };
    let v6: i32 = 1i32;
    let v7: Rc<str> = tab_field_9(v0.clone(), v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v4, &*v5, &*v7, &*v8].concat());
    let v10: i32 = 2i32;
    let v11: Rc<str> = tab_field_9(v0.clone(), v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" left_min="); } LIT.with(|lit| lit.clone()) };
    let v13: i32 = 5i32;
    let v14: Rc<str> = tab_field_9(v0.clone(), v13);
    let v15: u64 = decimal_of_10(v14.clone());
    let v16: u64 = minutes_left_31(v15, v1);
    let v17: Rc<str> = number_text_2(v16);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label='"); } LIT.with(|lit| lit.clone()) };
    let v19: i32 = 3i32;
    let v20: Rc<str> = tab_field_9(v0.clone(), v19);
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v20, &*v21, &*v22, &*v22].concat());
    let v24: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v11, &*v12, &*v17, &*v23].concat());
    v24.clone()
}
fn admission_needing_29(mut v0: u64, mut v1: Rc<str>, mut v2: u64, mut v3: u64) -> US0 {
    let mut v4: bool = &*v1 != "";
    if v4 {
        let v5: Rc<str> = blocker_terms_30(v1, v3);
        US0::US0_HeldBack(v5)
    } else {
        let v7: bool = v2 < v0;
        if v7 {
            US0::US0_LowMemory(v2, v0)
        } else {
            US0::US0_Admitted
        }
    }
}
fn verdict_terms_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64) -> Rc<str> {
    let v8: US1 = class_of_5(v0.clone());
    match &v8 {
        US1::US1_ClassKnown(v10) => {
            let v10: US2 = v10.clone();
            let v11: bool = v3 > 0u64;
            let v17: bool = if v11 {
                let v12: u64 = v3.wrapping_mul(60000u64);
                let v13: u64 = v6.wrapping_add(v12);
                let v14: u64 = 0u64;
                let v15: u64 = held_until_latest_7(v4.clone(), v14);
                let v16: bool = v13 <= v15;
                v16
            } else {
                false
            };
            let v18: u64 = floor_of_13(v10.clone(), v7);
            let v19: Rc<str> = first_blocker_15(v10.clone(), v1.clone(), v2, v17, v4.clone());
            let v20: u64 = 0u64;
            let v21: u64 = settling_total_25(v4.clone(), v6, v20);
            let v22: u64 = free_after_settling_28(v5, v21);
            let v23: US0 = admission_needing_29(v18, v19.clone(), v22, v6);
            admission_terms_1(v23.clone())
        }
        US1::US1_ClassUnknown => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason=unknown-class"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, u64, u64, Rc<str>, u64, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, u64, u64, Rc<str>, u64, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64| -> Rc<str> {
        verdict_terms_0(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5, v6, v7)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_window_verdict(v0: &str, v1: &str, v2: u64, v3: u64, v4: &str, v5: u64, v6: u64, v7: u64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, v3, Rc::<str>::from(v4), v5, v6, v7)
}
