#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Bare(Rc<str>),
    US0_Quoted(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_NoMoreWords,
    UH0_WordThen(US0, Rc<UH0>),
}
fn byte_at_2(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: i32 = (v0).as_bytes().get(v1 as usize).map(|byte| *byte as i32).unwrap_or(-1);
    v2
}
fn word_end_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = byte_at_2(v0.clone(), v1);
        let v4: bool = v3 < 0i32;
        let mut v6: bool = if v4 {
            true
        } else {
            let v5: bool = v3 == v2;
            v5
        };
        if v6 {
            return v1;
        } else {
            let v7: i32 = v1.wrapping_add(1i32);
            v1 = v7;
            continue;
        }
    }
}
fn text_between_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize..v2 as usize).unwrap_or(""));
    v3
}
fn words_scanned_1(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: i32 = byte_at_2(v0.clone(), v1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v2;
        } else {
            let v5: bool = v3 == 32i32;
            if v5 {
                let mut v6: i32 = v1.wrapping_add(1i32);
                v1 = v6;
                continue;
            } else {
                let v8: bool = v3 == 34i32;
                if v8 {
                    let v9: i32 = v1.wrapping_add(1i32);
                    let v10: i32 = 34i32;
                    let v11: i32 = word_end_3(v0.clone(), v9, v10);
                    let v12: i32 = v11.wrapping_add(1i32);
                    let v13: Rc<str> = text_between_4(v0.clone(), v9, v11);
                    let v14: US0 = US0::US0_Quoted(v13);
                    let v15: Rc<UH0> = Rc::new(UH0::UH0_WordThen(v14, v2));
                    (v1, v2) = (v12, v15);
                    continue;
                } else {
                    let v17: i32 = 32i32;
                    let v18: i32 = word_end_3(v0.clone(), v1, v17);
                    let v19: Rc<str> = text_between_4(v0.clone(), v1, v18);
                    let v20: US0 = US0::US0_Bare(v19);
                    let v21: Rc<UH0> = Rc::new(UH0::UH0_WordThen(v20, v2));
                    (v1, v2) = (v18, v21);
                    continue;
                }
            }
        }
    }
}
fn words_flipped_5(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_NoMoreWords => {
                return v1;
            }
            UH0::UH0_WordThen(v2, v3) => {
                let mut v2: US0 = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_WordThen(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(US0) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(US0) -> Rc<str>> = Rc::new(move |mut v0: US0| -> Rc<str> {
        match v0 {
            US0::US0_Bare(v1) => {
                let mut v1: Rc<str> = v1;
                v1
            }
            US0::US0_Quoted(v2) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v2));
                let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                v4
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn words_joined_7(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<dyn Fn(US0) -> Rc<str>>, mut v3: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_NoMoreWords => {
                return v3;
            }
            UH0::UH0_WordThen(v4, v5) => {
                let v4: US0 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v3 == "";
                let v11: Rc<str> = if v6 {
                    v2(v4)
                } else {
                    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1.clone()));
                    let v9: Rc<str> = v2(v4);
                    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
                    v10
                };
                (v0, v3) = (v5, v11);
                continue;
            }
        }
    }
}
fn any_letter_of_10(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (v0).chars().any(|letter| (v1).contains(letter));
    v2
}
fn word_needs_shell_9(mut v0: US0) -> bool {
    match v0 {
        US0::US0_Bare(v1) => {
            let mut v1: Rc<str> = v1;
            let mut v2: bool = cfg!(windows);
            let v5: Rc<str> = if v2 {
                let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|&<>^%()\n"); } LIT.with(|lit| lit.clone()) };
                v3
            } else {
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|&<>;$`*?~(){}[]'\"\\\n"); } LIT.with(|lit| lit.clone()) };
                v4
            };
            any_letter_of_10(v1, v5)
        }
        US0::US0_Quoted(v7) => {
            let v7: Rc<str> = v7;
            let v8: bool = cfg!(windows);
            let v11: Rc<str> = if v8 {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%\"\n"); } LIT.with(|lit| lit.clone()) };
                v9
            } else {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$`\"\\\n"); } LIT.with(|lit| lit.clone()) };
                v10
            };
            any_letter_of_10(v7, v11)
        }
    }
}
fn syntax_word_8(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_NoMoreWords => {
                let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v1;
            }
            UH0::UH0_WordThen(v2, v3) => {
                let mut v2: US0 = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = word_needs_shell_9(v2.clone());
                if v4 {
                    match v2 {
                        US0::US0_Bare(v5) => {
                            let v5: Rc<str> = v5;
                            return v5;
                        }
                        US0::US0_Quoted(v6) => {
                            let mut v6: Rc<str> = v6;
                            return v6;
                        }
                    }
                } else {
                    v0 = v3;
                    continue;
                }
            }
        }
    }
}
fn named_in_11(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (v1).split(char::from(124u8)).any(|name| name.eq_ignore_ascii_case(&*v0));
    v2
}
fn names_directory_12(mut v0: Rc<str>) -> bool {
    let mut v1: bool = (v0).contains(char::from(47u8)) || (v0).contains(char::from(92u8));
    v1
}
fn path_entries_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()).map(|dir| dir.to_string_lossy().into_owned()).collect::<Vec<_>>().join(&*v0));
    v1
}
fn has_extension_14(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::path::Path::new(&*v0).extension().is_some();
    v1
}
fn get_environment_variable_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn pathext_15() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PATHEXT"); } LIT.with(|lit| lit.clone()) };
    get_environment_variable_16(v0)
}
fn first_line_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
    v1
}
fn path_joined_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(format!("{}{}", &*v1, &*v2)).to_string_lossy().as_ref());
    v3
}
fn is_file_21(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn rest_lines_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
    v1
}
fn candidate_in_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: Rc<str> = first_line_18(v2.clone());
        let v4: Rc<str> = path_joined_20(v0.clone(), v1.clone(), v3);
        let v5: bool = is_file_21(v4.clone());
        if v5 {
            return v4;
        } else {
            let mut v6: Rc<str> = rest_lines_22(v2.clone());
            let v7: bool = &*v6 == "";
            if v7 {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v8;
            } else {
                let v9: Rc<str> = rest_lines_22(v2);
                v2 = v9;
                continue;
            }
        }
    }
}
fn program_in_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: bool = &*v0 == "";
        if v3 {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v4;
        } else {
            let v5: Rc<str> = first_line_18(v0.clone());
            let mut v6: bool = &*v5 == "";
            let v9: Rc<str> = if v6 {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v7
            } else {
                candidate_in_19(v5, v1.clone(), v2.clone())
            };
            let v10: bool = &*v9 != "";
            if v10 {
                return v9;
            } else {
                let v11: Rc<str> = rest_lines_22(v0);
                v0 = v11;
                continue;
            }
        }
    }
}
fn lowered_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    v1
}
fn closure2() -> Rc<dyn Fn(US0) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(US0) -> Rc<str>> = Rc::new(move |mut v0: US0| -> Rc<str> {
        match v0 {
            US0::US0_Bare(v1) => {
                let mut v1: Rc<str> = v1;
                v1
            }
            US0::US0_Quoted(v2) => {
                let mut v2: Rc<str> = v2;
                v2
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn launch_parts_of_6(mut v0: Rc<str>, mut v1: Rc<UH0>) -> (Rc<str>, Rc<str>) {
    let (mut v143, mut v144): (Rc<str>, Rc<str>) = match &*v1 {
        UH0::UH0_NoMoreWords => {
            let mut v2: bool = cfg!(windows);
            let v13: Rc<str> = if v2 {
                let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v4: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = words_joined_7(v1.clone(), v3, v4, v5);
                let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd.exe\n/c\n"); } LIT.with(|lit| lit.clone()) }, v6));
                v7
            } else {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v9: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v11: Rc<str> = words_joined_7(v1.clone(), v8, v9, v10);
                let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh\n-c\n"); } LIT.with(|lit| lit.clone()) }, v11));
                v12
            };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = words_joined_7(v1, v14, v15, v16);
            (v13, v17)
        }
        _ => {
            let (mut v141, mut v142): (Rc<str>, Rc<str>) = match &*v1 {
                UH0::UH0_WordThen(v18, v19) => {
                    let v18: US0 = v18.clone();
                    let v19: Rc<UH0> = v19.clone();
                    let v20: Rc<str> = syntax_word_8(v1.clone());
                    let v24: Rc<str> = match v18 {
                        US0::US0_Bare(v21) => {
                            let v21: Rc<str> = v21;
                            v21
                        }
                        US0::US0_Quoted(v22) => {
                            let v22: Rc<str> = v22;
                            v22
                        }
                    };
                    let v25: bool = &*v20 != "";
                    if v25 {
                        let v26: bool = cfg!(windows);
                        let v37: Rc<str> = if v26 {
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                            let v28: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v30: Rc<str> = words_joined_7(v1.clone(), v27, v28, v29);
                            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd.exe\n/c\n"); } LIT.with(|lit| lit.clone()) }, v30));
                            v31
                        } else {
                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                            let v33: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v35: Rc<str> = words_joined_7(v1.clone(), v32, v33, v34);
                            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh\n-c\n"); } LIT.with(|lit| lit.clone()) }, v35));
                            v36
                        };
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = words_joined_7(v1, v38, v39, v40);
                        (v37, v41)
                    } else {
                        let v42: bool = cfg!(windows);
                        let v45: bool = if v42 {
                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assoc|Break|Call|Cd|Chdir|Cls|Color|Copy|Date|Del|Dir|Echo|Endlocal|Erase|Exit|For|Ftype|Goto|If|Md|Mkdir|Mklink|Move|Path|Pause|Popd|Prompt|Pushd|Rd|Rem|Ren|Rename|Rmdir|Set|Setlocal|Shift|Start|Time|Title|Type|Ver|Verify|Vol"); } LIT.with(|lit| lit.clone()) };
                            named_in_11(v24.clone(), v43)
                        } else {
                            false
                        };
                        let (mut v137, mut v138): (Rc<str>, Rc<str>) = if v45 {
                            let v46: bool = cfg!(windows);
                            let v57: Rc<str> = if v46 {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                let v48: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v50: Rc<str> = words_joined_7(v1.clone(), v47, v48, v49);
                                let v51: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd.exe\n/c\n"); } LIT.with(|lit| lit.clone()) }, v50));
                                v51
                            } else {
                                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                let v53: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v55: Rc<str> = words_joined_7(v1.clone(), v52, v53, v54);
                                let v56: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh\n-c\n"); } LIT.with(|lit| lit.clone()) }, v55));
                                v56
                            };
                            let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                            let v59: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                            let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v61: Rc<str> = words_joined_7(v1, v58, v59, v60);
                            (v57, v61)
                        } else {
                            let v62: bool = names_directory_12(v24.clone());
                            let v71: Rc<str> = if v62 {
                                v0
                            } else {
                                let v63: bool = cfg!(windows);
                                if v63 {
                                    let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let v66: Rc<str> = path_entries_13(v65);
                                    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v66));
                                    v67
                                } else {
                                    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    path_entries_13(v68)
                                }
                            };
                            let v72: bool = cfg!(windows);
                            let v75: bool = if v72 {
                                let v73: bool = has_extension_14(v24.clone());
                                let v74: bool = v73 == false;
                                v74
                            } else {
                                false
                            };
                            let v84: Rc<str> = if v75 {
                                let v76: Rc<str> = pathext_15();
                                let v77: bool = &*v76 == "";
                                let v79: Rc<str> = if v77 {
                                    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".COM;.EXE;.BAT;.CMD"); } LIT.with(|lit| lit.clone()) };
                                    v78
                                } else {
                                    v76
                                };
                                let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                                let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v82: Rc<str> = Rc::<str>::from(v79.replace(&*v80, &*v81));
                                v82
                            } else {
                                let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v83
                            };
                            let v85: Rc<str> = program_in_17(v71, v24, v84);
                            let v86: bool = &*v85 == "";
                            let (mut v135, mut v136): (Rc<str>, Rc<str>) = if v86 {
                                let v87: bool = cfg!(windows);
                                let v98: Rc<str> = if v87 {
                                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    let v89: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v91: Rc<str> = words_joined_7(v1.clone(), v88, v89, v90);
                                    let v92: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd.exe\n/c\n"); } LIT.with(|lit| lit.clone()) }, v91));
                                    v92
                                } else {
                                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    let v94: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v96: Rc<str> = words_joined_7(v1.clone(), v93, v94, v95);
                                    let v97: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh\n-c\n"); } LIT.with(|lit| lit.clone()) }, v96));
                                    v97
                                };
                                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                let v100: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v102: Rc<str> = words_joined_7(v1, v99, v100, v101);
                                (v98, v102)
                            } else {
                                let v103: bool = cfg!(windows);
                                let v110: bool = if v103 {
                                    let v104: Rc<str> = lowered_23(v85.clone());
                                    let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".exe"); } LIT.with(|lit| lit.clone()) };
                                    let v106: bool = v104.ends_with(&*v105);
                                    if v106 {
                                        true
                                    } else {
                                        let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".com"); } LIT.with(|lit| lit.clone()) };
                                        let v108: bool = v104.ends_with(&*v107);
                                        v108
                                    }
                                } else {
                                    true
                                };
                                let (mut v133, mut v134): (Rc<str>, Rc<str>) = if v110 {
                                    let v111: US0 = US0::US0_Bare(v85);
                                    let v112: Rc<UH0> = Rc::new(UH0::UH0_WordThen(v111, v19));
                                    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let v114: Rc<dyn Fn(US0) -> Rc<str>> = closure2();
                                    let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v116: Rc<str> = words_joined_7(v112, v113, v114, v115.clone());
                                    (v116, v115)
                                } else {
                                    let v117: bool = cfg!(windows);
                                    let v128: Rc<str> = if v117 {
                                        let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                        let v119: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                        let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v121: Rc<str> = words_joined_7(v1.clone(), v118, v119, v120);
                                        let v122: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd.exe\n/c\n"); } LIT.with(|lit| lit.clone()) }, v121));
                                        v122
                                    } else {
                                        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                        let v124: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v126: Rc<str> = words_joined_7(v1.clone(), v123, v124, v125);
                                        let v127: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh\n-c\n"); } LIT.with(|lit| lit.clone()) }, v126));
                                        v127
                                    };
                                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    let v130: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v132: Rc<str> = words_joined_7(v1, v129, v130, v131);
                                    (v128, v132)
                                };
                                (v133, v134)
                            };
                            (v135, v136)
                        };
                        (v137, v138)
                    }
                }
                _ => unreachable!(),
            };
            (v141, v142)
        }
    };
    (v143, v144)
}
fn launch_argv_of_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: i32 = 0i32;
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMoreWords); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = words_scanned_1(v1, v2, v3);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMoreWords); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = words_flipped_5(v4, v5);
    let (mut v7, mut v8): (Rc<str>, Rc<str>) = launch_parts_of_6(v0, v6);
    v7
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        launch_argv_of_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn shell_line_of_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: i32 = 0i32;
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMoreWords); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = words_scanned_1(v1, v2, v3);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMoreWords); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = words_flipped_5(v4, v5);
    let (mut v7, mut v8): (Rc<str>, Rc<str>) = launch_parts_of_6(v0, v6);
    v8
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        shell_line_of_24(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_launch_argv(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_launch_line(v0: &str, v1: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
