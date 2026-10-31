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
    US0_RowMiss,
    US0_RowSkip(i32),
    US0_RowHit(u64, u64, Rc<str>, i32, i32),
}
#[derive(Clone)]
enum US2 {
    US2_CFamily,
    US2_HashFamily,
    US2_DashFamily,
    US2_HaskellFamily,
    US2_LeanFamily,
    US2_LispFamily,
    US2_ErlangFamily,
    US2_MarkupFamily,
    US2_MlFamily,
    US2_PascalFamily,
    US2_BatchFamily,
    US2_FsharpFamily,
}
#[derive(Clone)]
enum US3 {
    US3_SkipNestedRepository,
    US3_SkipVendorDirectory,
    US3_SkipUnknownSyntax,
    US3_SkipGeneratedOutput,
    US3_SkipLinkedEntry,
}
#[derive(Clone)]
enum US1 {
    US1_ScanPath(US2),
    US1_SkipPath(US3),
}
#[derive(Clone)]
enum US4 {
    US4_ScanContent,
    US4_SkipBinary,
    US4_SkipLarge,
    US4_SkipUnreadable,
}
#[derive(Clone)]
enum US5 {
    US5_Replaced,
    US5_TemporaryWriteFailed,
    US5_RenameFailed,
    US5_RenameFailedTemporaryKept,
    US5_ReadbackMismatch,
    US5_ChangedSinceRead,
}
struct Mut0 { l0: Rc<str> }
fn file_exists_1(mut v0: Rc<str>) -> bool {
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
fn method2(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn get_environment_variable_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn cache_path_3(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v2));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = get_environment_variable_4(v8);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/kit/todo-cache/"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v9, v10)), v7));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    v14
}
fn harvest_roots_from_5(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v2.clone();
        } else {
            let mut v6: i32 = v4.wrapping_add(19i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v0.get(v6 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v6 as usize) as i32).unwrap_or(-1);
            let v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
            let v10: bool = &*v2 == "";
            let v14: Rc<str> = if v10 {
                v9.clone()
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v11, &*v9, &*v12, &*v12].concat());
                v13.clone()
            };
            let v15: i32 = v8.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v15, v14.clone());
            continue;
        }
    }
}
fn cached_head_of_8(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let (mut v2, mut v3): (Rc<str>, Rc<str>) = agile_todo_cache::kit_agile_todo_head_of(&*v0, &*v1);
    (v2, v3)
}
fn changed_in_12(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Option<char> = char::from_u32(0u32);
    let v4: char = v2.unwrap();
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}", v4)), v1));
    let v7: Option<char> = char::from_u32(0u32);
    let v9: char = v7.unwrap();
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v6, Rc::<str>::from(format!("{}", v9))));
    let v12: bool = v0.contains(&*v11);
    v12
}
fn last_byte_of_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v4);
            continue;
        }
    }
}
fn spiral_sibling_relative_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v2: i32 = 0i32;
    let v3: i32 = -1i32;
    let v4: i32 = last_byte_of_14(v0.clone(), v1, v2, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = v4.wrapping_add(1i32);
    let v7: i32 = -1i32;
    let v8: i32 = last_byte_of_14(v0.clone(), v5, v6, v7);
    let v9: bool = v8 > v6;
    let v11: Rc<str> = if v9 {
        let v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v8 as usize).unwrap_or(""));
        v10
    } else {
        v0
    };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    v13
}
fn unchanged_11(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>) -> bool {
    let v13: bool = if v8 {
        let v11: bool = changed_in_12(v9.clone(), v10.clone());
        let v12: bool = v11 == false;
        v12
    } else {
        false
    };
    if v13 {
        let v14: Rc<str> = spiral_sibling_relative_13(v10);
        let v15: bool = changed_in_12(v9, v14);
        let v16: bool = v15 == false;
        v16
    } else {
        false
    }
}
fn cached_row_of_15(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let (mut v3, mut v4, mut v5, mut v6, mut v7, mut v8): (i32, u64, u64, Rc<str>, i32, i32) = agile_todo_cache::kit_agile_todo_row_of(&*v0, v1, &*v2);
    let v9: bool = v3 == 2i32;
    if v9 {
        US0::US0_RowHit(v4, v5, v6, v7, v8)
    } else {
        let v11: bool = v3 == 1i32;
        if v11 {
            US0::US0_RowSkip(v8)
        } else {
            US0::US0_RowMiss
        }
    }
}
fn cache_row_18(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: Rc<str>, mut v4: i32) -> Rc<str> {
    let v5: Rc<str> = agile_todo_cache::kit_agile_todo_cache_row(&*v0, v1, v2, &*v3, v4);
    v5
}
fn recorded_17(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v15: Rc<str> = cache_row_18(v10, v11, v12, v13.clone(), v14);
    let v16: i32 = v2.wrapping_add(v14);
    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v0));
    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v1));
    (v17, v18, v16, v3, v4, v5, v6, v7, v8, v9)
}
fn hit_recorded_16(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: i32, mut v15: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = recorded_17(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14);
    let v26: i32 = v4.wrapping_add(1i32);
    (v16, v17, v18, v19, v26, v15, v22, v23, v24, v25)
}
fn skipped_row_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = agile_todo_cache::kit_agile_todo_skipped_row(&*v0);
    v1
}
fn skipped_19(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v11: Rc<str> = skipped_row_20(v10);
    let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v0));
    (v12, v1, v2, v3, v4, v5, v6, v7, v8, v9)
}
fn path_verdict_of_code_22(mut v0: i32) -> US1 {
    let v1: bool = 0i32 < v0;
    if v1 {
        let v2: bool = v0 == 1i32;
        let v38: US2 = if v2 {
            US2::US2_CFamily
        } else {
            let v4: bool = v0 == 2i32;
            if v4 {
                US2::US2_HashFamily
            } else {
                let mut v6: bool = v0 == 3i32;
                if v6 {
                    US2::US2_DashFamily
                } else {
                    let v8: bool = v0 == 4i32;
                    if v8 {
                        US2::US2_HaskellFamily
                    } else {
                        let v10: bool = v0 == 5i32;
                        if v10 {
                            US2::US2_LeanFamily
                        } else {
                            let v12: bool = v0 == 6i32;
                            if v12 {
                                US2::US2_LispFamily
                            } else {
                                let v14: bool = v0 == 7i32;
                                if v14 {
                                    US2::US2_ErlangFamily
                                } else {
                                    let v16: bool = v0 == 8i32;
                                    if v16 {
                                        US2::US2_MarkupFamily
                                    } else {
                                        let v18: bool = v0 == 9i32;
                                        if v18 {
                                            US2::US2_MlFamily
                                        } else {
                                            let v20: bool = v0 == 10i32;
                                            if v20 {
                                                US2::US2_PascalFamily
                                            } else {
                                                let v22: bool = v0 == 11i32;
                                                if v22 {
                                                    US2::US2_BatchFamily
                                                } else {
                                                    let v24: bool = v0 == 12i32;
                                                    if v24 {
                                                        US2::US2_FsharpFamily
                                                    } else {
                                                        US2::US2_CFamily
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
        US1::US1_ScanPath(v38)
    } else {
        let v40: i32 = v0.wrapping_neg();
        let v41: bool = v40 == 1i32;
        let v56: US3 = if v41 {
            US3::US3_SkipNestedRepository
        } else {
            let v43: bool = v40 == 2i32;
            if v43 {
                US3::US3_SkipVendorDirectory
            } else {
                let v45: bool = v40 == 3i32;
                if v45 {
                    US3::US3_SkipUnknownSyntax
                } else {
                    let v47: bool = v40 == 4i32;
                    if v47 {
                        US3::US3_SkipGeneratedOutput
                    } else {
                        let v49: bool = v40 == 5i32;
                        if v49 {
                            US3::US3_SkipLinkedEntry
                        } else {
                            US3::US3_SkipUnknownSyntax
                        }
                    }
                }
            }
        };
        US1::US1_SkipPath(v56)
    }
}
fn findings_of_24(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, i32) {
    let v4: i32 = match &v0 {
        US2::US2_BatchFamily => {
            11i32
        }
        US2::US2_CFamily => {
            1i32
        }
        US2::US2_DashFamily => {
            3i32
        }
        US2::US2_ErlangFamily => {
            7i32
        }
        US2::US2_FsharpFamily => {
            12i32
        }
        US2::US2_HashFamily => {
            2i32
        }
        US2::US2_HaskellFamily => {
            4i32
        }
        US2::US2_LeanFamily => {
            5i32
        }
        US2::US2_LispFamily => {
            6i32
        }
        US2::US2_MarkupFamily => {
            8i32
        }
        US2::US2_MlFamily => {
            9i32
        }
        US2::US2_PascalFamily => {
            10i32
        }
    };
    let (mut v5, mut v6, mut v7): (Rc<str>, i32, Rc<str>) = agile_todo_lines::kit_agile_todo_findings(v4, "", &*v1, &*v2, &*v3, true);
    (v5, v6)
}
fn ids_in_25(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = agile_todo_cache::kit_agile_todo_ids_in(&*v0);
    v1
}
fn occurrences_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = (v1.clone().len() as i32);
            let v7: i32 = v4.wrapping_add(v6);
            let v8: i32 = v3.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v7, v8);
            continue;
        }
    }
}
fn words_in_26(mut v0: Rc<str>) -> i32 {
    let v1: bool = &*v0 == "";
    if v1 {
        0i32
    } else {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v3: i32 = 0i32;
        let v4: i32 = 0i32;
        let v5: i32 = occurrences_27(v0, v2, v3, v4);
        let mut v6: i32 = v5.wrapping_add(1i32);
        v6
    }
}
fn file_read_23(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: US2, mut v11: Rc<str>, mut v12: u64, mut v13: u64) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let (mut v14, mut v15): (bool, Rc<str>) = match std::fs::read(&*v11) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
    let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v14 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v17: i32 = 0i32;
        let (mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = recorded_17(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v11.clone(), v12, v13, v16, v17);
        let v28: i32 = v3.wrapping_add(1i32);
        (v18, v19, v20, v28, v22, v23, v24, v25, v26, v27)
    } else {
        let (mut v29, mut v30): (Rc<str>, i32) = findings_of_24(v10, v7.clone(), v15, v11.clone());
        let v31: Rc<str> = ids_in_25(v29);
        let v32: i32 = words_in_26(v31.clone());
        let v33: i32 = v30.wrapping_sub(v32);
        let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = recorded_17(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v11, v12, v13, v31, v33);
        let v44: i32 = v3.wrapping_add(1i32);
        (v34, v35, v36, v44, v38, v39, v40, v41, v42, v43)
    };
    (v45, v46, v47, v48, v49, v50, v51, v52, v53, v54)
}
fn file_scanned_21(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v12: bool = std::path::Path::new(&*v10).with_extension("spi").is_file();
    let v13: i32 = agile_todo_paths::kit_agile_todo_path_verdict(&*v11, v12);
    let v14: US1 = path_verdict_of_code_22(v13);
    let (mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match &v14 {
        US1::US1_ScanPath(v15) => {
            let v15: US2 = v15.clone();
            let v16: u64 = std::fs::metadata(&*v10).map(|metadata| metadata.len()).unwrap_or(0);
            let v17: bool = v16 > 33554432u64;
            let v20: US4 = if v17 {
                US4::US4_SkipLarge
            } else {
                US4::US4_ScanContent
            };
            match &v20 {
                US4::US4_SkipLarge => {
                    skipped_19(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v10.clone())
                }
                _ => {
                    let v31: u64 = std::fs::metadata(&*v10).and_then(|metadata| metadata.modified()).ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
                    let v32: US0 = cached_row_of_15(v6.clone(), v5, v10.clone());
                    let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match &v32 {
                        US0::US0_RowHit(v33, v34, v35, v36, v37) => {
                            let v33: u64 = *v33;
                            let v34: u64 = *v34;
                            let v35: Rc<str> = v35.clone();
                            let v36: i32 = *v36;
                            let v37: i32 = *v37;
                            let v38: bool = v33 == v16;
                            let v40: bool = if v38 {
                                let v39: bool = v34 == v31;
                                v39
                            } else {
                                false
                            };
                            if v40 {
                                let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = hit_recorded_16(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v10.clone(), v16, v31, v35.clone(), v36, v37);
                                (v41.clone(), v42.clone(), v43, v44, v45, v46, v47.clone(), v48.clone(), v49, v50.clone())
                            } else {
                                let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = file_read_23(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v15.clone(), v10.clone(), v16, v31);
                                (v51.clone(), v52.clone(), v53, v54, v55, v56, v57.clone(), v58.clone(), v59, v60.clone())
                            }
                        }
                        _ => {
                            let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = file_read_23(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v15.clone(), v10.clone(), v16, v31);
                            (v71.clone(), v72.clone(), v73, v74, v75, v76, v77.clone(), v78.clone(), v79, v80.clone())
                        }
                    };
                    (v81.clone(), v82.clone(), v83, v84, v85, v86, v87.clone(), v88.clone(), v89, v90.clone())
                }
            }
        }
        _ => {
            let (mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = skipped_19(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v10.clone());
            (v101.clone(), v102.clone(), v103, v104, v105, v106, v107.clone(), v108.clone(), v109, v110.clone())
        }
    };
    (v111.clone(), v112.clone(), v113, v114, v115, v116, v117.clone(), v118.clone(), v119, v120.clone())
}
fn file_walked_10(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v12)), v11));
    let v15: bool = unchanged_11(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v11.clone());
    let v18: US0 = if v15 {
        cached_row_of_15(v6.clone(), v5, v14.clone())
    } else {
        US0::US0_RowMiss
    };
    let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match &v18 {
        US0::US0_RowHit(v19, v20, v21, v22, v23) => {
            let v19: u64 = *v19;
            let v20: u64 = *v20;
            let v21: Rc<str> = v21.clone();
            let v22: i32 = *v22;
            let v23: i32 = *v23;
            hit_recorded_16(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v14.clone(), v19, v20, v21, v22, v23)
        }
        _ => {
            let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match &v18 {
                US0::US0_RowSkip(v34) => {
                    let v34: i32 = *v34;
                    let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = skipped_19(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v14.clone());
                    (v35, v36, v37, v38, v39, v34, v41, v42, v43, v44)
                }
                _ => {
                    let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match &v18 {
                        US0::US0_RowMiss => {
                            file_scanned_21(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v11)
                        }
                        _ => unreachable!(),
                    };
                    (v55, v56, v57, v58, v59, v60, v61, v62, v63, v64)
                }
            };
            (v65, v66, v67, v68, v69, v70, v71, v72, v73, v74)
        }
    };
    (v75, v76, v77, v78, v79, v80, v81, v82, v83, v84)
}
fn listed_9(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v13: i32 = (v11.clone().len() as i32);
    let v14: bool = v13 <= v12;
    let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v14 {
        (v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone())
    } else {
        let v15: Option<char> = char::from_u32(0u32);
        let v17: char = v15.unwrap();
        let v18: Rc<str> = Rc::<str>::from(format!("{}", v17));
        let v19: i32 = v11.get(v12 as usize..).and_then(|rest| rest.find(&*v18)).map(|found| (found + v12 as usize) as i32).unwrap_or(-1);
        let v20: bool = v19 < 0i32;
        let v21: i32 = if v20 {
            v13
        } else {
            v19
        };
        let v22: Rc<str> = std::rc::Rc::<str>::from(v11.get(v12 as usize..v21 as usize).unwrap_or(""));
        let v23: bool = &*v22 == "";
        let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v23 {
            (v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone())
        } else {
            let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = file_walked_10(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v10.clone(), v22.clone());
            (v24.clone(), v25.clone(), v26, v27, v28, v29, v30.clone(), v31.clone(), v32, v33.clone())
        };
        let v44: i32 = v21.wrapping_add(1i32);
        let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = listed_9(v34.clone(), v35.clone(), v36, v37, v38, v39, v40.clone(), v41.clone(), v42, v43.clone(), v10.clone(), v11.clone(), v44);
        (v45.clone(), v46.clone(), v47, v48, v49, v50, v51.clone(), v52.clone(), v53, v54.clone())
    };
    (v55.clone(), v56.clone(), v57, v58, v59, v60, v61.clone(), v62.clone(), v63, v64.clone())
}
fn head_row_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = agile_todo_cache::kit_agile_todo_head_row(&*v0, &*v1, &*v2);
    v3
}
fn root_walked_7(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let (mut v11, mut v12): (bool, Rc<str>) = agile_todo_sources::kit_agile_todo_sources(&*v10);
    let v13: Rc<str> = agile_todo_sources::kit_agile_todo_git_head(&*v10);
    let (mut v14, mut v15): (Rc<str>, Rc<str>) = cached_head_of_8(v6.clone(), v10.clone());
    let v16: bool = &*v13 == "";
    let v19: Rc<str> = if v16 {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v17
    } else {
        let v18: Rc<str> = agile_todo_sources::kit_agile_todo_git_changed(&*v10, &*v13);
        v18
    };
    let v21: bool = if v16 {
        true
    } else {
        let v20: bool = &*v14 == "";
        v20
    };
    let v26: Rc<str> = if v21 {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v22
    } else {
        let v23: bool = &*v14 == &*v13;
        if v23 {
            v19.clone()
        } else {
            let v24: Rc<str> = agile_todo_sources::kit_agile_todo_git_changed(&*v10, &*v14);
            v24
        }
    };
    let v27: bool = &*v26 == "";
    let v29: bool = if v27 {
        true
    } else {
        let v28: bool = &*v19 == "";
        v28
    };
    let v43: Rc<str> = if v29 {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v30
    } else {
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v19));
        let v32: Option<char> = char::from_u32(0u32);
        let v34: char = v32.unwrap();
        let v37: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v31, Rc::<str>::from(format!("{}", v34)))), v15));
        let v38: Option<char> = char::from_u32(0u32);
        let v40: char = v38.unwrap();
        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v37, Rc::<str>::from(format!("{}", v40))));
        v42
    };
    let v44: bool = &*v43 != "";
    let (mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v11 {
        let v45: i32 = 0i32;
        let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = listed_9(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v44, v43.clone(), v10.clone(), v12, v45);
        (v46, v47, v48, v49, v50, v51, v52, v53, v54, v55)
    } else {
        (v0, v1, v2, v3, v4, v5, v6, v7, v44, v43)
    };
    let v67: bool = if v16 {
        true
    } else {
        let v66: bool = &*v19 == "";
        v66
    };
    let v70: Rc<UH0> = if v67 {
        v56.clone()
    } else {
        let v68: Rc<str> = head_row_28(v10, v13, v19);
        Rc::new(UH0::UH0_Cons(v68, v56))
    };
    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v70, v57, v58, v59, v60, v61, v62, v63, false, v71)
}
fn roots_walked_6(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v12: i32 = (v10.clone().len() as i32);
    let v13: bool = v12 <= v11;
    let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v13 {
        (v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone())
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v15: i32 = v10.get(v11 as usize..).and_then(|rest| rest.find(&*v14)).map(|found| (found + v11 as usize) as i32).unwrap_or(-1);
        let v16: bool = v15 < 0i32;
        let v17: i32 = if v16 {
            v12
        } else {
            v15
        };
        let v18: Rc<str> = std::rc::Rc::<str>::from(v10.get(v11 as usize..v17 as usize).unwrap_or(""));
        let v19: bool = &*v18 == "";
        let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v19 {
            (v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone())
        } else {
            let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = root_walked_7(v0.clone(), v1.clone(), v2, v3, v4, v5, v6.clone(), v7.clone(), v8, v9.clone(), v18.clone());
            (v20.clone(), v21.clone(), v22, v23, v24, v25, v26.clone(), v27.clone(), v28, v29.clone())
        };
        let v40: i32 = v17.wrapping_add(1i32);
        let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = roots_walked_6(v30.clone(), v31.clone(), v32, v33, v34, v35, v36.clone(), v37.clone(), v38, v39.clone(), v10.clone(), v40);
        (v41.clone(), v42.clone(), v43, v44, v45, v46, v47.clone(), v48.clone(), v49, v50.clone())
    };
    (v51.clone(), v52.clone(), v53, v54, v55, v56, v57.clone(), v58.clone(), v59, v60.clone())
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method30(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn renamed_within_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let v4: bool = v2 <= 1i32;
            if v4 {
                return false;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(50u64));
                let v5: i32 = v2.wrapping_sub(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v5);
                continue;
            }
        }
    }
}
fn method34(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_33(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method34(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn item_lines_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> (i32, i32) {
    loop {
        let v5: i32 = (v0.clone().len() as i32);
        let mut v6: bool = v5 <= v2;
        if v6 {
            return (v3, v4);
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let v9: bool = v8 < 0i32;
            let v10: i32 = if v9 {
                v5
            } else {
                v8
            };
            let v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v10 as usize).unwrap_or(""));
            let v12: bool = &*v11 == "";
            let v19: i32 = if v12 {
                0i32
            } else {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v13, v11)), v13));
                let v16: i32 = 0i32;
                let v17: i32 = 0i32;
                occurrences_27(v1.clone(), v15.clone(), v16, v17)
            };
            let v20: bool = v19 > 0i32;
            let v26: i32 = if v20 {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  wip="); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v21));
                let v23: Rc<str> = format_real_33(v19);
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
                let v25: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v24.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                v25
            } else {
                0i32
            };
            let v27: i32 = v10.wrapping_add(1i32);
            let v29: i32 = if v20 {
                let v28: i32 = v3.wrapping_add(1i32);
                v28
            } else {
                v3
            };
            let v30: i32 = v4.wrapping_add(v19);
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v27, v29, v30);
            continue;
        }
    }
}
fn method35(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: i32 = words_in_26(v2.clone());
                let v5: i32 = v1.wrapping_add(v4);
                (v0, v1) = (v3.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_36(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method34(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn wip_run_0(mut v0: Rc<str>) -> i32 {
    let v1: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v3, &*v3, &*v3].concat());
    let v5: bool = file_exists_1(v4.clone());
    let mut v6: bool = v5 == false;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile wip rejected: no backlog state at "); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
        let v9: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v8 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v9
    } else {
        let v11: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v4);
        let v13: Vec<u8> = v11.unwrap();
        let v14: Vec<u8> = method2(v13.clone());
        let v16: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v14);
        let v18: std::string::String = v16.unwrap();
        let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
        let v21: Rc<str> = cache_path_3(v0.clone());
        let v22: bool = file_exists_1(v21.clone());
        let v34: Rc<str> = if v22 {
            let v24: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v21);
            let v26: Vec<u8> = v24.unwrap();
            let v27: Vec<u8> = method2(v26.clone());
            let v29: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v27);
            let v31: std::string::String = v29.unwrap();
            let v33: Rc<str> = Rc::<str>::from(String::as_str(&v31));
            v33.clone()
        } else {
            v3.clone()
        };
        let v35: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v36: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v37: i32 = 0i32;
        let v38: i32 = 0i32;
        let v39: i32 = 0i32;
        let v40: i32 = 0i32;
        let v41: bool = false;
        let v42: i32 = 0i32;
        let v43: Rc<str> = harvest_roots_from_5(v20.clone(), v42, v3.clone());
        let v44: i32 = 0i32;
        let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = roots_walked_6(v35.clone(), v36.clone(), v37, v38, v39, v40, v34.clone(), v20.clone(), v41, v3.clone(), v43.clone(), v44);
        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
        let v56: Rc<str> = get_environment_variable_4(v55.clone());
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/kit/todo-cache"); } LIT.with(|lit| lit.clone()) };
        let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
        let v60: Result<(), std::io::Error> = std::fs::create_dir_all(&*v58);
        let v61: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v62: Rc<UH0> = method29(v45.clone(), v61.clone());
        let v63: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v64: i32 = 0i32;
        let v65: i32 = method30(v63.clone(), v62.clone(), v64);
        let v66: Rc<Vec<Rc<str>>> = Rc::new(v63.borrow().clone());
        let v67: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v66).as_ref().clone()));
        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v69: Rc<str> = Rc::<str>::from((*v67.borrow()).join(&*v68));
        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v70));
        let v72: bool = std::fs::write(&*v71, &*v69).is_ok();
        let v85: US5 = if v72 {
            let v73: i32 = 40i32;
            let v74: bool = renamed_within_31(v21.clone(), v71.clone(), v73);
            if v74 {
                let v75: bool = std::fs::read_to_string(&*v21).is_ok_and(|value| value == *v69);
                if v75 {
                    US5::US5_Replaced
                } else {
                    US5::US5_ReadbackMismatch
                }
            } else {
                let v79: bool = std::fs::remove_file(&*v71).is_ok() || !std::path::Path::new(&*v71).exists();
                if v79 {
                    US5::US5_RenameFailed
                } else {
                    US5::US5_RenameFailedTemporaryKept
                }
            }
        } else {
            US5::US5_TemporaryWriteFailed
        };
        let v86: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v87: i32 = 0i32;
        let v88: i32 = method30(v86.clone(), v46.clone(), v87);
        let v89: Rc<Vec<Rc<str>>> = Rc::new(v86.borrow().clone());
        let v90: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v89).as_ref().clone()));
        let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v92: Rc<str> = Rc::<str>::from((*v90.borrow()).join(&*v91));
        let v94: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v91, v92)), v91));
        let v95: Rc<str> = agile_task_census::kit_agile_unsettled_ids(&*v20);
        let v96: i32 = 0i32;
        let v97: i32 = 0i32;
        let v98: i32 = 0i32;
        let (mut v99, mut v100): (i32, i32) = item_lines_32(v95.clone(), v94.clone(), v96, v97, v98);
        let v101: i32 = 0i32;
        let v102: i32 = method35(v46.clone(), v101);
        let v103: i32 = v102.wrapping_sub(v100);
        let v104: Rc<str> = format_real_33(v103);
        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stale wip: "); } LIT.with(|lit| lit.clone()) };
        let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v104));
        let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" markers naming a Done or unknown item"); } LIT.with(|lit| lit.clone()) };
        let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v107));
        let v109: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v108.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        let v110: Rc<str> = format_real_33(v47);
        let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("untracked wip: "); } LIT.with(|lit| lit.clone()) };
        let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v110));
        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" markers without an ID"); } LIT.with(|lit| lit.clone()) };
        let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v113));
        let v115: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v114.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        let v116: Rc<str> = format_real_33(v99);
        let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile wip ok items="); } LIT.with(|lit| lit.clone()) };
        let v118: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v116));
        let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" markers="); } LIT.with(|lit| lit.clone()) };
        let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v119));
        let v121: Rc<str> = format_real_33(v100);
        let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v121));
        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" untracked="); } LIT.with(|lit| lit.clone()) };
        let v124: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v123));
        let v125: Rc<str> = format_real_33(v47);
        let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v124, v125));
        let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" scanned="); } LIT.with(|lit| lit.clone()) };
        let v128: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v127));
        let v129: Rc<str> = format_real_33(v48);
        let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v128, v129));
        let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cached="); } LIT.with(|lit| lit.clone()) };
        let v132: Rc<str> = Rc::<str>::from(format!("{}{}", v130, v131));
        let v133: Rc<str> = format_real_33(v49);
        let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v133));
        let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
        let v136: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v135));
        let v137: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v1);
        let v138: Rc<str> = format_real_36(v137);
        let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v138));
        let v140: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v139.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        v140
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 1i32;
        let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        wip_run_0(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_wip_run() -> i32 {
    closure0()()
}
