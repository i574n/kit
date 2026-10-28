#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_PrintCensus,
    US0_WriteBaseline(Rc<str>),
    US0_MergeBaseline(Rc<str>),
    US0_CheckRatchet(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_SpiralFile,
    US3_Directory,
}
#[derive(Clone)]
enum US4 {
    US4_MayNotFall,
    US4_MayNotRise,
    US4_Unranked,
}
#[derive(Clone)]
enum UH1 {
    UH1_NoCounts,
    UH1_CountThen(Rc<str>, US4, i32, Rc<UH1>),
}
#[derive(Clone)]
enum US5 {
    US5_Held,
    US5_Regressed(Rc<str>),
}
fn text_written_3(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = std::fs::write(&*v0, (v1).as_bytes()).is_ok();
    v2
}
fn stdout_line_4(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn stderr_line_5(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn baseline_saved_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union census_row = CensusCount : string * string * i32 * i32\n\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v1, &*v4, &*v5, &*v5].concat());
    let v7: bool = text_written_3(v0.clone(), v6.clone());
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards baseline ok packages="); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v2, &*v9, &*v0, &*v5].concat());
        stdout_line_4(v10.clone());
        0i32
    } else {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards could not write "); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v0, &*v5, &*v5, &*v5].concat());
        stderr_line_5(v12.clone());
        2i32
    }
}
fn method8(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_7(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn count_text_6(mut v0: i32) -> Rc<str> {
    format_real_7(v0)
}
fn file_text_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
    v1
}
fn quote_11() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(char::from(34u8).to_string().as_str());
    v0
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method12(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method12(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure1(v0, v1)
}
fn appended_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    if v2 {
        v1.clone()
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v1, &*v4, &*v4].concat());
        v5.clone()
    }
}
fn kept_rows_10(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            return v3.clone();
        } else {
            let mut v6: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_"); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v6.starts_with(&*v7);
            let v28: bool = if v8 {
                let mut v9: Rc<str> = quote_11();
                let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v6.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let v11: Rc<RefCell<Vec<Rc<str>>>> = v10 ;
                let v12: i32 = (v11.clone().borrow().len() as i32);
                let v13: bool = 1i32 < v12;
                let v17: US1 = if v13 {
                    let v14: Rc<str> = v11.clone().borrow()[1i32 as usize].clone();
                    US1::US1_Some(v14.clone())
                } else {
                    US1::US1_None
                };
                let v21: Rc<str> = match &v17 {
                    US1::US1_None => {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v19.clone()
                    }
                    US1::US1_Some(v18) => {
                        let v18: Rc<str> = v18.clone();
                        v18.clone()
                    }
                };
                let v22: i32 = 0i32;
                let mut v23: Rc<dyn Fn(Rc<UH0>) -> US2> = method12(v21.clone(), v22);
                let v24: US2 = v23(v2.clone());
                let v25: bool = match &v24 {
                    US2::US2_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v26: bool = v25 != true;
                let v27: bool = v26 == false;
                v27
            } else {
                false
            };
            let v29: i32 = v1.wrapping_add(1i32);
            let v31: Rc<str> = if v28 {
                appended_13(v3.clone(), v6.clone())
            } else {
                v3.clone()
            };
            (v0, v1, v2, v3) = (v0.clone(), v29, v2.clone(), v31.clone());
            continue;
        }
    }
}
fn census_report_1(mut v0: US0, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<UH0>) -> i32 {
    let v5: Rc<str> = count_text_6(v1);
    match &v0 {
        US0::US0_CheckRatchet(v20) => {
            let v20: Rc<str> = v20.clone();
            let v21: bool = v2 == 0i32;
            let v24: Rc<str> = if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v22.clone()
            } else {
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
                v23.clone()
            };
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards ratchet "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" packages="); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" regressions="); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = count_text_6(v2);
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v5, &*v27, &*v28, &*v29].concat());
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" baseline="); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v24, &*v30, &*v31, &*v20].concat());
            stdout_line_4(v32.clone());
            if v21 {
                0i32
            } else {
                1i32
            }
        }
        US0::US0_MergeBaseline(v11) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = file_text_9(v11.clone());
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v12.split(&*v13).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v15: i32 = 0i32;
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = kept_rows_10(v14.clone(), v15, v4.clone(), v16.clone());
            let v18: Rc<str> = appended_13(v17.clone(), v3.clone());
            baseline_saved_2(v11.clone(), v18.clone(), v5.clone())
        }
        US0::US0_PrintCensus => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards census ok packages="); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v5, &*v7, &*v7, &*v7].concat());
            stdout_line_4(v8.clone());
            0i32
        }
        US0::US0_WriteBaseline(v9) => {
            let mut v9: Rc<str> = v9.clone();
            baseline_saved_2(v9.clone(), v3.clone(), v5.clone())
        }
    }
}
fn command_line_argument_count_15() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_16(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_14(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_15();
        let v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US1 = if v5 {
        let mut v6: Rc<str> = command_line_argument_16(v1);
        US1::US1_Some(v6.clone())
    } else {
        US1::US1_None
    };
    match &v9 {
        US1::US1_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US1::US1_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn mode_of_17(mut v0: Rc<str>) -> US0 {
    let v1: bool = &*v0 == "--baseline";
    if v1 {
        let v2: i32 = 3i32;
        let v3: Rc<str> = argument_14(v2);
        US0::US0_WriteBaseline(v3)
    } else {
        let v5: bool = &*v0 == "--baseline-merge";
        if v5 {
            let mut v6: i32 = 3i32;
            let v7: Rc<str> = argument_14(v6);
            US0::US0_MergeBaseline(v7)
        } else {
            let mut v9: bool = &*v0 == "--ratchet";
            if v9 {
                let v10: i32 = 3i32;
                let v11: Rc<str> = argument_14(v10);
                US0::US0_CheckRatchet(v11)
            } else {
                US0::US0_PrintCensus
            }
        }
    }
}
fn census_paths_18(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(v0 as usize).collect::<Vec<_>>().join("
").as_str());
    v1
}
fn first_line_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
    v1
}
fn other_lines_23(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
    v1.clone()
}
fn directory_exists_26(mut v0: Rc<str>) -> bool {
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
fn is_dir_25(mut v0: Rc<str>) -> bool {
    directory_exists_26(v0)
}
fn entry_matches_24(mut v0: US3, mut v1: Rc<str>) -> bool {
    match &v0 {
        US3::US3_Directory => {
            is_dir_25(v1.clone())
        }
        US3::US3_SpiralFile => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v3: bool = v1.ends_with(&*v2);
            v3
        }
    }
}
fn kept_entries_22(mut v0: US3, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: bool = &*v1 == "";
        if v3 {
            return v2.clone();
        } else {
            let v4: Rc<str> = first_line_20(v1.clone());
            let v5: Rc<str> = other_lines_23(v1.clone());
            let mut v6: bool = entry_matches_24(v0.clone(), v4.clone());
            let v8: Rc<str> = if v6 {
                appended_13(v2.clone(), v4.clone())
            } else {
                v2.clone()
            };
            (v0, v1, v2) = (v0.clone(), v5.clone(), v8.clone());
            continue;
        }
    }
}
fn entry_list_27(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|e| e.path().to_string_lossy().into_owned()).collect::<Vec<_>>().join("
").as_str());
    v1.clone()
}
fn sorted_lines_28(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from({ let mut lines = (v0).lines().collect::<Vec<_>>(); lines.sort(); lines.join("
") }.as_str());
    v1
}
fn spi_paths_21(mut v0: Rc<str>) -> Rc<str> {
    let v1: US3 = US3::US3_SpiralFile;
    let v2: Rc<str> = entry_list_27(v0.clone());
    let v3: Rc<str> = sorted_lines_28(v2.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    kept_entries_22(v1.clone(), v3.clone(), v4.clone())
}
fn child_dirs_29(mut v0: Rc<str>) -> Rc<str> {
    let v1: US3 = US3::US3_Directory;
    let v2: Rc<str> = entry_list_27(v0.clone());
    let v3: Rc<str> = sorted_lines_28(v2.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    kept_entries_22(v1.clone(), v3.clone(), v4.clone())
}
fn package_children_30(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let v2: bool = &*v0 == "";
        if v2 {
            return v1.clone();
        } else {
            let v3: Rc<str> = first_line_20(v0.clone());
            let v4: Rc<str> = other_lines_23(v0.clone());
            let v5: Rc<str> = spi_paths_21(v3.clone());
            let mut v6: bool = &*v5 == "";
            let v8: Rc<str> = if v6 {
                v1.clone()
            } else {
                appended_13(v1.clone(), v3.clone())
            };
            (v0, v1) = (v4.clone(), v8.clone());
            continue;
        }
    }
}
fn package_dirs_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let v2: bool = &*v0 == "";
        if v2 {
            return v1.clone();
        } else {
            let v3: Rc<str> = first_line_20(v0.clone());
            let v4: Rc<str> = spi_paths_21(v3.clone());
            let v5: bool = &*v4 == "";
            let mut v9: Rc<str> = if v5 {
                let mut v6: Rc<str> = child_dirs_29(v3.clone());
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                package_children_30(v6.clone(), v7.clone())
            } else {
                v3.clone()
            };
            let v10: Rc<str> = other_lines_23(v0.clone());
            let v11: bool = &*v9 == "";
            let v13: Rc<str> = if v11 {
                v1.clone()
            } else {
                appended_13(v1.clone(), v9.clone())
            };
            (v0, v1) = (v10.clone(), v13.clone());
            continue;
        }
    }
}
fn text_of_files_33(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let v2: bool = &*v0 == "";
        if v2 {
            return v1.clone();
        } else {
            let v3: Rc<str> = first_line_20(v0.clone());
            let v4: Rc<str> = file_text_9(v3.clone());
            let v5: Rc<str> = other_lines_23(v0.clone());
            let mut v6: bool = &*v4 == "";
            let mut v9: bool = if v6 {
                true
            } else {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v8: bool = v4.ends_with(&*v7);
                v8
            };
            let v12: Rc<str> = if v9 {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                v11.clone()
            };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v4, &*v12, &*v13, &*v13].concat());
            (v0, v1) = (v5.clone(), v14.clone());
            continue;
        }
    }
}
fn matching_lines_34(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: i32 = regex::Regex::new(&*v1).map(|pattern| (v0).lines().filter(|line| pattern.is_match(line)).count() as i32).unwrap_or(0);
    v2
}
fn package_name_35(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default().as_str());
    v1.clone()
}
fn line_total_36(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).lines().count() as i32;
    v1
}
fn census_of_32(mut v0: Rc<str>) -> (Rc<str>, i32, Rc<UH1>) {
    let v1: Rc<str> = spi_paths_21(v0.clone());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = text_of_files_33(v1.clone(), v2.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*\\|\\s*[A-Z]\\w*\\s*::[^=]*$"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = matching_lines_34(v3.clone(), v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\bexists\\s+\\w+\\."); } LIT.with(|lit| lit.clone()) };
    let v7: i32 = matching_lines_34(v3.clone(), v6.clone());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\bforall\\b"); } LIT.with(|lit| lit.clone()) };
    let mut v9: i32 = matching_lines_34(v3.clone(), v8.clone());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*union\\s+(rec\\s+)?(private\\s+)?\\w+\\s+\\w+\\s*="); } LIT.with(|lit| lit.clone()) };
    let v11: i32 = matching_lines_34(v3.clone(), v10.clone());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*(prototype|instance)\\b"); } LIT.with(|lit| lit.clone()) };
    let v13: i32 = matching_lines_34(v3.clone(), v12.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*(union|type|nominal)\\b"); } LIT.with(|lit| lit.clone()) };
    let v15: i32 = matching_lines_34(v3.clone(), v14.clone());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s+.*(\\$'|\\$\\x22)"); } LIT.with(|lit| lit.clone()) };
    let v17: i32 = matching_lines_34(v3.clone(), v16.clone());
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\$'[^']{200,}'"); } LIT.with(|lit| lit.clone()) };
    let v19: i32 = matching_lines_34(v3.clone(), v18.clone());
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(^|[^\\w.])(rust_global\\b|global\\s*(\\(|\\x22))"); } LIT.with(|lit| lit.clone()) };
    let v21: i32 = matching_lines_34(v3.clone(), v20.clone());
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("char::from\\((9|30|31)u8\\)|\\x22\\\\t\\x22"); } LIT.with(|lit| lit.clone()) };
    let mut v23: i32 = matching_lines_34(v3.clone(), v22.clone());
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pipe_(index|member|lookup)"); } LIT.with(|lit| lit.clone()) };
    let v25: i32 = matching_lines_34(v3.clone(), v24.clone());
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*\\|\\s*\\d+(u64|i32|u32)?\\s*=>"); } LIT.with(|lit| lit.clone()) };
    let v27: i32 = matching_lines_34(v3.clone(), v26.clone());
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\bconcat5\\b"); } LIT.with(|lit| lit.clone()) };
    let v29: i32 = matching_lines_34(v3.clone(), v28.clone());
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\S.*(\\$'|\\$\\x22)"); } LIT.with(|lit| lit.clone()) };
    let v31: i32 = matching_lines_34(v3.clone(), v30.clone());
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Rc::<str>::from"); } LIT.with(|lit| lit.clone()) };
    let v33: i32 = matching_lines_34(v3.clone(), v32.clone());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\$'\\{"); } LIT.with(|lit| lit.clone()) };
    let v35: i32 = matching_lines_34(v3.clone(), v34.clone());
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_std_string\\."); } LIT.with(|lit| lit.clone()) };
    let v37: i32 = matching_lines_34(v3.clone(), v36.clone());
    let v38: Rc<str> = package_name_35(v0.clone());
    let v39: i32 = line_total_36(v3.clone());
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("std-string-use"); } LIT.with(|lit| lit.clone()) };
    let v41: US4 = US4::US4_MayNotRise;
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("let-block"); } LIT.with(|lit| lit.clone()) };
    let v43: US4 = US4::US4_MayNotRise;
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rc-wrap"); } LIT.with(|lit| lit.clone()) };
    let v45: US4 = US4::US4_MayNotRise;
    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("top-level-macro"); } LIT.with(|lit| lit.clone()) };
    let v47: US4 = US4::US4_MayNotRise;
    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("string-concat"); } LIT.with(|lit| lit.clone()) };
    let v49: US4 = US4::US4_MayNotRise;
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("number-arm"); } LIT.with(|lit| lit.clone()) };
    let v51: US4 = US4::US4_MayNotRise;
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pipe-table"); } LIT.with(|lit| lit.clone()) };
    let v53: US4 = US4::US4_MayNotRise;
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("encoded-row"); } LIT.with(|lit| lit.clone()) };
    let v55: US4 = US4::US4_MayNotRise;
    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-global"); } LIT.with(|lit| lit.clone()) };
    let v57: US4 = US4::US4_MayNotRise;
    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wide-host-macro"); } LIT.with(|lit| lit.clone()) };
    let v59: US4 = US4::US4_MayNotRise;
    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-macro"); } LIT.with(|lit| lit.clone()) };
    let v61: US4 = US4::US4_MayNotRise;
    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
    let v63: US4 = US4::US4_Unranked;
    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prototype"); } LIT.with(|lit| lit.clone()) };
    let v65: US4 = US4::US4_MayNotFall;
    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("indexed-union"); } LIT.with(|lit| lit.clone()) };
    let v67: US4 = US4::US4_MayNotFall;
    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("forall"); } LIT.with(|lit| lit.clone()) };
    let v69: US4 = US4::US4_MayNotFall;
    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("existential"); } LIT.with(|lit| lit.clone()) };
    let v71: US4 = US4::US4_MayNotFall;
    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gadt-case"); } LIT.with(|lit| lit.clone()) };
    let v73: US4 = US4::US4_MayNotFall;
    let v74: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoCounts); } CASE.with(|case| case.clone()) };
    let v75: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v72.clone(), v73.clone(), v5, v74.clone()));
    let v76: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v70.clone(), v71.clone(), v7, v75.clone()));
    let v77: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v68.clone(), v69.clone(), v9, v76.clone()));
    let v78: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v66.clone(), v67.clone(), v11, v77.clone()));
    let v79: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v64.clone(), v65.clone(), v13, v78.clone()));
    let v80: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v62.clone(), v63.clone(), v15, v79.clone()));
    let v81: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v60.clone(), v61.clone(), v17, v80.clone()));
    let v82: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v58.clone(), v59.clone(), v19, v81.clone()));
    let v83: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v56.clone(), v57.clone(), v21, v82.clone()));
    let v84: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v54.clone(), v55.clone(), v23, v83.clone()));
    let v85: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v52.clone(), v53.clone(), v25, v84.clone()));
    let v86: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v50.clone(), v51.clone(), v27, v85.clone()));
    let v87: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v48.clone(), v49.clone(), v29, v86.clone()));
    let v88: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v46.clone(), v47.clone(), v31, v87.clone()));
    let v89: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v44.clone(), v45.clone(), v33, v88.clone()));
    let v90: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v42.clone(), v43.clone(), v35, v89.clone()));
    let v91: Rc<UH1> = Rc::new(UH1::UH1_CountThen(v40.clone(), v41.clone(), v37, v90.clone()));
    (v38.clone(), v39, v91.clone())
}
fn counts_text_37(mut v0: Rc<UH1>) -> Rc<str> {
    match &*v0 {
        UH1::UH1_CountThen(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2.clone();
            let v3: US4 = v3.clone();
            let v4: i32 = *v4;
            let v5: Rc<UH1> = v5.clone();
            let mut v6: Rc<str> = counts_text_37(v5.clone());
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = count_text_6(v4);
            let v10: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v2, &*v8, &*v9].concat());
            v10.clone()
        }
        UH1::UH1_NoCounts => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
    }
}
fn binding_of_39(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_lowercase().replace(char::from(45u8), "_").as_str());
    v1
}
fn census_key_40(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CensusCount ("); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = quote_11();
    let v4: Rc<str> = quote_11();
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = quote_11();
    let v7: Rc<str> = quote_11();
    let v8: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v6, &*v1, &*v7, &*v5].concat());
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v0, &*v4, &*v8].concat());
    v9.clone()
}
fn baseline_rows_38(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH1::UH1_CountThen(v4, v5, v6, v7) => {
                let v4: Rc<str> = v4.clone();
                let v5: US4 = v5.clone();
                let mut v6: i32 = *v6;
                let v7: Rc<UH1> = v7.clone();
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_"); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = binding_of_39(v1.clone());
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                let v11: Rc<str> = binding_of_39(v4.clone());
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v10, &*v11, &*v12].concat());
                let v14: Rc<str> = census_key_40(v1.clone(), v4.clone());
                let v15: Rc<str> = count_text_6(v6);
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32, "); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = count_text_6(v2);
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32)"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v19, &*v19, &*v19].concat());
                let v21: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v16, &*v20].concat());
                let v22: Rc<str> = appended_13(v3.clone(), v21.clone());
                (v0, v1, v2, v3) = (v7.clone(), v1.clone(), v2, v22.clone());
                continue;
            }
            UH1::UH1_NoCounts => {
                return v3.clone();
            }
        }
    }
}
fn line_after_43(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().find_map(|line| line.split_once(&*v1).map(|(_, rest)| rest)).unwrap_or(""));
    v2.clone()
}
fn integer_field_44(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: i32 = (v0).split(char::from(44u8)).nth(v1 as usize).and_then(|part| part.trim().trim_end_matches(")").trim_end_matches("i32").parse::<i32>().ok()).unwrap_or(-1);
    v2
}
fn entry_of_42(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (i32, i32) {
    let v3: Rc<str> = census_key_40(v1, v2);
    let v4: Rc<str> = line_after_43(v0, v3);
    let v5: bool = &*v4 == "";
    let (mut v10, mut v11): (i32, i32) = if v5 {
        (-1i32, 0i32)
    } else {
        let mut v6: i32 = 0i32;
        let v7: i32 = integer_field_44(v4.clone(), v6);
        let v8: i32 = 1i32;
        let mut v9: i32 = integer_field_44(v4, v8);
        (v7, v9)
    };
    (v10, v11)
}
fn count_of_46(mut v0: Rc<UH1>, mut v1: Rc<str>) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_CountThen(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: US4 = v3.clone();
                let v4: i32 = *v4;
                let v5: Rc<UH1> = v5.clone();
                let mut v6: bool = &*v2 == &*v1;
                if v6 {
                    return v4;
                } else {
                    (v0, v1) = (v5.clone(), v1.clone());
                    continue;
                }
            }
            UH1::UH1_NoCounts => {
                return 0i32;
            }
        }
    }
}
fn feature_total_47(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_CountThen(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: US4 = v3.clone();
                let v4: i32 = *v4;
                let v5: Rc<UH1> = v5.clone();
                match &v3 {
                    US4::US4_MayNotFall => {
                        let mut v6: i32 = v1.wrapping_add(v4);
                        (v0, v1) = (v5.clone(), v6);
                        continue;
                    }
                    _ => {
                        (v0, v1) = (v5.clone(), v1);
                        continue;
                    }
                }
            }
            UH1::UH1_NoCounts => {
                return v1;
            }
        }
    }
}
fn new_package_problem_45(mut v0: Rc<UH1>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-global"); } LIT.with(|lit| lit.clone()) };
    let v2: i32 = count_of_46(v0.clone(), v1.clone());
    let v3: bool = v2 > 0i32;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package carries rust_global"); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wide-host-macro"); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = count_of_46(v0.clone(), v5.clone());
        let v7: bool = v6 > 0i32;
        if v7 {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package has host macros over 200 characters"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        } else {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("encoded-row"); } LIT.with(|lit| lit.clone()) };
            let v10: i32 = count_of_46(v0.clone(), v9.clone());
            let v11: bool = v10 > 0i32;
            if v11 {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package encodes rows with tab or record separators"); } LIT.with(|lit| lit.clone()) };
                v12.clone()
            } else {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
                let v14: i32 = count_of_46(v0.clone(), v13.clone());
                let v15: bool = v14 >= 2i32;
                let v19: bool = if v15 {
                    let v16: i32 = 0i32;
                    let v17: i32 = feature_total_47(v0.clone(), v16);
                    let v18: bool = v17 == 0i32;
                    v18
                } else {
                    false
                };
                if v19 {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package declares types but uses none of the typefeatureset (GADT cases, existentials, foralls, indexed unions, prototypes)"); } LIT.with(|lit| lit.clone()) };
                    v20.clone()
                } else {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v21.clone()
                }
            }
        }
    }
}
fn regressed_49(mut v0: US4, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
    let v5: f64 = (v1 as f64) * 1000.0 / (v2).max(1) as f64;
    let mut v6: f64 = (v3 as f64) * 1000.0 / (v4).max(1) as f64;
    match &v0 {
        US4::US4_MayNotFall => {
            let v12: f64 = v5 * 0.95f64;
            let v13: f64 = v12 - 0.5f64;
            let v14: bool = v6 < v13;
            if v14 {
                let v15: i32 = v1.wrapping_sub(v3);
                let v16: i32 = v2.wrapping_sub(v4);
                let v17: bool = v15 > v16;
                v17
            } else {
                false
            }
        }
        US4::US4_MayNotRise => {
            let v7: f64 = v5 * 1.05f64;
            let v8: f64 = v7 + 0.5f64;
            let mut v9: bool = v6 > v8;
            if v9 {
                let v10: bool = v3 > v1;
                v10
            } else {
                false
            }
        }
        US4::US4_Unranked => {
            false
        }
    }
}
fn probes_compared_48(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH1::UH1_CountThen(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: US4 = v6.clone();
                let v7: i32 = *v7;
                let v8: Rc<UH1> = v8.clone();
                let (mut v9, mut v10): (i32, i32) = entry_of_42(v1.clone(), v2.clone(), v5.clone());
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = count_text_6(v9);
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = count_text_6(v10);
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = count_text_6(v7);
                let v17: Rc<str> = count_text_6(v3);
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v13, &*v17, &*v18, &*v18].concat());
                let v20: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v14, &*v15, &*v19].concat());
                let v21: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v11, &*v5, &*v11, &*v20].concat());
                let v22: bool = v9 >= 0i32;
                let v24: bool = if v22 {
                    regressed_49(v6.clone(), v9, v10, v7, v3)
                } else {
                    false
                };
                let v26: Rc<str> = if v24 {
                    appended_13(v4.clone(), v21.clone())
                } else {
                    v4.clone()
                };
                (v0, v1, v2, v3, v4) = (v8.clone(), v1.clone(), v2.clone(), v3, v26.clone());
                continue;
            }
            UH1::UH1_NoCounts => {
                return v4.clone();
            }
        }
    }
}
fn census_verdict_41(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<str>) -> US5 {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
    let (mut v5, mut v6): (i32, i32) = entry_of_42(v3.clone(), v0.clone(), v4.clone());
    let v7: bool = v5 < 0i32;
    if v7 {
        let v8: Rc<str> = new_package_problem_45(v2.clone());
        let mut v9: bool = &*v8 == "";
        if v9 {
            US5::US5_Held
        } else {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  new  "); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v11, &*v8, &*v12, &*v12].concat());
            US5::US5_Regressed(v13.clone())
        }
    } else {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = probes_compared_48(v2.clone(), v3.clone(), v0.clone(), v1, v16.clone());
        let v18: bool = &*v17 == "";
        if v18 {
            US5::US5_Held
        } else {
            US5::US5_Regressed(v17.clone())
        }
    }
}
fn census_walk_31(mut v0: US0, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<UH0>) -> (i32, i32, Rc<str>, Rc<UH0>) {
    let mut v6: bool = &*v1 == "";
    let (mut v76, mut v77, mut v78, mut v79): (i32, i32, Rc<str>, Rc<UH0>) = if v6 {
        (v2, v3, v4.clone(), v5.clone())
    } else {
        let v7: Rc<str> = first_line_20(v1.clone());
        let (mut v8, mut v9, mut v10): (Rc<str>, i32, Rc<UH1>) = census_of_32(v7.clone());
        let v11: i32 = v2.wrapping_add(1i32);
        let (mut v72, mut v73, mut v74, mut v75): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
            US0::US0_PrintCensus => {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines="); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = count_text_6(v9);
                let v14: Rc<str> = counts_text_37(v10.clone());
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v12, &*v13, &*v14, &*v15].concat());
                stdout_line_4(v16.clone());
                let v17: Rc<str> = other_lines_23(v1.clone());
                census_walk_31(v0.clone(), v17.clone(), v11, v3, v4.clone(), v5.clone())
            }
            _ => {
                let (mut v68, mut v69, mut v70, mut v71): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                    US0::US0_WriteBaseline(v22) => {
                        let v22: Rc<str> = v22.clone();
                        let mut v23: Rc<str> = other_lines_23(v1.clone());
                        let v24: Rc<str> = baseline_rows_38(v10.clone(), v8.clone(), v9, v4.clone());
                        census_walk_31(v0.clone(), v23.clone(), v11, v3, v24.clone(), v5.clone())
                    }
                    _ => {
                        let (mut v64, mut v65, mut v66, mut v67): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                            US0::US0_MergeBaseline(v29) => {
                                let v29: Rc<str> = v29.clone();
                                let v30: Rc<str> = other_lines_23(v1.clone());
                                let v31: Rc<str> = baseline_rows_38(v10.clone(), v8.clone(), v9, v4.clone());
                                let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v5.clone()));
                                census_walk_31(v0.clone(), v30.clone(), v11, v3, v31.clone(), v32.clone())
                            }
                            _ => {
                                let (mut v60, mut v61, mut v62, mut v63): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                                    US0::US0_CheckRatchet(v37) => {
                                        let v37: Rc<str> = v37.clone();
                                        let v38: Rc<str> = file_text_9(v37.clone());
                                        let v39: US5 = census_verdict_41(v8.clone(), v9, v10.clone(), v38.clone());
                                        match &v39 {
                                            US5::US5_Held => {
                                                let v40: Rc<str> = other_lines_23(v1.clone());
                                                census_walk_31(v0.clone(), v40.clone(), v11, v3, v4.clone(), v5.clone())
                                            }
                                            _ => {
                                                let (mut v52, mut v53, mut v54, mut v55): (i32, i32, Rc<str>, Rc<UH0>) = match &v39 {
                                                    US5::US5_Regressed(v45) => {
                                                        let v45: Rc<str> = v45.clone();
                                                        stdout_line_4(v45.clone());
                                                        let v46: Rc<str> = other_lines_23(v1.clone());
                                                        let v47: i32 = v3.wrapping_add(1i32);
                                                        census_walk_31(v0.clone(), v46.clone(), v11, v47, v4.clone(), v5.clone())
                                                    }
                                                    _ => unreachable!(),
                                                };
                                                (v52, v53, v54.clone(), v55.clone())
                                            }
                                        }
                                    }
                                    _ => unreachable!(),
                                };
                                (v60, v61, v62.clone(), v63.clone())
                            }
                        };
                        (v64, v65, v66.clone(), v67.clone())
                    }
                };
                (v68, v69, v70.clone(), v71.clone())
            }
        };
        (v72, v73, v74.clone(), v75.clone())
    };
    (v76, v77, v78.clone(), v79.clone())
}
fn census_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: Rc<str> = argument_14(v0);
    let v2: US0 = mode_of_17(v1.clone());
    let v3: i32 = match &v2 {
        US0::US0_PrintCensus => {
            3i32
        }
        _ => {
            4i32
        }
    };
    let v4: Rc<str> = census_paths_18(v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = package_dirs_19(v4.clone(), v5.clone());
    let v7: bool = &*v6 == "";
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards found no Spiral packages; "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: standards --census <dir>... | standards --baseline <file.spi> <dir>... | standards --baseline-merge <file.spi> <dir>... (replaces only the walked packages' rows) | standards --ratchet <file.spi> <dir>... (a directory without .spi files stands for its package subdirectories)"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v5, &*v5, &*v5].concat());
        stderr_line_5(v10.clone());
        2i32
    } else {
        let v11: i32 = 0i32;
        let v12: i32 = 0i32;
        let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let (mut v14, mut v15, mut v16, mut v17): (i32, i32, Rc<str>, Rc<UH0>) = census_walk_31(v2.clone(), v6.clone(), v11, v12, v5.clone(), v13.clone());
        census_report_1(v2.clone(), v14, v15, v16.clone(), v17.clone())
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        census_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_census_run() -> i32 {
    closure0()()
}
