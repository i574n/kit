#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Admitted,
    US0_HeldBack(Rc<str>),
    US0_LowMemory(u64, u64),
}
pub struct Mut0 { pub l0: Rc<str> }
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
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn method5(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn number_text_3(mut v0: u64) -> Rc<str> {
    format_real_4(v0)
}
fn admission_terms_1(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Admitted => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_HeldBack(v2) => {
            let v2: Rc<str> = v2;
            v2
        }
        US0::US0_LowMemory(v3, v4) => {
            let mut v3: u64 = v3;
            let mut v4: u64 = v4;
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason=low-memory free_mb="); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = number_text_3(v3);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" need_mb="); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = number_text_3(v4);
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_2(v5, v6, v7, v8, v9)
        }
    }
}
fn class_word_7(mut v0: US2) -> Rc<str> {
    match v0 {
        US2::US2_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US2::US2_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US2::US2_CompilerInstall => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US2::US2_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
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
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US2::US2_TreeWrite => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-write"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn class_of_6(mut v0: Rc<str>) -> US1 {
    let v1: US2 = US2::US2_LiveGui;
    let v2: Rc<str> = class_word_7(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US2 = US2::US2_LiveGui;
        US1::US1_ClassKnown(v4)
    } else {
        let v6: US2 = US2::US2_TestSuite;
        let v7: Rc<str> = class_word_7(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: US2 = US2::US2_TestSuite;
            US1::US1_ClassKnown(v9)
        } else {
            let v11: US2 = US2::US2_HeavyBuild;
            let v12: Rc<str> = class_word_7(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US2 = US2::US2_HeavyBuild;
                US1::US1_ClassKnown(v14)
            } else {
                let v16: US2 = US2::US2_Compile;
                let v17: Rc<str> = class_word_7(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US2 = US2::US2_Compile;
                    US1::US1_ClassKnown(v19)
                } else {
                    let v21: US2 = US2::US2_CompilerInstall;
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
                                let v36: US2 = US2::US2_CompileHold;
                                let v37: Rc<str> = class_word_7(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US2 = US2::US2_CompileHold;
                                    US1::US1_ClassKnown(v39)
                                } else {
                                    let v41: US2 = US2::US2_TreeWrite;
                                    let v42: Rc<str> = class_word_7(v41);
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
fn first_row_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn tab_field_10(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn method12(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_11(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method12(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn other_rows_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn held_until_latest_8(mut v0: Rc<str>, mut v1: u64) -> u64 {
    loop {
        let v2: bool = &*v0 == "";
        if v2 {
            return v1;
        } else {
            let mut v3: Rc<str> = first_row_9(v0.clone());
            let mut v4: i32 = 2i32;
            let mut v5: Rc<str> = tab_field_10(v3.clone(), v4);
            let v6: bool = &*v5 == "held";
            let v10: u64 = if v6 {
                let v7: i32 = 5i32;
                let v8: Rc<str> = tab_field_10(v3, v7);
                decimal_of_11(v8)
            } else {
                0u64
            };
            let v11: Rc<str> = other_rows_13(v0);
            let v12: bool = v10 > v1;
            let v13: u64 = if v12 {
                v10
            } else {
                v1
            };
            (v0, v1) = (v11, v13);
            continue;
        }
    }
}
fn needed_mb_15(mut v0: US2) -> u64 {
    match v0 {
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
        US2::US2_TreeWrite => {
            0u64
        }
    }
}
fn floor_of_14(mut v0: US2, mut v1: u64) -> u64 {
    let v2: u64 = needed_mb_15(v0.clone());
    let mut v3: bool = v1 > v2;
    if v3 {
        v1
    } else {
        needed_mb_15(v0)
    }
}
fn compiles_19(mut v0: US2) -> bool {
    match v0 {
        US2::US2_Compile => {
            true
        }
        _ => {
            false
        }
    }
}
fn lease_only_20(mut v0: US2) -> bool {
    match v0 {
        US2::US2_TreeRead => {
            true
        }
        US2::US2_TreeWrite => {
            true
        }
        _ => {
            false
        }
    }
}
fn exclusive_21(mut v0: US2) -> bool {
    match v0 {
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
fn quiet_22(mut v0: US2) -> bool {
    match v0 {
        US2::US2_Bench => {
            true
        }
        _ => {
            false
        }
    }
}
fn loud_23(mut v0: US2) -> bool {
    match v0 {
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
fn holds_compiles_24(mut v0: US2) -> bool {
    match v0 {
        US2::US2_CompileHold => {
            true
        }
        _ => {
            false
        }
    }
}
fn clashes_18(mut v0: US2, mut v1: US2) -> bool {
    let v2: bool = lease_only_20(v0.clone());
    let mut v4: bool = if v2 {
        true
    } else {
        lease_only_20(v1.clone())
    };
    if v4 {
        false
    } else {
        let mut v5: bool = exclusive_21(v0.clone());
        let v7: bool = if v5 {
            true
        } else {
            exclusive_21(v1.clone())
        };
        if v7 {
            true
        } else {
            let v8: bool = quiet_22(v0.clone());
            let v10: bool = if v8 {
                loud_23(v1.clone())
            } else {
                false
            };
            let v14: bool = if v10 {
                true
            } else {
                let v11: bool = quiet_22(v1.clone());
                if v11 {
                    loud_23(v0.clone())
                } else {
                    false
                }
            };
            let v18: bool = if v14 {
                true
            } else {
                let v15: bool = holds_compiles_24(v0.clone());
                if v15 {
                    compiles_19(v1.clone())
                } else {
                    false
                }
            };
            if v18 {
                true
            } else {
                let v19: bool = holds_compiles_24(v1);
                if v19 {
                    compiles_19(v0)
                } else {
                    false
                }
            }
        }
    }
}
fn row_of_25(mut v0: Rc<str>) -> US4 {
    let v1: i32 = 1i32;
    let v2: Rc<str> = tab_field_10(v0.clone(), v1);
    let mut v3: US1 = class_of_6(v2);
    match v3 {
        US1::US1_ClassKnown(v5) => {
            let mut v5: US2 = v5;
            let v6: i32 = 2i32;
            let v7: Rc<str> = tab_field_10(v0.clone(), v6);
            let v8: bool = &*v7 == "held";
            let v11: US5 = if v8 {
                US5::US5_Held
            } else {
                US5::US5_Waiting
            };
            let v12: i32 = 0i32;
            let v13: Rc<str> = tab_field_10(v0.clone(), v12);
            let v14: i32 = 3i32;
            let v15: Rc<str> = tab_field_10(v0.clone(), v14);
            let v16: i32 = 4i32;
            let v17: Rc<str> = tab_field_10(v0.clone(), v16);
            let v18: u64 = decimal_of_11(v17);
            let v19: i32 = 5i32;
            let v20: Rc<str> = tab_field_10(v0, v19);
            let v21: u64 = decimal_of_11(v20);
            US4::US4_RowKnown(v13, v5, v11, v15, v18, v21)
        }
        US1::US1_ClassUnknown => {
            US4::US4_RowUnknown
        }
    }
}
fn writes_tree_27(mut v0: US2) -> bool {
    match v0 {
        US2::US2_TreeWrite => {
            true
        }
        _ => {
            false
        }
    }
}
fn tree_access_28(mut v0: US2) -> bool {
    match v0 {
        US2::US2_TreeRead => {
            true
        }
        US2::US2_TreeWrite => {
            true
        }
        _ => {
            false
        }
    }
}
fn tree_clash_26(mut v0: US2, mut v1: US2) -> bool {
    let v2: bool = tree_access_28(v0.clone());
    let mut v4: bool = if v2 {
        tree_access_28(v1.clone())
    } else {
        false
    };
    if v4 {
        let mut v5: bool = writes_tree_27(v0);
        if v5 {
            true
        } else {
            writes_tree_27(v1)
        }
    } else {
        false
    }
}
fn path_prefix_30(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = Rc::<str>::from(v3.to_lowercase());
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v2));
    v7
}
fn path_overlaps_29(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = path_prefix_30(v0);
    let mut v3: Rc<str> = path_prefix_30(v1);
    let mut v4: bool = v3.starts_with(&*v2);
    if v4 {
        true
    } else {
        let mut v5: bool = v2.starts_with(&*v3);
        v5
    }
}
fn row_blocks_17(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: bool, mut v5: Rc<str>) -> bool {
    let v6: US4 = row_of_25(v5);
    match v6 {
        US4::US4_RowKnown(v7, v8, v9, v10, v11, v12) => {
            let v7: Rc<str> = v7;
            let v8: US2 = v8;
            let v9: US5 = v9;
            let v10: Rc<str> = v10;
            let v11: u64 = v11;
            let v12: u64 = v12;
            let v13: bool = &*v7 == &*v2;
            if v13 {
                false
            } else {
                let v14: bool = tree_clash_26(v8.clone(), v0.clone());
                let v16: bool = if v14 {
                    path_overlaps_29(v10, v1)
                } else {
                    false
                };
                if v16 {
                    match v9 {
                        US5::US5_Held => {
                            true
                        }
                        US5::US5_Waiting => {
                            let v17: bool = writes_tree_27(v8);
                            if v17 {
                                let v18: bool = v11 < v3;
                                v18
                            } else {
                                false
                            }
                        }
                    }
                } else {
                    match v9 {
                        US5::US5_Held => {
                            clashes_18(v8, v0)
                        }
                        US5::US5_Waiting => {
                            let v23: bool = exclusive_21(v8.clone());
                            let v25: bool = if v23 {
                                true
                            } else {
                                quiet_22(v8.clone())
                            };
                            let v27: bool = if v25 {
                                true
                            } else {
                                holds_compiles_24(v8.clone())
                            };
                            let v29: bool = if v27 {
                                let v28: bool = v11 < v3;
                                v28
                            } else {
                                false
                            };
                            let v31: bool = if v29 {
                                clashes_18(v8, v0)
                            } else {
                                false
                            };
                            if v31 {
                                let v32: bool = v4 == false;
                                v32
                            } else {
                                false
                            }
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
fn first_blocker_16(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: bool, mut v5: Rc<str>) -> Rc<str> {
    loop {
        let v6: bool = &*v5 == "";
        if v6 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v7;
        } else {
            let v8: Rc<str> = first_row_9(v5.clone());
            let v9: bool = row_blocks_17(v0.clone(), v1.clone(), v2.clone(), v3, v4, v8.clone());
            if v9 {
                return v8;
            } else {
                let v10: Rc<str> = other_rows_13(v5);
                v5 = v10;
                continue;
            }
        }
    }
}
fn settle_ms_33(mut v0: US2) -> u64 {
    match v0 {
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
fn settling_mb_32(mut v0: Rc<str>, mut v1: u64) -> u64 {
    let v2: US4 = row_of_25(v0);
    match v2 {
        US4::US4_RowKnown(v3, v4, v5, v6, v7, v8) => {
            let mut v3: Rc<str> = v3;
            let mut v4: US2 = v4;
            let mut v5: US5 = v5;
            let v6: Rc<str> = v6;
            let v7: u64 = v7;
            let v8: u64 = v8;
            match v5 {
                US5::US5_Held => {
                    let v9: u64 = settle_ms_33(v4.clone());
                    let v10: u64 = v7.wrapping_add(v9);
                    let v11: bool = v1 < v10;
                    if v11 {
                        needed_mb_15(v4)
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
fn settling_total_31(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> u64 {
    loop {
        let mut v3: bool = &*v0 == "";
        if v3 {
            return v2;
        } else {
            let mut v4: Rc<str> = other_rows_13(v0.clone());
            let mut v5: Rc<str> = first_row_9(v0);
            let v6: u64 = settling_mb_32(v5, v1);
            let v7: u64 = v2.wrapping_add(v6);
            (v0, v2) = (v4, v7);
            continue;
        }
    }
}
fn free_after_settling_34(mut v0: u64, mut v1: u64) -> u64 {
    let v2: bool = v0 > v1;
    if v2 {
        let mut v3: u64 = v0.wrapping_sub(v1);
        v3
    } else {
        0u64
    }
}
fn minutes_left_37(mut v0: u64, mut v1: u64) -> u64 {
    let v2: bool = v0 <= v1;
    if v2 {
        0u64
    } else {
        let mut v3: u64 = v0.wrapping_sub(v1);
        let mut v4: u64 = v3.wrapping_add(59999u64);
        let mut v5: u64 = v4.wrapping_div(60000u64);
        v5
    }
}
fn blocker_terms_36(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by="); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = 0i32;
    let mut v4: Rc<str> = tab_field_10(v0.clone(), v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" holder="); } LIT.with(|lit| lit.clone()) };
    let v6: i32 = 1i32;
    let v7: Rc<str> = tab_field_10(v0.clone(), v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = concat5_2(v2, v4, v5, v7, v8);
    let v10: i32 = 2i32;
    let v11: Rc<str> = tab_field_10(v0.clone(), v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" left_min="); } LIT.with(|lit| lit.clone()) };
    let v13: i32 = 5i32;
    let v14: Rc<str> = tab_field_10(v0.clone(), v13);
    let v15: u64 = decimal_of_11(v14);
    let v16: u64 = minutes_left_37(v15, v1);
    let v17: Rc<str> = number_text_3(v16);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label='"); } LIT.with(|lit| lit.clone()) };
    let v19: i32 = 3i32;
    let v20: Rc<str> = tab_field_10(v0, v19);
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = concat5_2(v18, v20, v21, v22.clone(), v22);
    concat5_2(v9, v11, v12, v17, v23)
}
fn admission_needing_35(mut v0: u64, mut v1: Rc<str>, mut v2: u64, mut v3: u64) -> US0 {
    let mut v4: bool = &*v1 != "";
    if v4 {
        let mut v5: Rc<str> = blocker_terms_36(v1, v3);
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
fn verdict_terms_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: Rc<str>, mut v6: u64, mut v7: u64, mut v8: u64) -> Rc<str> {
    let v9: US1 = class_of_6(v0);
    match v9 {
        US1::US1_ClassKnown(v11) => {
            let v11: US2 = v11;
            let v12: bool = v4 > 0u64;
            let v18: bool = if v12 {
                let v13: u64 = v4.wrapping_mul(60000u64);
                let v14: u64 = v7.wrapping_add(v13);
                let v15: u64 = 0u64;
                let v16: u64 = held_until_latest_8(v5.clone(), v15);
                let v17: bool = v14 <= v16;
                v17
            } else {
                false
            };
            let v19: u64 = floor_of_14(v11.clone(), v8);
            let v20: Rc<str> = first_blocker_16(v11, v1, v2, v3, v18, v5.clone());
            let v21: u64 = 0u64;
            let v22: u64 = settling_total_31(v5, v7, v21);
            let v23: u64 = free_after_settling_34(v6, v22);
            let v24: US0 = admission_needing_35(v19, v20, v23, v7);
            admission_terms_1(v24)
        }
        US1::US1_ClassUnknown => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason=unknown-class"); } LIT.with(|lit| lit.clone()) };
            v10
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, u64, u64, Rc<str>, u64, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, u64, u64, Rc<str>, u64, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: Rc<str>, mut v6: u64, mut v7: u64, mut v8: u64| -> Rc<str> {
        verdict_terms_0(v0, v1, v2, v3, v4, v5, v6, v7, v8)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_window_verdict(v0: &str, v1: &str, v2: &str, v3: u64, v4: u64, v5: &str, v6: u64, v7: u64, v8: u64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3, v4, Rc::<str>::from(v5), v6, v7, v8)
}
pub type SpiralShape_97df63ebabc3 = Mut0;
