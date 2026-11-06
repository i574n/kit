#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_MigratePreview,
    US0_MigrateApply,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(US0),
    US2_None,
}
fn migrate_usage_1() -> i32 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile usage: migrate-activity preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
    eprintln!("{}", v0);
    2i32
}
fn file_exists_3(mut v0: Rc<str>) -> bool {
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
fn method4(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn schema_module_6() -> Rc<str> {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("activity_schema"); } LIT.with(|lit| lit.clone()) };
    v0
}
fn modules_with_schema_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = schema_module_6();
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) }, v1));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v3.clone()));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v0.clone()));
    let mut v6: bool = v5.contains(&*v4);
    if v6 {
        v0
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v0.contains(&*v7);
        if v8 {
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n"); } LIT.with(|lit| lit.clone()) }, v3));
            let v10: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v9));
            v10
        } else {
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n"); } LIT.with(|lit| lit.clone()) }));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v3));
            v12
        }
    }
}
fn step_word_7(mut v0: bool, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: bool = v1 == false;
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kept"); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        if v0 {
            v2
        } else {
            v3
        }
    }
}
fn migrated_2(mut v0: US0, mut v1: Rc<str>) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) }));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/package.spiproj"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/activity_schema.spi"); } LIT.with(|lit| lit.clone()) }));
    let v8: bool = file_exists_3(v6.clone());
    let v9: bool = v8 == false;
    if v9 {
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile migrate-activity: no state/package.spiproj under "); } LIT.with(|lit| lit.clone()) }, v1));
        eprintln!("{}", v10);
        2i32
    } else {
        let v11: bool = match &v0 {
            US0::US0_MigrateApply => {
                true
            }
            _ => {
                false
            }
        };
        let v12: bool = file_exists_3(v7.clone());
        let v13: bool = v12 == false;
        let v15: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
        let v17: Vec<u8> = v15.unwrap();
        let v18: Vec<u8> = method4(v17);
        let v20: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v18);
        let v22: std::string::String = v20.unwrap();
        let v24: Rc<str> = Rc::<str>::from(String::as_str(&v22));
        let v25: Rc<str> = modules_with_schema_5(v24.clone());
        let v26: bool = v11 && v13;
        if v26 {
            let mut v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\n"); } LIT.with(|lit| lit.clone()) };
            v27.borrow_mut().push(v28);
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union activity_row =\n"); } LIT.with(|lit| lit.clone()) };
            v27.borrow_mut().push(v29);
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | StateChange :: string * string * string * string * string * string -> activity_row\n"); } LIT.with(|lit| lit.clone()) };
            v27.borrow_mut().push(v30);
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | ActualMinutes :: string * string * i32 * string -> activity_row\n"); } LIT.with(|lit| lit.clone()) };
            v27.borrow_mut().push(v31);
            let v32: Rc<Vec<Rc<str>>> = Rc::new(v27.borrow().clone());
            let v33: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v32).as_ref().clone()));
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v35: Rc<str> = Rc::<str>::from((*v33.borrow()).join(&*v34));
            std::fs::write(&*v7, &*v35).unwrap();
            ()
        };
        let v38: bool = if v11 {
            let v37: bool = &*v25 != &*v24;
            v37
        } else {
            false
        };
        if v38 {
            std::fs::write(&*v6, &*v25).unwrap();
            ()
        };
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("written"); } LIT.with(|lit| lit.clone()) };
        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("would-write"); } LIT.with(|lit| lit.clone()) };
        let v42: Rc<str> = step_word_7(v11, v13, v40, v41);
        let v43: bool = &*v25 != &*v24;
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("added"); } LIT.with(|lit| lit.clone()) };
        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("would-add"); } LIT.with(|lit| lit.clone()) };
        let v46: Rc<str> = step_word_7(v11, v43, v44, v45);
        let v49: Rc<str> = match v0 {
            US0::US0_MigrateApply => {
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
                v48
            }
            US0::US0_MigratePreview => {
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview"); } LIT.with(|lit| lit.clone()) };
                v47
            }
        };
        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" schema="); } LIT.with(|lit| lit.clone()) }, v42));
        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" module="); } LIT.with(|lit| lit.clone()) }));
        let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v46));
        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile migrate-activity ok mode="); } LIT.with(|lit| lit.clone()) }, v49));
        let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v52));
        println!("{}", v54);
        0i32
    }
}
fn command_line_argument_count_9() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_10(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_8(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_9();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US1 = if v4 {
        let v5: Rc<str> = command_line_argument_10(v0);
        US1::US1_Some(v5)
    } else {
        US1::US1_None
    };
    match v8 {
        US1::US1_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US1::US1_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_8(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn mode_named_11(mut v0: Rc<str>) -> US2 {
    let v1: bool = &*v0 == "preview";
    if v1 {
        let v2: US0 = US0::US0_MigratePreview;
        US2::US2_Some(v2)
    } else {
        let v4: bool = &*v0 == "apply";
        if v4 {
            let v5: US0 = US0::US0_MigrateApply;
            US2::US2_Some(v5)
        } else {
            US2::US2_None
        }
    }
}
fn migrate_run_0() -> i32 {
    let v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_8(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8;
                                    let v9: Rc<UH0> = v9;
                                    let v10: US2 = mode_named_11(v6);
                                    match v10 {
                                        US2::US2_None => {
                                            migrate_usage_1()
                                        }
                                        US2::US2_Some(v11) => {
                                            let v11: US0 = v11;
                                            migrated_2(v11, v8)
                                        }
                                    }
                                }
                                _ => {
                                    migrate_usage_1()
                                }
                            }
                        }
                        _ => {
                            migrate_usage_1()
                        }
                    }
                }
                _ => {
                    migrate_usage_1()
                }
            }
        }
        _ => {
            migrate_usage_1()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        migrate_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_migrate_activity() -> i32 {
    closure0()()
}
