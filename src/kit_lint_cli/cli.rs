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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_LintMacros,
    US1_LintShadows,
    US1_LintGrowth,
}
fn growth_routed_1(mut v0: Rc<UH0>) -> i32 {
    match &*v0 {
        UH0::UH0_Cons(v1, v2) => {
            let v1: Rc<str> = v1.clone();
            let v2: Rc<UH0> = v2.clone();
            match &*v2 {
                UH0::UH0_Cons(v3, v4) => {
                    let v3: Rc<str> = v3.clone();
                    let v4: Rc<UH0> = v4.clone();
                    match &*v4 {
                        UH0::UH0_Cons(v5, v6) => {
                            let v5: Rc<str> = v5.clone();
                            let v6: Rc<UH0> = v6.clone();
                            match &*v6 {
                                UH0::UH0_Cons(v7, v8) => {
                                    let v7: Rc<str> = v7.clone();
                                    let v8: Rc<UH0> = v8.clone();
                                    match &*v8 {
                                        UH0::UH0_Nil => {
                                            let v9: i32 = kit_lint_growth::kit_lint_growth_run(&v1, &v3, &v5, &v7);
                                            v9
                                        }
                                        _ => {
                                            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: lint growth <baseline-repo> <baseline.spi> <scan-repo> <scan-dir>"); } LIT.with(|lit| lit.clone()) };
                                            eprintln!("{}", v10);
                                            2i32
                                        }
                                    }
                                }
                                _ => {
                                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: lint growth <baseline-repo> <baseline.spi> <scan-repo> <scan-dir>"); } LIT.with(|lit| lit.clone()) };
                                    eprintln!("{}", v12);
                                    2i32
                                }
                            }
                        }
                        _ => {
                            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: lint growth <baseline-repo> <baseline.spi> <scan-repo> <scan-dir>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v14);
                            2i32
                        }
                    }
                }
                _ => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: lint growth <baseline-repo> <baseline.spi> <scan-repo> <scan-dir>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v16);
                    2i32
                }
            }
        }
        _ => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: lint growth <baseline-repo> <baseline.spi> <scan-repo> <scan-dir>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v18);
            2i32
        }
    }
}
fn command_line_argument_count_3() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_2(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_3();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_4(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_2(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn lint_command_of_5(mut v0: Rc<str>) -> US1 {
    let v1: bool = "macros" == &*v0;
    if v1 {
        US1::US1_LintMacros
    } else {
        let v3: bool = "shadows" == &*v0;
        if v3 {
            US1::US1_LintShadows
        } else {
            let v5: bool = "growth" == &*v0;
            if v5 {
                US1::US1_LintGrowth
            } else {
                US1::US1_LintMacros
            }
        }
    }
}
fn lint_cli_run_0() -> i32 {
    let v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_2(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4.clone();
                    let v5: Rc<UH0> = v5.clone();
                    let v6: US1 = lint_command_of_5(v4);
                    match &v6 {
                        US1::US1_LintGrowth => {
                            growth_routed_1(v5)
                        }
                        US1::US1_LintMacros => {
                            let v9: i32 = kit_lint_macros::kit_lint_run();
                            v9
                        }
                        US1::US1_LintShadows => {
                            let v8: i32 = kit_lint_macros::kit_lint_shadows_routed();
                            v8
                        }
                    }
                }
                _ => {
                    let v13: i32 = kit_lint_macros::kit_lint_run();
                    v13
                }
            }
        }
        _ => {
            let v15: i32 = kit_lint_macros::kit_lint_run();
            v15
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        lint_cli_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_cli_run() -> i32 {
    closure0()()
}
