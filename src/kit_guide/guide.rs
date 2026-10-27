#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_BetweenBlocks,
    US1_InHabit,
    US1_InCommand,
}
#[derive(Clone)]
enum US2 {
    US2_Written,
    US2_WriteFailed(Rc<str>),
}
fn command_line_argument_count_2() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_1(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_2();
        let v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US0 = if v5 {
        let v6: Rc<str> = command_line_argument_3(v1);
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn guided_4(mut v0: Rc<str>, mut v1: i32, mut v2: US1, mut v3: i32, mut v4: Rc<str>) -> (i32, Rc<str>) {
    let v5: i32 = (v0.clone().len() as i32);
    let v6: bool = v1 >= v5;
    let (mut v71, mut v72): (i32, Rc<str>) = if v6 {
        (v3, v4.clone())
    } else {
        let v7: i32 = (v0).get(v1 as usize..).and_then(|rest| rest.find(char::from(10u8))).map(|at| at as i32 + v1).unwrap_or((v0).len() as i32);
        let v8: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize..v7 as usize).unwrap_or(""));
        let (mut v65, mut v66, mut v67): (US1, i32, Rc<str>) = match &v2 {
            US1::US1_BetweenBlocks => {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("## "); } LIT.with(|lit| lit.clone()) };
                let v32: bool = v8.starts_with(&*v31);
                if v32 {
                    let v33: i32 = v3.wrapping_add(1i32);
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v34)), v8)), v34));
                    let v38: US1 = US1::US1_BetweenBlocks;
                    (v38.clone(), v33, v37.clone())
                } else {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Habit"); } LIT.with(|lit| lit.clone()) };
                    let v40: bool = v8.starts_with(&*v39);
                    if v40 {
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nInstead of: "); } LIT.with(|lit| lit.clone()) };
                        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v41)), v8));
                        let v44: US1 = US1::US1_InHabit;
                        (v44.clone(), v3, v43.clone())
                    } else {
                        let v45: bool = &*v8 == "```command";
                        if v45 {
                            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nOne call:\n\n```sh\n"); } LIT.with(|lit| lit.clone()) };
                            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v46));
                            let v48: US1 = US1::US1_InCommand;
                            (v48.clone(), v3, v47.clone())
                        } else {
                            let v49: US1 = US1::US1_BetweenBlocks;
                            (v49.clone(), v3, v4.clone())
                        }
                    }
                }
            }
            US1::US1_InCommand => {
                let v20: bool = &*v8 == "```";
                if v20 {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```\n"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v21));
                    let v23: US1 = US1::US1_BetweenBlocks;
                    (v23.clone(), v3, v22.clone())
                } else {
                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v8));
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
                    let v27: US1 = US1::US1_InCommand;
                    (v27.clone(), v3, v26.clone())
                }
            }
            US1::US1_InHabit => {
                let v9: bool = &*v8 == "";
                if v9 {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v10));
                    let v12: US1 = US1::US1_BetweenBlocks;
                    (v12.clone(), v3, v11.clone())
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v13)), v8));
                    let v16: US1 = US1::US1_InHabit;
                    (v16.clone(), v3, v15.clone())
                }
            }
        };
        let v68: i32 = v7.wrapping_add(1i32);
        let (mut v69, mut v70): (i32, Rc<str>) = guided_4(v0.clone(), v68, v65.clone(), v66, v67.clone());
        (v69, v70.clone())
    };
    (v71, v72.clone())
}
fn guide_run_0() -> i32 {
    let v0: i32 = 2i32;
    let v1: Rc<str> = argument_1(v0);
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v1).unwrap_or_default().replace(char::from(13u8), "").as_str());
    let v3: bool = &*v2 == "";
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: guide needs a readable spec file; usage: guide <spec/one-call.md> [--out <file.md>]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", &*v4);
        2i32
    } else {
        let v5: i32 = 0i32;
        let v6: US1 = US1::US1_BetweenBlocks;
        let v7: i32 = 0i32;
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# kit one-call guide\n\nGenerated by kit guide from spec/one-call.md: per intent, the habit it replaces and the one kit call. Do not edit by hand; regenerate.\n"); } LIT.with(|lit| lit.clone()) };
        let (mut v9, mut v10): (i32, Rc<str>) = guided_4(v2.clone(), v5, v6.clone(), v7, v8.clone());
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--out"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v11).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
        let v13: bool = &*v12 == "";
        if v13 {
            { use std::io::Write; let _ = std::io::stdout().lock().write_all((v10).as_bytes()); };
            0i32
        } else {
            let v14: Rc<str> = std::rc::Rc::<str>::from({ let target = std::path::Path::new(&*v12); target.parent().filter(|parent| !parent.as_os_str().is_empty()).map(std::fs::create_dir_all).transpose().and_then(|_| std::fs::write(target, (v10).as_bytes())).err().map(|error| error.to_string()).unwrap_or_default() }.as_str());
            let v15: bool = &*v14 == "";
            let v18: US2 = if v15 {
                US2::US2_Written
            } else {
                US2::US2_WriteFailed(v14.clone())
            };
            match &v18 {
                US2::US2_WriteFailed(v25) => {
                    let v25: Rc<str> = v25.clone();
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: guide could not write "); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v12));
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v27, v28)), v25));
                    eprintln!("{}", &*v30);
                    2i32
                }
                US2::US2_Written => {
                    let v19: Rc<str> = std::rc::Rc::<str>::from((v9).to_string().as_str());
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit guide ok intents="); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v21, v22)), v12));
                    println!("{}", &*v24);
                    0i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        guide_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_guide_run() -> i32 {
    closure0()()
}
