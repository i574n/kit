#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_ToolRust,
    US1_ToolZig,
    US1_ToolOtp,
    US1_ToolElixir,
    US1_ToolGleam,
    US1_ToolRebar,
    US1_ToolPython,
}
#[derive(Clone)]
enum US0 {
    US0_Some(US1),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Matches(Rc<str>),
    US3_Differs(Rc<str>),
    US3_MissingLocally,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
fn probe_local_0(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, Rc<str>) {
    let (mut v2, mut v3): (bool, Rc<str>) = kit_ci_probe::kit_ci_local_version(&*v0, &*v1);
    (v2, v3)
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> {
    Rc::new(move |mut v1: Rc<str>| -> (bool, Rc<str>) {
        probe_local_0(v0.clone(), v1.clone())
    })
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)>> = Rc::new(move |mut v0: Rc<str>| -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> {
        closure2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn rust_installed_1(mut v0: Rc<str>) -> bool {
    let v1: bool = kit_ci_probe::kit_ci_rust_installed(&*v0);
    v1
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        rust_installed_1(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tool_of_name_3(mut v0: Rc<str>) -> US0 {
    let v1: bool = &*v0 == "rust";
    if v1 {
        let v2: US1 = US1::US1_ToolRust;
        US0::US0_Some(v2)
    } else {
        let v4: bool = &*v0 == "zig";
        if v4 {
            let v5: US1 = US1::US1_ToolZig;
            US0::US0_Some(v5)
        } else {
            let mut v7: bool = &*v0 == "otp";
            if v7 {
                let v8: US1 = US1::US1_ToolOtp;
                US0::US0_Some(v8)
            } else {
                let v10: bool = &*v0 == "elixir";
                if v10 {
                    let v11: US1 = US1::US1_ToolElixir;
                    US0::US0_Some(v11)
                } else {
                    let v13: bool = &*v0 == "gleam";
                    if v13 {
                        let mut v14: US1 = US1::US1_ToolGleam;
                        US0::US0_Some(v14)
                    } else {
                        let v16: bool = &*v0 == "rebar3";
                        if v16 {
                            let v17: US1 = US1::US1_ToolRebar;
                            US0::US0_Some(v17)
                        } else {
                            let v19: bool = &*v0 == "python";
                            if v19 {
                                let v20: US1 = US1::US1_ToolPython;
                                US0::US0_Some(v20)
                            } else {
                                US0::US0_None
                            }
                        }
                    }
                }
            }
        }
    }
}
fn pin_matches_5(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    match &v0 {
        US1::US1_ToolRust => {
            let v3: bool = &*v1 == &*v2;
            v3
        }
        _ => {
            let v4: bool = &*v1 == &*v2;
            if v4 {
                true
            } else {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v5));
                let mut v7: bool = v2.starts_with(&*v6);
                v7
            }
        }
    }
}
fn local_verdict_4(mut v0: US1, mut v1: Rc<str>, mut v2: US2) -> US3 {
    match &v2 {
        US2::US2_None => {
            US3::US3_MissingLocally
        }
        US2::US2_Some(v4) => {
            let v4: Rc<str> = v4.clone();
            let v5: bool = pin_matches_5(v0, v1, v4.clone());
            if v5 {
                US3::US3_Matches(v4.clone())
            } else {
                US3::US3_Differs(v4)
            }
        }
    }
}
fn fix_line_7(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    match &v0 {
        US1::US1_ToolRust => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fix: rustup toolchain install "); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v1));
            v6.clone()
        }
        US1::US1_ToolZig => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fix: SPIRAL_ZIG="); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
            v4.clone()
        }
        _ => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("candidate: "); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v2));
            v8.clone()
        }
    }
}
fn method11(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method11(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn directory_exists_13(mut v0: Rc<str>) -> bool {
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
fn names_in_12(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: bool = directory_exists_13(v0.clone());
    let v2: bool = v1 == false;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v7: Vec<std::string::String> = v5.unwrap();
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
        let mut v12: Rc<UH0> = acc;
        let mut v14: std::string::String = x;
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16.clone(), v12.clone()));
        let v19: bool = true; v18 });
        let v21: Rc<UH0> = _vec_fold_;
        v21.clone()
    }
}
fn method15(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<UH0>) -> US2 {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: bool = pin_matches_5(v0.clone(), v1.clone(), v4.clone());
                if v6 {
                    return US2::US2_Some(v4.clone());
                } else {
                    (v0, v1, v2) = (v0.clone(), v1.clone(), v5.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn matching_version_14(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<UH0>) -> US2 {
    method15(v0, v1, v2)
}
fn file_exists_16(mut v0: Rc<str>) -> bool {
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
fn method9(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<UH0>, mut v5: US2) -> US2 {
    loop {
        match &*v4 {
            UH0::UH0_Cons(v6, v7) => {
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH0> = v7.clone();
                let v27: US2 = match &v5 {
                    US2::US2_None => {
                        let v9: Rc<str> = method10(v2.clone(), v6.clone());
                        let v10: Rc<UH0> = names_in_12(v9.clone());
                        let v11: US2 = matching_version_14(v0.clone(), v1.clone(), v10.clone());
                        match &v11 {
                            US2::US2_None => {
                                US2::US2_None
                            }
                            US2::US2_Some(v12) => {
                                let mut v12: Rc<str> = v12.clone();
                                let v13: Rc<str> = method10(v2.clone(), v6.clone());
                                let mut v14: Rc<str> = method10(v13.clone(), v12.clone());
                                let v15: Rc<str> = method10(v14.clone(), v3.clone());
                                let v16: bool = file_exists_16(v15.clone());
                                if v16 {
                                    let v17: Rc<str> = method10(v2.clone(), v6.clone());
                                    let v18: Rc<str> = method10(v17.clone(), v12.clone());
                                    let v19: Rc<str> = method10(v18.clone(), v3.clone());
                                    US2::US2_Some(v19.clone())
                                } else {
                                    US2::US2_None
                                }
                            }
                        }
                    }
                    US2::US2_Some(v8) => {
                        let v8: Rc<str> = v8.clone();
                        v5.clone()
                    }
                };
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v7.clone(), v27.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v5.clone();
            }
        }
    }
}
fn get_environment_variable_18(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn scoop_apps_17() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("USERPROFILE"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = get_environment_variable_18(v0.clone());
    let v2: bool = &*v1 == "";
    let v5: US2 = if v2 {
        US2::US2_None
    } else {
        US2::US2_Some(v1.clone())
    };
    let v9: Rc<str> = match &v5 {
        US2::US2_None => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US2::US2_Some(v6) => {
            let mut v6: Rc<str> = v6.clone();
            v6.clone()
        }
    };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("scoop"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = method10(v9.clone(), v10.clone());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apps"); } LIT.with(|lit| lit.clone()) };
    method10(v11.clone(), v12.clone())
}
fn tool_name_19(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_ToolElixir => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elixir"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US1::US1_ToolGleam => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US1::US1_ToolOtp => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("otp"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US1::US1_ToolPython => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US1::US1_ToolRebar => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rebar3"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US1::US1_ToolRust => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US1::US1_ToolZig => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
    }
}
fn method20(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<UH0> = v4.clone();
            let v5: Rc<UH0> = method20(v0.clone(), v4, v2.clone());
            let mut v6: bool = v3.starts_with(&*v0);
            if v6 {
                Rc::new(UH0::UH0_Cons(v3, v5.clone()))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn scoop_match_8(mut v0: US1, mut v1: Rc<str>) -> US2 {
    let v2: Rc<str> = scoop_apps_17();
    let v3: Rc<str> = tool_name_19(v0.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".exe"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let mut v6: Rc<UH0> = names_in_12(v2.clone());
    let mut v7: Rc<str> = tool_name_19(v0.clone());
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<UH0> = method20(v7.clone(), v6.clone(), v8.clone());
    let v10: US2 = US2::US2_None;
    method9(v0.clone(), v1.clone(), v2.clone(), v5.clone(), v9.clone(), v10.clone())
}
fn fix_for_6(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: US1, mut v2: Rc<str>, mut v3: US3) -> US2 {
    match &v3 {
        US3::US3_Matches(v4) => {
            let v4: Rc<str> = v4.clone();
            US2::US2_None
        }
        _ => {
            match &v1 {
                US1::US1_ToolRust => {
                    let mut v6: bool = v0(v2.clone());
                    if v6 {
                        US2::US2_None
                    } else {
                        let v8: US1 = US1::US1_ToolRust;
                        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v10: Rc<str> = fix_line_7(v8.clone(), v2.clone(), v9.clone());
                        US2::US2_Some(v10.clone())
                    }
                }
                _ => {
                    let v13: US2 = scoop_match_8(v1.clone(), v2.clone());
                    match &v13 {
                        US2::US2_None => {
                            US2::US2_None
                        }
                        US2::US2_Some(v14) => {
                            let mut v14: Rc<str> = v14.clone();
                            let v15: Rc<str> = fix_line_7(v1.clone(), v2.clone(), v14.clone());
                            US2::US2_Some(v15.clone())
                        }
                    }
                }
            }
        }
    }
}
fn verdict_line_21(mut v0: US1, mut v1: Rc<str>, mut v2: US3) -> Rc<str> {
    let (mut v13, mut v14): (Rc<str>, Rc<str>) = match &v2 {
        US3::US3_Differs(v5) => {
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("differs"); } LIT.with(|lit| lit.clone()) };
            (v5.clone(), v6.clone())
        }
        US3::US3_Matches(v3) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("same"); } LIT.with(|lit| lit.clone()) };
            (v3.clone(), v4.clone())
        }
        US3::US3_MissingLocally => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing"); } LIT.with(|lit| lit.clone()) };
            (v7.clone(), v8.clone())
        }
    };
    let v15: Rc<str> = tool_name_19(v0.clone());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ci="); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v15, v16)), v1));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" local="); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v19)), v13));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v21, v22)), v14));
    v24.clone()
}
fn differs_22(mut v0: US3) -> bool {
    match &v0 {
        US3::US3_Matches(v1) => {
            let v1: Rc<str> = v1.clone();
            false
        }
        _ => {
            true
        }
    }
}
fn row_of_2(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)>>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> (Rc<str>, bool) {
    let v5: US0 = tool_of_name_3(v2.clone());
    match &v5 {
        US0::US0_None => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ci="); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v6)), v3));
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" local=- unknown"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
            (v10.clone(), true)
        }
        US0::US0_Some(v11) => {
            let v11: US1 = v11.clone();
            let mut v12: Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> = v0(v2.clone());
            let (mut v13, mut v14): (bool, Rc<str>) = v12(v4.clone());
            let v17: US2 = if v13 {
                US2::US2_Some(v14.clone())
            } else {
                US2::US2_None
            };
            let v18: US3 = local_verdict_4(v11.clone(), v3.clone(), v17.clone());
            let v19: US2 = fix_for_6(v1.clone(), v11.clone(), v3.clone(), v18.clone());
            let v25: Rc<str> = match &v19 {
                US2::US2_None => {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v23.clone()
                }
                US2::US2_Some(v20) => {
                    let v20: Rc<str> = v20.clone();
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n  "); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v20));
                    v22.clone()
                }
            };
            let v26: Rc<str> = verdict_line_21(v11.clone(), v3.clone(), v18.clone());
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v25));
            let v28: bool = differs_22(v18.clone());
            (v27.clone(), v28)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (Rc<str>, bool) {
        let v3: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)>> = closure1();
        let v4: Rc<dyn Fn(Rc<str>) -> bool> = closure3();
        row_of_2(v3.clone(), v4.clone(), v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_ci_parity_row(v0: &str, v1: &str, v2: &str) -> (Rc<str>, bool) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
