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
    US4_ContentScan,
    US4_ContentBinary,
    US4_ContentLarge,
    US4_ContentUnreadable,
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
pub struct Mut0 { pub l0: Rc<str> }
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn file_exists_2(mut v0: Rc<str>) -> bool {
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
fn method3(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn get_environment_variable_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn cache_path_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v2));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = get_environment_variable_5(v8);
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/kit/todo-cache/"); } LIT.with(|lit| lit.clone()) }));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v7));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) }));
    v12
}
fn harvest_roots_from_6(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v2;
        } else {
            let mut v6: i32 = v4.wrapping_add(19i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v0.get(v6 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v6 as usize) as i32).unwrap_or(-1);
            let v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
            let v10: bool = &*v2 == "";
            let v14: Rc<str> = if v10 {
                v9
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                concat5_1(v2, v11, v9, v12.clone(), v12.clone())
            };
            let v15: i32 = v8.wrapping_add(1i32);
            (v1, v2) = (v15, v14);
            continue;
        }
    }
}
fn cached_head_of_9(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let (mut v2, mut v3): (Rc<str>, Rc<str>) = agile_todo_cache::kit_agile_todo_head_of(&*v0, &*v1);
    (v2, v3)
}
fn changed_in_13(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Option<char> = char::from_u32(0u32);
    let v4: char = v2.unwrap();
    let v5: Rc<str> = Rc::<str>::from(format!("{}", v4));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v1));
    let v7: Option<char> = char::from_u32(0u32);
    let v9: char = v7.unwrap();
    let v10: Rc<str> = Rc::<str>::from(format!("{}", v9));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v10));
    let v12: bool = v0.contains(&*v11);
    v12
}
fn last_byte_of_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            (v2, v3) = (v6, v4);
            continue;
        }
    }
}
fn spiral_sibling_relative_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = 0i32;
    let v3: i32 = -1i32;
    let v4: i32 = last_byte_of_15(v0.clone(), v1, v2, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = v4.wrapping_add(1i32);
    let v7: i32 = -1i32;
    let v8: i32 = last_byte_of_15(v0.clone(), v5, v6, v7);
    let v9: bool = v8 > v6;
    let v11: Rc<str> = if v9 {
        let v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v8 as usize).unwrap_or(""));
        v10
    } else {
        v0
    };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) }));
    v12
}
fn unchanged_12(mut v0: &Rc<UH0>, mut v1: &Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>) -> bool {
    let v13: bool = if v8 {
        let v11: bool = changed_in_13(v9.clone(), v10.clone());
        let v12: bool = v11 == false;
        v12
    } else {
        false
    };
    if v13 {
        let v14: Rc<str> = spiral_sibling_relative_14(v10);
        let v15: bool = changed_in_13(v9, v14);
        let v16: bool = v15 == false;
        v16
    } else {
        false
    }
}
fn cached_row_of_16(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US0 {
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
fn cache_row_19(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: Rc<str>, mut v4: i32) -> Rc<str> {
    let v5: Rc<str> = agile_todo_cache::kit_agile_todo_cache_row(&*v0, v1, v2, &*v3, v4);
    v5
}
fn recorded_18(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v15: Rc<str> = cache_row_19(v10, v11, v12, v13.clone(), v14);
    let v16: i32 = v2.wrapping_add(v14);
    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v0));
    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v1));
    (v17, v18, v16, v3, v4, v5, v6, v7, v8, v9)
}
fn hit_recorded_17(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: i32, mut v15: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = recorded_18(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14);
    let v26: i32 = v4.wrapping_add(1i32);
    (v16, v17, v18, v19, v26, v15, v22, v23, v24, v25)
}
fn skipped_row_21(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = agile_todo_cache::kit_agile_todo_skipped_row(&*v0);
    v1
}
fn skipped_20(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v11: Rc<str> = skipped_row_21(v10);
    let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v0));
    (v12, v1, v2, v3, v4, v5, v6, v7, v8, v9)
}
fn path_verdict_of_code_23(mut v0: i32) -> US1 {
    let v1: bool = 0i32 < v0;
    if v1 {
        let mut v2: bool = v0 == 1i32;
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
fn findings_of_25(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, i32) {
    let v4: i32 = match v0 {
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
fn ids_in_26(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = agile_todo_cache::kit_agile_todo_ids_in(&*v0);
    v1
}
fn occurrences_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = (v1.clone().len() as i32);
            let v7: i32 = v4.wrapping_add(v6);
            let v8: i32 = v3.wrapping_add(1i32);
            (v2, v3) = (v7, v8);
            continue;
        }
    }
}
fn words_in_27(mut v0: Rc<str>) -> i32 {
    let v1: bool = &*v0 == "";
    if v1 {
        0i32
    } else {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v3: i32 = 0i32;
        let v4: i32 = 0i32;
        let v5: i32 = occurrences_28(v0, v2, v3, v4);
        let mut v6: i32 = v5.wrapping_add(1i32);
        v6
    }
}
fn file_read_24(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: US2, mut v11: Rc<str>, mut v12: u64, mut v13: u64) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let (mut v14, mut v15): (bool, Rc<str>) = match std::fs::read(&*v11) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
    let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v14 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v17: i32 = 0i32;
        let (mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = recorded_18(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v11, v12, v13, v16, v17);
        let v28: i32 = v3.wrapping_add(1i32);
        (v18, v19, v20, v28, v22, v23, v24, v25, v26, v27)
    } else {
        let (mut v29, mut v30): (Rc<str>, i32) = findings_of_25(v10, v7.clone(), v15, v11.clone());
        let v31: Rc<str> = ids_in_26(v29);
        let v32: i32 = words_in_27(v31.clone());
        let v33: i32 = v30.wrapping_sub(v32);
        let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = recorded_18(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v11, v12, v13, v31, v33);
        let v44: i32 = v3.wrapping_add(1i32);
        (v34, v35, v36, v44, v38, v39, v40, v41, v42, v43)
    };
    (v45, v46, v47, v48, v49, v50, v51, v52, v53, v54)
}
fn file_scanned_22(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v12: bool = std::path::Path::new(&*v10).with_extension("spi").is_file();
    let v13: i32 = agile_todo_paths::kit_agile_todo_path_verdict(&*v11, v12);
    let v14: US1 = path_verdict_of_code_23(v13);
    let (mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match v14 {
        US1::US1_ScanPath(v15) => {
            let v15: US2 = v15;
            let v16: u64 = std::fs::metadata(&*v10).map(|metadata| metadata.len()).unwrap_or(0);
            let v17: bool = v16 > 33554432u64;
            let v20: US4 = if v17 {
                US4::US4_ContentLarge
            } else {
                US4::US4_ContentScan
            };
            match v20 {
                US4::US4_ContentLarge => {
                    skipped_20(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
                }
                _ => {
                    let v31: u64 = std::fs::metadata(&*v10).and_then(|metadata| metadata.modified()).ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
                    let v32: US0 = cached_row_of_16(v6.clone(), v5, v10.clone());
                    let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match v32 {
                        US0::US0_RowHit(v33, v34, v35, v36, v37) => {
                            let v33: u64 = v33;
                            let v34: u64 = v34;
                            let v35: Rc<str> = v35;
                            let v36: i32 = v36;
                            let v37: i32 = v37;
                            let v38: bool = v33 == v16;
                            let v40: bool = if v38 {
                                let v39: bool = v34 == v31;
                                v39
                            } else {
                                false
                            };
                            if v40 {
                                let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = hit_recorded_17(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v16, v31, v35, v36, v37);
                                (v41, v42, v43, v44, v45, v46, v47, v48, v49, v50)
                            } else {
                                let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = file_read_24(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v15, v10, v16, v31);
                                (v51, v52, v53, v54, v55, v56, v57, v58, v59, v60)
                            }
                        }
                        _ => {
                            let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = file_read_24(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v15, v10, v16, v31);
                            (v71, v72, v73, v74, v75, v76, v77, v78, v79, v80)
                        }
                    };
                    (v81, v82, v83, v84, v85, v86, v87, v88, v89, v90)
                }
            }
        }
        _ => {
            let (mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = skipped_20(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10);
            (v101, v102, v103, v104, v105, v106, v107, v108, v109, v110)
        }
    };
    (v111, v112, v113, v114, v115, v116, v117, v118, v119, v120)
}
fn file_walked_11(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11.clone()));
    let v14: bool = unchanged_12(&v0, &v1, v2, v3, v4, v5, &v6, &v7, v8, v9.clone(), v11.clone());
    let v17: US0 = if v14 {
        cached_row_of_16(v6.clone(), v5, v13.clone())
    } else {
        US0::US0_RowMiss
    };
    let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match &v17 {
        US0::US0_RowHit(v18, v19, v20, v21, v22) => {
            let v18: u64 = *v18;
            let v19: u64 = *v19;
            let v20: Rc<str> = v20.clone();
            let v21: i32 = *v21;
            let v22: i32 = *v22;
            hit_recorded_17(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v13, v18, v19, v20, v21, v22)
        }
        _ => {
            let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match &v17 {
                US0::US0_RowSkip(v33) => {
                    let v33: i32 = *v33;
                    let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = skipped_20(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v13);
                    (v34, v35, v36, v37, v38, v33, v40, v41, v42, v43)
                }
                _ => {
                    let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = match v17 {
                        US0::US0_RowMiss => {
                            file_scanned_22(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v13, v11)
                        }
                        _ => unreachable!(),
                    };
                    (v54, v55, v56, v57, v58, v59, v60, v61, v62, v63)
                }
            };
            (v64, v65, v66, v67, v68, v69, v70, v71, v72, v73)
        }
    };
    (v74, v75, v76, v77, v78, v79, v80, v81, v82, v83)
}
fn listed_10(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v13: i32 = (v11.clone().len() as i32);
    let v14: bool = v13 <= v12;
    let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v14 {
        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
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
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
        } else {
            let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = file_walked_11(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10.clone(), v22);
            (v24, v25, v26, v27, v28, v29, v30, v31, v32, v33)
        };
        let v44: i32 = v21.wrapping_add(1i32);
        let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = listed_10(v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v10, v11, v44);
        (v45, v46, v47, v48, v49, v50, v51, v52, v53, v54)
    };
    (v55, v56, v57, v58, v59, v60, v61, v62, v63, v64)
}
fn head_row_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = agile_todo_cache::kit_agile_todo_head_row(&*v0, &*v1, &*v2);
    v3
}
fn root_walked_8(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: &Rc<str>, mut v10: Rc<str>) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let (mut v11, mut v12): (bool, Rc<str>) = agile_todo_sources::kit_agile_todo_sources(&*v10);
    let v13: Rc<str> = agile_todo_sources::kit_agile_todo_git_head(&*v10);
    let (mut v14, mut v15): (Rc<str>, Rc<str>) = cached_head_of_9(v6.clone(), v10.clone());
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
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v19.clone()));
        let v32: Option<char> = char::from_u32(0u32);
        let v34: char = v32.unwrap();
        let v35: Rc<str> = Rc::<str>::from(format!("{}", v34));
        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v35));
        let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v15));
        let v38: Option<char> = char::from_u32(0u32);
        let v40: char = v38.unwrap();
        let v41: Rc<str> = Rc::<str>::from(format!("{}", v40));
        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v41));
        v42
    };
    let v44: bool = &*v43 != "";
    let (mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v11 {
        let v45: i32 = 0i32;
        let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = listed_10(v0, v1, v2, v3, v4, v5, v6, v7, v44, v43, v10.clone(), v12, v45);
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
        v56
    } else {
        let v68: Rc<str> = head_row_29(v10, v13, v19);
        Rc::new(UH0::UH0_Cons(v68, v56))
    };
    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v70, v57, v58, v59, v60, v61, v62, v63, false, v71)
}
fn roots_walked_7(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: i32) -> (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) {
    let v12: i32 = (v10.clone().len() as i32);
    let v13: bool = v12 <= v11;
    let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = if v13 {
        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
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
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
        } else {
            let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = root_walked_8(v0, v1, v2, v3, v4, v5, v6, v7, v8, &v9, v18);
            (v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
        };
        let v40: i32 = v17.wrapping_add(1i32);
        let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = roots_walked_7(v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v10, v40);
        (v41, v42, v43, v44, v45, v46, v47, v48, v49, v50)
    };
    (v51, v52, v53, v54, v55, v56, v57, v58, v59, v60)
}
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method31(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn renamed_within_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
                v2 = v5;
                continue;
            }
        }
    }
}
fn method35(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_34(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method35(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn item_lines_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> (i32, i32) {
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
            let v18: i32 = if v12 {
                0i32
            } else {
                let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }, v11.clone()));
                let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                let v15: i32 = 0i32;
                let v16: i32 = 0i32;
                occurrences_28(v1.clone(), v14, v15, v16)
            };
            let v19: bool = v18 > 0i32;
            let v24: i32 = if v19 {
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  wip="); } LIT.with(|lit| lit.clone()) }));
                let v21: Rc<str> = format_real_34(v18);
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                let v23: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v22.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                v23
            } else {
                0i32
            };
            let v25: i32 = v10.wrapping_add(1i32);
            let v27: i32 = if v19 {
                let v26: i32 = v3.wrapping_add(1i32);
                v26
            } else {
                v3
            };
            let v28: i32 = v4.wrapping_add(v18);
            (v2, v3, v4) = (v25, v27, v28);
            continue;
        }
    }
}
fn method36(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: i32 = words_in_27(v2);
                let v5: i32 = v1.wrapping_add(v4);
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_37(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method35(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn wip_run_0(mut v0: Rc<str>) -> i32 {
    let v1: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = concat5_1(v0.clone(), v2, v3.clone(), v3.clone(), v3.clone());
    let v5: bool = file_exists_2(v4.clone());
    let mut v6: bool = v5 == false;
    if v6 {
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile wip rejected: no backlog state at "); } LIT.with(|lit| lit.clone()) }, v4));
        let v8: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v7 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v8
    } else {
        let v10: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v4);
        let v12: Vec<u8> = v10.unwrap();
        let v13: Vec<u8> = method3(v12);
        let v15: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v13);
        let v17: std::string::String = v15.unwrap();
        let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
        let v20: Rc<str> = cache_path_4(v0);
        let v21: bool = file_exists_2(v20.clone());
        let v33: Rc<str> = if v21 {
            let v23: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v20);
            let v25: Vec<u8> = v23.unwrap();
            let v26: Vec<u8> = method3(v25);
            let v28: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v26);
            let v30: std::string::String = v28.unwrap();
            let v32: Rc<str> = Rc::<str>::from(String::as_str(&v30));
            v32
        } else {
            v3.clone()
        };
        let v34: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v35: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v36: i32 = 0i32;
        let v37: i32 = 0i32;
        let v38: i32 = 0i32;
        let v39: i32 = 0i32;
        let v40: bool = false;
        let v41: i32 = 0i32;
        let v42: Rc<str> = harvest_roots_from_6(v19.clone(), v41, v3.clone());
        let v43: i32 = 0i32;
        let (mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (Rc<UH0>, Rc<UH0>, i32, i32, i32, i32, Rc<str>, Rc<str>, bool, Rc<str>) = roots_walked_7(v34, v35, v36, v37, v38, v39, v33, v19.clone(), v40, v3, v42, v43);
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
        let v55: Rc<str> = get_environment_variable_5(v54);
        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/kit/todo-cache"); } LIT.with(|lit| lit.clone()) }));
        let v58: Result<(), std::io::Error> = std::fs::create_dir_all(&*v56);
        let v59: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v60: Rc<UH0> = method30(v44, v59);
        let v61: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v62: i32 = 0i32;
        let v63: i32 = method31(v61.clone(), v60, v62);
        let v64: Rc<Vec<Rc<str>>> = Rc::new(v61.borrow().clone());
        let v65: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v64).as_ref().clone()));
        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v67: Rc<str> = Rc::<str>::from((*v65.borrow()).join(&*v66));
        let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v20.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
        let v69: bool = std::fs::write(&*v68, &*v67).is_ok();
        let v82: US5 = if v69 {
            let v70: i32 = 40i32;
            let v71: bool = renamed_within_32(v20.clone(), v68.clone(), v70);
            if v71 {
                let v72: bool = std::fs::read_to_string(&*v20).is_ok_and(|value| value == *v67);
                if v72 {
                    US5::US5_Replaced
                } else {
                    US5::US5_ReadbackMismatch
                }
            } else {
                let v76: bool = std::fs::remove_file(&*v68).is_ok() || !std::path::Path::new(&*v68).exists();
                if v76 {
                    US5::US5_RenameFailed
                } else {
                    US5::US5_RenameFailedTemporaryKept
                }
            }
        } else {
            US5::US5_TemporaryWriteFailed
        };
        let v83: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v84: i32 = 0i32;
        let v85: i32 = method31(v83.clone(), v45.clone(), v84);
        let v86: Rc<Vec<Rc<str>>> = Rc::new(v83.borrow().clone());
        let v87: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v86).as_ref().clone()));
        let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v89: Rc<str> = Rc::<str>::from((*v87.borrow()).join(&*v88));
        let v90: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }, v89));
        let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v90, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
        let v92: Rc<str> = agile_task_census::kit_agile_unsettled_ids(&*v19);
        let v93: i32 = 0i32;
        let v94: i32 = 0i32;
        let v95: i32 = 0i32;
        let (mut v96, mut v97): (i32, i32) = item_lines_33(v92, v91, v93, v94, v95);
        let v98: i32 = 0i32;
        let v99: i32 = method36(v45, v98);
        let v100: i32 = v99.wrapping_sub(v97);
        let v101: Rc<str> = format_real_34(v100);
        let v102: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stale wip: "); } LIT.with(|lit| lit.clone()) }, v101));
        let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v102, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" markers naming a Done or unknown item"); } LIT.with(|lit| lit.clone()) }));
        let v104: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v103.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        let v105: Rc<str> = format_real_34(v46);
        let v106: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("untracked wip: "); } LIT.with(|lit| lit.clone()) }, v105));
        let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v106, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" markers without an ID"); } LIT.with(|lit| lit.clone()) }));
        let v108: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v107.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        let v109: Rc<str> = format_real_34(v96);
        let v110: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile wip ok items="); } LIT.with(|lit| lit.clone()) }, v109));
        let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v110, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" markers="); } LIT.with(|lit| lit.clone()) }));
        let v112: Rc<str> = format_real_34(v97);
        let v113: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v112));
        let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v113, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" untracked="); } LIT.with(|lit| lit.clone()) }));
        let v115: Rc<str> = format_real_34(v46);
        let v116: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v115));
        let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v116, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" scanned="); } LIT.with(|lit| lit.clone()) }));
        let v118: Rc<str> = format_real_34(v47);
        let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v118));
        let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v119, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cached="); } LIT.with(|lit| lit.clone()) }));
        let v121: Rc<str> = format_real_34(v48);
        let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v121));
        let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v122, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
        let v124: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v1);
        let v125: Rc<str> = format_real_37(v124);
        let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v125));
        let v127: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v126.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        v127
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 1i32;
        let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        wip_run_0(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_wip_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
