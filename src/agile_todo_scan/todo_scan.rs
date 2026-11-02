#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_RegularFile,
    US0_Directory,
    US0_LinkedEntry,
    US0_AbsentEntry,
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
    US5_AllTodos,
    US5_TaggedTodos,
    US5_TodosOf(Rc<str>),
}
fn command_line_argument_count_2() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_1() -> i32 {
    let v0: i32 = command_line_argument_count_2();
    let v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn concat5_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn directory_exists_4(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn file_exists_5(mut v0: Rc<str>) -> bool {
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
fn scan_start_6(mut v0: bool, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, v0, v1, false, v4, v2, v3, 0i32, 0i32, 0i32)
}
fn path_verdict_of_code_10(mut v0: i32) -> US1 {
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
                                            let mut v20: bool = v0 == 10i32;
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
fn counted_path_11(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: US1) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    let v12: i32 = v0.wrapping_add(1i32);
    let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
        US1::US1_ScanPath(v13) => {
            let v13: US2 = v13.clone();
            (v12, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
        }
        _ => {
            let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                US1::US1_SkipPath(v14) => {
                    let v14: US3 = v14.clone();
                    match &v14 {
                        US3::US3_SkipNestedRepository => {
                            let v15: i32 = v3.wrapping_add(1i32);
                            (v12, v1, v2, v15, v4, v5, v6, v7, v8, v9, v10)
                        }
                        _ => {
                            let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v14 {
                                US3::US3_SkipVendorDirectory => {
                                    let v16: i32 = v4.wrapping_add(1i32);
                                    (v12, v1, v2, v3, v16, v5, v6, v7, v8, v9, v10)
                                }
                                _ => {
                                    let (mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v14 {
                                        US3::US3_SkipUnknownSyntax => {
                                            let v17: i32 = v5.wrapping_add(1i32);
                                            (v12, v1, v2, v3, v4, v17, v6, v7, v8, v9, v10)
                                        }
                                        _ => {
                                            let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v14 {
                                                US3::US3_SkipGeneratedOutput => {
                                                    let v18: i32 = v6.wrapping_add(1i32);
                                                    (v12, v1, v2, v3, v4, v5, v18, v7, v8, v9, v10)
                                                }
                                                _ => {
                                                    let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v14 {
                                                        US3::US3_SkipLinkedEntry => {
                                                            let v19: i32 = v9.wrapping_add(1i32);
                                                            (v12, v1, v2, v3, v4, v5, v6, v7, v8, v19, v10)
                                                        }
                                                        _ => unreachable!(),
                                                    };
                                                    (v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30)
                                                }
                                            };
                                            (v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41)
                                        }
                                    };
                                    (v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52)
                                }
                            };
                            (v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85)
        }
    };
    (v86, v87, v88, v89, v90, v91, v92, v93, v94, v95, v96)
}
fn counted_content_12(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: US4) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    let (mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
        US4::US4_ScanContent => {
            let v12: i32 = v1.wrapping_add(1i32);
            (v0, v12, v2, v3, v4, v5, v6, v7, v8, v9, v10)
        }
        _ => {
            let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                US4::US4_SkipBinary => {
                    let v13: i32 = v7.wrapping_add(1i32);
                    (v0, v1, v2, v3, v4, v5, v6, v13, v8, v9, v10)
                }
                _ => {
                    let (mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                        US4::US4_SkipLarge => {
                            let v14: i32 = v8.wrapping_add(1i32);
                            (v0, v1, v2, v3, v4, v5, v6, v7, v14, v9, v10)
                        }
                        _ => {
                            let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = match &v11 {
                                US4::US4_SkipUnreadable => {
                                    let v15: i32 = v10.wrapping_add(1i32);
                                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v15)
                                }
                                _ => unreachable!(),
                            };
                            (v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
                        }
                    };
                    (v27, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37)
                }
            };
            (v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48)
        }
    };
    (v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59)
}
fn counted_todos_13(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
    let v12: i32 = v2.wrapping_add(v11);
    (v0, v1, v12, v3, v4, v5, v6, v7, v8, v9, v10)
}
fn widened_14(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>, mut v21: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let v22: i32 = (v20.clone().len() as i32);
    let v23: bool = v22 <= v21;
    let (mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v23 {
        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
    } else {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v25: i32 = v20.get(v21 as usize..).and_then(|rest| rest.find(&*v24)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
        let v26: bool = v25 < 0i32;
        let v27: i32 = if v26 {
            v22
        } else {
            v25
        };
        let v28: bool = v11 || v12;
        let v34: i32 = if v28 {
            0i32
        } else {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wide (over 120 columns, the Spiral formatter rewraps it): "); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = std::rc::Rc::<str>::from(v20.get(v21 as usize..v27 as usize).unwrap_or(""));
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = concat5_3(v29.clone(), v30.clone(), v31.clone(), v31.clone(), v31.clone());
            let v33: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v32.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v33
        };
        let v35: i32 = v19.wrapping_add(1i32);
        let v36: i32 = v27.wrapping_add(1i32);
        let (mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = widened_14(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v35, v20.clone(), v36);
        (v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51.clone(), v52.clone(), v53.clone(), v54, v55, v56)
    };
    (v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70, v71.clone(), v72.clone(), v73.clone(), v74, v75, v76)
}
fn scanned_file_9(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>, mut v21: Rc<str>, mut v22: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let v23: bool = std::fs::symlink_metadata(&*v21).map(|metadata| metadata.file_type().is_symlink()).unwrap_or(false);
    let v32: US0 = if v23 {
        US0::US0_LinkedEntry
    } else {
        let v25: bool = directory_exists_4(v21.clone());
        if v25 {
            US0::US0_Directory
        } else {
            let v27: bool = file_exists_5(v21.clone());
            if v27 {
                US0::US0_RegularFile
            } else {
                US0::US0_AbsentEntry
            }
        }
    };
    let (mut v367, mut v368, mut v369, mut v370, mut v371, mut v372, mut v373, mut v374, mut v375, mut v376, mut v377, mut v378, mut v379, mut v380, mut v381, mut v382, mut v383, mut v384, mut v385, mut v386): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v32 {
        US0::US0_RegularFile => {
            let v33: bool = std::path::Path::new(&*v21).with_extension("spi").is_file();
            let v34: i32 = agile_todo_paths::kit_agile_todo_path_verdict(&*v22, v33);
            let v35: US1 = path_verdict_of_code_10(v34);
            let (mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_11(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v35.clone());
            match &v35 {
                US1::US1_ScanPath(v47) => {
                    let v47: US2 = v47.clone();
                    let v48: u64 = std::fs::metadata(&*v21).map(|metadata| metadata.len()).unwrap_or(0);
                    let v49: bool = v48 > 33554432u64;
                    let v52: US4 = if v49 {
                        US4::US4_SkipLarge
                    } else {
                        US4::US4_ScanContent
                    };
                    match &v52 {
                        US4::US4_SkipLarge => {
                            let v53: US4 = US4::US4_SkipLarge;
                            let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_12(v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v53.clone());
                            (v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                        }
                        _ => {
                            let (mut v65, mut v66): (bool, Rc<str>) = match std::fs::read(&*v21) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                            let v69: US4 = if v65 {
                                US4::US4_SkipBinary
                            } else {
                                US4::US4_ScanContent
                            };
                            let (mut v228, mut v229, mut v230, mut v231, mut v232, mut v233, mut v234, mut v235, mut v236, mut v237, mut v238, mut v239, mut v240, mut v241, mut v242, mut v243, mut v244, mut v245, mut v246, mut v247): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v69 {
                                US4::US4_ScanContent => {
                                    let v70: bool = &*v16 == "";
                                    let v76: US5 = if v70 {
                                        US5::US5_AllTodos
                                    } else {
                                        let v72: bool = &*v16 == "*";
                                        if v72 {
                                            US5::US5_TaggedTodos
                                        } else {
                                            US5::US5_TodosOf(v16.clone())
                                        }
                                    };
                                    let v80: bool = match &v76 {
                                        US5::US5_TodosOf(v77) => {
                                            let v77: Rc<str> = v77.clone();
                                            let v78: i32 = v66.get(0i32 as usize..).and_then(|rest| rest.find(&*v77)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
                                            let v79: bool = v78 >= 0i32;
                                            v79
                                        }
                                        _ => {
                                            true
                                        }
                                    };
                                    let v81: bool = v80 == false;
                                    if v81 {
                                        let v82: US4 = US4::US4_ScanContent;
                                        let (mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_12(v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v82.clone());
                                        (v83, v84, v85, v86, v87, v88, v89, v90, v91, v92, v93, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                                    } else {
                                        let v94: i32 = match &v47 {
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
                                        let (mut v95, mut v96, mut v97): (Rc<str>, i32, Rc<str>) = agile_todo_lines::kit_agile_todo_findings(v94, &*v16, &*v15, &*v66, &*v20, v11);
                                        let v98: US4 = US4::US4_ScanContent;
                                        let (mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_12(v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v98.clone());
                                        let (mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_todos_13(v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v96);
                                        let v121: i32 = 0i32;
                                        let (mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = widened_14(v110, v111, v112, v113, v114, v115, v116, v117, v118, v119, v120, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v97.clone(), v121);
                                        let v142: bool = v96 <= 0i32;
                                        let (mut v177, mut v178, mut v179, mut v180, mut v181, mut v182, mut v183, mut v184, mut v185, mut v186, mut v187, mut v188, mut v189, mut v190, mut v191, mut v192, mut v193, mut v194, mut v195, mut v196): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v142 {
                                            (v122, v123, v124, v125, v126, v127, v128, v129, v130, v131, v132, v133, v134, v135, v136.clone(), v137.clone(), v138.clone(), v139, v140, v141)
                                        } else {
                                            let (mut v152, mut v153, mut v154, mut v155, mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165, mut v166, mut v167, mut v168, mut v169, mut v170, mut v171): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v134 {
                                                let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                let v145: Rc<str> = concat5_3(v136.clone(), v95.clone(), v143.clone(), v144.clone(), v144.clone());
                                                (v122, v123, v124, v125, v126, v127, v128, v129, v130, v131, v132, v133, v134, true, v145.clone(), v137.clone(), v138.clone(), v139, v140, v141)
                                            } else {
                                                let v146: bool = v133 && v135;
                                                let v150: Rc<str> = if v146 {
                                                    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                    let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    concat5_3(v147.clone(), v95.clone(), v148.clone(), v148.clone(), v148.clone())
                                                } else {
                                                    v95.clone()
                                                };
                                                let v151: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v150.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                                (v122, v123, v124, v125, v126, v127, v128, v129, v130, v131, v132, v133, v134, true, v136.clone(), v137.clone(), v138.clone(), v139, v140, v141)
                                            };
                                            let v172: bool = v163 || v164;
                                            let (mut v173, mut v174): (i32, i32) = agile_todo_standing::kit_agile_todo_standings(&*v167, &*v95, v172);
                                            let v175: i32 = v169.wrapping_add(v173);
                                            let v176: i32 = v170.wrapping_add(v174);
                                            (v152, v153, v154, v155, v156, v157, v158, v159, v160, v161, v162, v163, v164, v165, v166.clone(), v167.clone(), v168.clone(), v175, v176, v171)
                                        };
                                        (v177, v178, v179, v180, v181, v182, v183, v184, v185, v186, v187, v188, v189, v190, v191.clone(), v192.clone(), v193.clone(), v194, v195, v196)
                                    }
                                }
                                _ => {
                                    let (mut v217, mut v218, mut v219, mut v220, mut v221, mut v222, mut v223, mut v224, mut v225, mut v226, mut v227): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_12(v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v69.clone());
                                    (v217, v218, v219, v220, v221, v222, v223, v224, v225, v226, v227, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                                }
                            };
                            (v228, v229, v230, v231, v232, v233, v234, v235, v236, v237, v238, v239, v240, v241, v242.clone(), v243.clone(), v244.clone(), v245, v246, v247)
                        }
                    }
                }
                _ => {
                    (v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                }
            }
        }
        _ => {
            let (mut v347, mut v348, mut v349, mut v350, mut v351, mut v352, mut v353, mut v354, mut v355, mut v356, mut v357, mut v358, mut v359, mut v360, mut v361, mut v362, mut v363, mut v364, mut v365, mut v366): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v32 {
                US0::US0_LinkedEntry => {
                    let v288: US3 = US3::US3_SkipLinkedEntry;
                    let v289: US1 = US1::US1_SkipPath(v288.clone());
                    let (mut v290, mut v291, mut v292, mut v293, mut v294, mut v295, mut v296, mut v297, mut v298, mut v299, mut v300): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_11(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v289.clone());
                    (v290, v291, v292, v293, v294, v295, v296, v297, v298, v299, v300, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                }
                _ => {
                    let v301: i32 = agile_todo_paths::kit_agile_todo_path_verdict(&*v22, false);
                    let v302: US1 = path_verdict_of_code_10(v301);
                    let (mut v303, mut v304, mut v305, mut v306, mut v307, mut v308, mut v309, mut v310, mut v311, mut v312, mut v313): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_path_11(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v302.clone());
                    let (mut v327, mut v328, mut v329, mut v330, mut v331, mut v332, mut v333, mut v334, mut v335, mut v336, mut v337, mut v338, mut v339, mut v340, mut v341, mut v342, mut v343, mut v344, mut v345, mut v346): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v302 {
                        US1::US1_ScanPath(v314) => {
                            let v314: US2 = v314.clone();
                            let v315: US4 = US4::US4_SkipUnreadable;
                            let (mut v316, mut v317, mut v318, mut v319, mut v320, mut v321, mut v322, mut v323, mut v324, mut v325, mut v326): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = counted_content_12(v303, v304, v305, v306, v307, v308, v309, v310, v311, v312, v313, v315.clone());
                            (v316, v317, v318, v319, v320, v321, v322, v323, v324, v325, v326, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                        }
                        _ => {
                            (v303, v304, v305, v306, v307, v308, v309, v310, v311, v312, v313, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                        }
                    };
                    (v327, v328, v329, v330, v331, v332, v333, v334, v335, v336, v337, v338, v339, v340, v341.clone(), v342.clone(), v343.clone(), v344, v345, v346)
                }
            };
            (v347, v348, v349, v350, v351, v352, v353, v354, v355, v356, v357, v358, v359, v360, v361.clone(), v362.clone(), v363.clone(), v364, v365, v366)
        }
    };
    (v367, v368, v369, v370, v371, v372, v373, v374, v375, v376, v377, v378, v379, v380, v381.clone(), v382.clone(), v383.clone(), v384, v385, v386)
}
fn listed_files_15(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>, mut v21: Rc<str>, mut v22: Rc<str>, mut v23: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let v24: i32 = (v22.clone().len() as i32);
    let v25: bool = v24 <= v23;
    let (mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v25 {
        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
    } else {
        let v26: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
        let v27: i32 = v22.get(v23 as usize..).and_then(|rest| rest.find(&*v26)).map(|found| (found + v23 as usize) as i32).unwrap_or(-1);
        let v28: bool = v27 < 0i32;
        let v29: i32 = if v28 {
            v24
        } else {
            v27
        };
        let v30: Rc<str> = std::rc::Rc::<str>::from(v22.get(v23 as usize..v29 as usize).unwrap_or(""));
        let v31: i32 = v29.wrapping_add(1i32);
        let v32: bool = &*v30 == "";
        let (mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = if v32 {
            let (mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = listed_files_15(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v20.clone(), v21.clone(), v22.clone(), v31);
            (v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47.clone(), v48.clone(), v49.clone(), v50, v51, v52)
        } else {
            let v53: bool = &*v21 == "";
            let v61: Rc<str> = if v53 {
                v30.clone()
            } else {
                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v55: bool = v21.ends_with(&*v54);
                if v55 {
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_3(v21.clone(), v30.clone(), v56.clone(), v56.clone(), v56.clone())
                } else {
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_3(v21.clone(), v54.clone(), v30.clone(), v58.clone(), v58.clone())
                }
            };
            let v62: bool = &*v20 == "";
            let v70: Rc<str> = if v62 {
                v30.clone()
            } else {
                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v64: bool = v20.ends_with(&*v63);
                if v64 {
                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_3(v20.clone(), v30.clone(), v65.clone(), v65.clone(), v65.clone())
                } else {
                    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_3(v20.clone(), v63.clone(), v30.clone(), v67.clone(), v67.clone())
                }
            };
            let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scanned_file_9(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v61.clone(), v70.clone(), v30.clone());
            let (mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = listed_files_15(v71, v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85.clone(), v86.clone(), v87.clone(), v88, v89, v90, v20.clone(), v21.clone(), v22.clone(), v31);
            (v91, v92, v93, v94, v95, v96, v97, v98, v99, v100, v101, v102, v103, v104, v105.clone(), v106.clone(), v107.clone(), v108, v109, v110)
        };
        (v111, v112, v113, v114, v115, v116, v117, v118, v119, v120, v121, v122, v123, v124, v125.clone(), v126.clone(), v127.clone(), v128, v129, v130)
    };
    (v131, v132, v133, v134, v135, v136, v137, v138, v139, v140, v141, v142, v143, v144, v145.clone(), v146.clone(), v147.clone(), v148, v149, v150)
}
fn scanned_root_8(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v22));
    let v24: bool = std::fs::symlink_metadata(&*v20).map(|metadata| metadata.file_type().is_symlink()).unwrap_or(false);
    let v33: US0 = if v24 {
        US0::US0_LinkedEntry
    } else {
        let v26: bool = directory_exists_4(v20.clone());
        if v26 {
            US0::US0_Directory
        } else {
            let v28: bool = file_exists_5(v20.clone());
            if v28 {
                US0::US0_RegularFile
            } else {
                US0::US0_AbsentEntry
            }
        }
    };
    let (mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v33 {
        US0::US0_RegularFile => {
            let v34: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v22));
            scanned_file_9(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v23.clone(), v20.clone(), v34.clone())
        }
        _ => {
            let (mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v33 {
                US0::US0_Directory => {
                    let (mut v55, mut v56): (bool, Rc<str>) = agile_todo_sources::kit_agile_todo_sources(&*v20);
                    if v55 {
                        let v57: i32 = 0i32;
                        let (mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = listed_files_15(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v20.clone(), v23.clone(), v56.clone(), v57);
                        (v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70, v71, v72.clone(), v73.clone(), v74.clone(), v75, v76, v77)
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                    }
                }
                _ => {
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19)
                }
            };
            (v98, v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, v111, v112.clone(), v113.clone(), v114.clone(), v115, v116, v117)
        }
    };
    (v118, v119, v120, v121, v122, v123, v124, v125, v126, v127, v128, v129, v130, v131, v132.clone(), v133.clone(), v134.clone(), v135, v136, v137)
}
fn scanned_arguments_7(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: i32, mut v21: i32, mut v22: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) {
    loop {
        let v23: bool = v21 <= v20;
        if v23 {
            return (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v22);
        } else {
            let v24: Rc<str> = usize::try_from(v20).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v25: bool = &*v24 == "--id";
            let v30: i32 = if v25 {
                2i32
            } else {
                let v26: bool = &*v24 == "--tagged";
                let v28: bool = if v26 {
                    true
                } else {
                    let v27: bool = &*v24 == "--json";
                    v27
                };
                if v28 {
                    1i32
                } else {
                    0i32
                }
            };
            let v31: bool = 0i32 < v30;
            if v31 {
                let v32: i32 = v20.wrapping_add(v30);
                v20 = v32;
                continue;
            } else {
                let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scanned_root_8(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v24.clone());
                let v74: i32 = v20.wrapping_add(1i32);
                let v75: i32 = v22.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v22) = (v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68.clone(), v69.clone(), v70.clone(), v71, v72, v73, v74, v75);
                continue;
            }
        }
    }
}
fn scanned_listed_roots_17(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>, mut v21: i32, mut v22: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) {
    loop {
        let v23: i32 = (v20.clone().len() as i32);
        let v24: bool = v23 <= v21;
        if v24 {
            return (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v22);
        } else {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v26: i32 = v20.get(v21 as usize..).and_then(|rest| rest.find(&*v25)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
            let v27: bool = v26 < 0i32;
            let v28: i32 = if v27 {
                v23
            } else {
                v26
            };
            let v29: Rc<str> = std::rc::Rc::<str>::from(v20.get(v21 as usize..v28 as usize).unwrap_or(""));
            let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scanned_root_8(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v29.clone());
            let v50: i32 = v28.wrapping_add(1i32);
            let v51: i32 = v22.wrapping_add(1i32);
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v21, v22) = (v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44.clone(), v45.clone(), v46.clone(), v47, v48, v49, v50, v51);
            continue;
        }
    }
}
fn harvest_roots_from_18(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
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
                concat5_3(v2.clone(), v11.clone(), v9.clone(), v12.clone(), v12.clone())
            };
            let v15: i32 = v8.wrapping_add(1i32);
            (v1, v2) = (v15, v14.clone());
            continue;
        }
    }
}
fn scanned_default_roots_16(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: i32, mut v18: i32, mut v19: i32, mut v20: Rc<str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) {
    let v21: i32 = 0i32;
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = harvest_roots_from_18(v15.clone(), v21, v22);
    let v24: bool = &*v23 == "";
    if v24 {
        let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scanned_root_8(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18, v19, v20);
        (v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, 1i32)
    } else {
        let v45: i32 = 0i32;
        let v46: i32 = 0i32;
        scanned_listed_roots_17(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v23, v45, v46)
    }
}
fn todos_entry_0() -> i32 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v1: i32 = argument_count_1();
    let v2: bool = v1 <= 1i32;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile todos <root> [<path>...] [--json] [--tagged | --id <ID>]"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile todos rejected: "); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = concat5_3(v4.clone(), v3.clone(), v5.clone(), v5.clone(), v5.clone());
        let v7: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v6 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v7
    } else {
        let v8: i32 = 1i32;
        let v9: Rc<str> = usize::try_from(v8).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v10: bool = std::fs::symlink_metadata(&*v9).map(|metadata| metadata.file_type().is_symlink()).unwrap_or(false);
        let v19: US0 = if v10 {
            US0::US0_LinkedEntry
        } else {
            let v12: bool = directory_exists_4(v9.clone());
            if v12 {
                US0::US0_Directory
            } else {
                let v14: bool = file_exists_5(v9.clone());
                if v14 {
                    US0::US0_RegularFile
                } else {
                    US0::US0_AbsentEntry
                }
            }
        };
        match &v19 {
            US0::US0_Directory => {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
                let v21: bool = std::env::args().skip(4i32 as usize).any(|argument| argument == &*v20);
                let v24: i32 = if v21 {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"todos\":["); } LIT.with(|lit| lit.clone()) };
                    let v23: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v22.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v23
                } else {
                    0i32
                };
                let v25: bool = false;
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = concat5_3(v9.clone(), v26.clone(), v27.clone(), v27.clone(), v27.clone());
                let (mut v29, mut v30): (bool, Rc<str>) = match std::fs::read(&*v28) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                let v31: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v30);
                let v32: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().skip(4).collect::<Vec<_>>(); match args.iter().position(|argument| argument == "--id") { Some(at) => args.get(at + 1).cloned().unwrap_or_default(), None => if args.iter().any(|argument| argument == "--tagged") { "*".to_owned() } else { String::new() } } });
                let (mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scan_start_6(v21, v25, v31.clone(), v32.clone());
                let v53: i32 = 2i32;
                let v54: i32 = 0i32;
                let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) = scanned_arguments_7(v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47.clone(), v48.clone(), v49.clone(), v50, v51, v52, v53, v1, v54);
                let v76: bool = v75 == 0i32;
                let (mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) = if v76 {
                    scanned_default_roots_16(v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69.clone(), v70.clone(), v71.clone(), v72, v73, v74, v9.clone())
                } else {
                    (v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69.clone(), v70.clone(), v71.clone(), v72, v73, v74, v75)
                };
                if v21 {
                    let v119: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"roots\":"); } LIT.with(|lit| lit.clone()) };
                    let v121: Rc<str> = std::rc::Rc::<str>::from((v118).to_string());
                    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"files\":"); } LIT.with(|lit| lit.clone()) };
                    let v123: Rc<str> = std::rc::Rc::<str>::from((v98).to_string());
                    let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"scanned\":"); } LIT.with(|lit| lit.clone()) };
                    let v125: Rc<str> = concat5_3(v120.clone(), v121.clone(), v122.clone(), v123.clone(), v124.clone());
                    let v126: Rc<str> = std::rc::Rc::<str>::from((v99).to_string());
                    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"todo_count\":"); } LIT.with(|lit| lit.clone()) };
                    let v128: Rc<str> = std::rc::Rc::<str>::from((v100).to_string());
                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"skipped\":{\"nested\":"); } LIT.with(|lit| lit.clone()) };
                    let v130: Rc<str> = std::rc::Rc::<str>::from((v101).to_string());
                    let v131: Rc<str> = concat5_3(v126.clone(), v127.clone(), v128.clone(), v129.clone(), v130.clone());
                    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"vendor\":"); } LIT.with(|lit| lit.clone()) };
                    let v133: Rc<str> = std::rc::Rc::<str>::from((v102).to_string());
                    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"generated\":"); } LIT.with(|lit| lit.clone()) };
                    let v135: Rc<str> = std::rc::Rc::<str>::from((v104).to_string());
                    let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"syntax\":"); } LIT.with(|lit| lit.clone()) };
                    let v137: Rc<str> = concat5_3(v132.clone(), v133.clone(), v134.clone(), v135.clone(), v136.clone());
                    let v138: Rc<str> = std::rc::Rc::<str>::from((v103).to_string());
                    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"binary\":"); } LIT.with(|lit| lit.clone()) };
                    let v140: Rc<str> = std::rc::Rc::<str>::from((v105).to_string());
                    let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"large\":"); } LIT.with(|lit| lit.clone()) };
                    let v142: Rc<str> = std::rc::Rc::<str>::from((v106).to_string());
                    let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"links\":"); } LIT.with(|lit| lit.clone()) };
                    let v144: Rc<str> = std::rc::Rc::<str>::from((v107).to_string());
                    let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"unreadable\":"); } LIT.with(|lit| lit.clone()) };
                    let v146: Rc<str> = std::rc::Rc::<str>::from((v108).to_string());
                    let v147: Rc<str> = concat5_3(v142.clone(), v143.clone(), v144.clone(), v145.clone(), v146.clone());
                    let v148: Rc<str> = concat5_3(v138.clone(), v139.clone(), v140.clone(), v141.clone(), v147.clone());
                    let v149: Rc<str> = concat5_3(v125.clone(), v131.clone(), v137.clone(), v148.clone(), v27.clone());
                    let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("},\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
                    let v151: Rc<str> = std::rc::Rc::<str>::from((v119).to_string());
                    let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
                    let v153: Rc<str> = concat5_3(v149.clone(), v150.clone(), v151.clone(), v152.clone(), v27.clone());
                    let v154: i32 = (v153.clone().len() as i32);
                    let v155: i32 = v154.wrapping_sub(1i32);
                    let v156: Rc<str> = std::rc::Rc::<str>::from(v153.get(0i32 as usize..v155 as usize).unwrap_or(""));
                    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"stale\":"); } LIT.with(|lit| lit.clone()) };
                    let v158: Rc<str> = std::rc::Rc::<str>::from((v115).to_string());
                    let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"unknown\":"); } LIT.with(|lit| lit.clone()) };
                    let v160: Rc<str> = std::rc::Rc::<str>::from((v116).to_string());
                    let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"wide\":"); } LIT.with(|lit| lit.clone()) };
                    let v162: Rc<str> = std::rc::Rc::<str>::from((v117).to_string());
                    let v163: Rc<str> = concat5_3(v160.clone(), v161.clone(), v162.clone(), v152.clone(), v27.clone());
                    let v164: Rc<str> = concat5_3(v156.clone(), v157.clone(), v158.clone(), v159.clone(), v163.clone());
                    let v165: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v164.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v165
                } else {
                    let v166: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile todos ok roots="); } LIT.with(|lit| lit.clone()) };
                    let v168: Rc<str> = std::rc::Rc::<str>::from((v118).to_string());
                    let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) };
                    let v170: Rc<str> = std::rc::Rc::<str>::from((v98).to_string());
                    let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" scanned="); } LIT.with(|lit| lit.clone()) };
                    let v172: Rc<str> = concat5_3(v167.clone(), v168.clone(), v169.clone(), v170.clone(), v171.clone());
                    let v173: Rc<str> = std::rc::Rc::<str>::from((v99).to_string());
                    let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" todos="); } LIT.with(|lit| lit.clone()) };
                    let v175: Rc<str> = std::rc::Rc::<str>::from((v100).to_string());
                    let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" skipped="); } LIT.with(|lit| lit.clone()) };
                    let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nested:"); } LIT.with(|lit| lit.clone()) };
                    let v178: Rc<str> = std::rc::Rc::<str>::from((v101).to_string());
                    let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",vendor:"); } LIT.with(|lit| lit.clone()) };
                    let v180: Rc<str> = std::rc::Rc::<str>::from((v102).to_string());
                    let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",generated:"); } LIT.with(|lit| lit.clone()) };
                    let v182: Rc<str> = concat5_3(v177.clone(), v178.clone(), v179.clone(), v180.clone(), v181.clone());
                    let v183: Rc<str> = std::rc::Rc::<str>::from((v104).to_string());
                    let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",syntax:"); } LIT.with(|lit| lit.clone()) };
                    let v185: Rc<str> = std::rc::Rc::<str>::from((v103).to_string());
                    let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",binary:"); } LIT.with(|lit| lit.clone()) };
                    let v187: Rc<str> = std::rc::Rc::<str>::from((v105).to_string());
                    let v188: Rc<str> = concat5_3(v183.clone(), v184.clone(), v185.clone(), v186.clone(), v187.clone());
                    let v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",large:"); } LIT.with(|lit| lit.clone()) };
                    let v190: Rc<str> = std::rc::Rc::<str>::from((v106).to_string());
                    let v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",links:"); } LIT.with(|lit| lit.clone()) };
                    let v192: Rc<str> = std::rc::Rc::<str>::from((v107).to_string());
                    let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",unreadable:"); } LIT.with(|lit| lit.clone()) };
                    let v194: Rc<str> = std::rc::Rc::<str>::from((v108).to_string());
                    let v195: Rc<str> = concat5_3(v191.clone(), v192.clone(), v193.clone(), v194.clone(), v27.clone());
                    let v196: Rc<str> = concat5_3(v182.clone(), v188.clone(), v189.clone(), v190.clone(), v195.clone());
                    let v197: Rc<str> = concat5_3(v173.clone(), v174.clone(), v175.clone(), v176.clone(), v196.clone());
                    let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                    let v199: Rc<str> = std::rc::Rc::<str>::from((v166).to_string());
                    let v200: Rc<str> = concat5_3(v172.clone(), v197.clone(), v198.clone(), v199.clone(), v27.clone());
                    let v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stale="); } LIT.with(|lit| lit.clone()) };
                    let v202: Rc<str> = std::rc::Rc::<str>::from((v115).to_string());
                    let v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unknown="); } LIT.with(|lit| lit.clone()) };
                    let v204: Rc<str> = std::rc::Rc::<str>::from((v116).to_string());
                    let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wide="); } LIT.with(|lit| lit.clone()) };
                    let v206: Rc<str> = std::rc::Rc::<str>::from((v117).to_string());
                    let v207: Rc<str> = concat5_3(v205.clone(), v206.clone(), v27.clone(), v27.clone(), v27.clone());
                    let v208: Rc<str> = concat5_3(v201.clone(), v202.clone(), v203.clone(), v204.clone(), v207.clone());
                    let v209: Rc<str> = concat5_3(v200.clone(), v208.clone(), v27.clone(), v27.clone(), v27.clone());
                    let v210: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v209.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v210
                }
            }
            _ => {
                let v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the backlog root "); } LIT.with(|lit| lit.clone()) };
                let v213: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a directory"); } LIT.with(|lit| lit.clone()) };
                let v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v215: Rc<str> = concat5_3(v212.clone(), v9.clone(), v213.clone(), v214.clone(), v214.clone());
                let v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile todos rejected: "); } LIT.with(|lit| lit.clone()) };
                let v217: Rc<str> = concat5_3(v216.clone(), v215.clone(), v214.clone(), v214.clone(), v214.clone());
                let v218: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v217 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v218
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        todos_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn todo_sites_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = false;
    let v3: bool = true;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = concat5_3(v0.clone(), v4, v5.clone(), v5.clone(), v5);
    let (mut v7, mut v8): (bool, Rc<str>) = match std::fs::read(&*v6) { Ok(bytes) => (bytes.contains(&0u8), std::rc::Rc::<str>::from(String::from_utf8_lossy(&bytes).as_ref())), Err(_) => (true, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
    let v9: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v8);
    let (mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = scan_start_6(v2, v3, v9, v1);
    let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32, i32) = scanned_default_roots_16(v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v0);
    v44
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        todo_sites_19(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_todos_run() -> i32 {
    closure0()()
}
pub fn kit_agile_todo_sites(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
