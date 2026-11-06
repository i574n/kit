#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_RepoAffected,
    US0_RepoAcceptReview,
    US0_RepoScope,
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
#[derive(Clone)]
enum US3 {
    US3_TableLogs,
    US3_TableSequences,
    US3_TableTest,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US3),
    US4_None,
}
fn repo_ran_1(mut v0: US0) -> i32 {
    match v0 {
        US0::US0_RepoAcceptReview => {
            let v2: i32 = kit_repo_accept_review::kit_repo_accept_review_run();
            v2
        }
        US0::US0_RepoAffected => {
            let v1: i32 = kit_repo_affected::kit_repo_affected_run();
            v1
        }
        US0::US0_RepoScope => {
            let v3: i32 = kit_proof_scope::kit_proof_scope_run();
            v3
        }
    }
}
fn command_line_argument_count_4() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_5(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_3(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_4();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US1 = if v4 {
        let v5: Rc<str> = command_line_argument_5(v0);
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
            let v12: Rc<UH0> = arguments_from_3(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn subcommand_2() -> Rc<str> {
    let v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_3(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    v4
                }
                _ => {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v6
                }
            }
        }
        _ => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8
        }
    }
}
fn repo_named_6(mut v0: Rc<str>) -> US2 {
    let v1: bool = "affected" == &*v0;
    if v1 {
        let v2: US0 = US0::US0_RepoAffected;
        US2::US2_Some(v2)
    } else {
        let v4: bool = "accept-review" == &*v0;
        if v4 {
            let v5: US0 = US0::US0_RepoAcceptReview;
            US2::US2_Some(v5)
        } else {
            let v7: bool = "scope" == &*v0;
            if v7 {
                let v8: US0 = US0::US0_RepoScope;
                US2::US2_Some(v8)
            } else {
                US2::US2_None
            }
        }
    }
}
fn repo_routed_0() -> i32 {
    let v0: Rc<str> = subcommand_2();
    let v1: US2 = repo_named_6(v0);
    match v1 {
        US2::US2_None => {
            let v4: i32 = kit_repo_state::kit_repo_run();
            v4
        }
        US2::US2_Some(v2) => {
            let v2: US0 = v2;
            repo_ran_1(v2)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        repo_routed_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn table_ran_8(mut v0: US3) -> i32 {
    match v0 {
        US3::US3_TableLogs => {
            let v1: i32 = kit_table_logs::kit_table_logs_run();
            v1
        }
        US3::US3_TableSequences => {
            let v2: i32 = kit_table_sequences::kit_table_sequences_run();
            v2
        }
        US3::US3_TableTest => {
            let v3: i32 = kit_table_test::kit_table_test_run();
            v3
        }
    }
}
fn table_named_9(mut v0: Rc<str>) -> US4 {
    let v1: bool = "logs" == &*v0;
    if v1 {
        let v2: US3 = US3::US3_TableLogs;
        US4::US4_Some(v2)
    } else {
        let v4: bool = "sequences" == &*v0;
        if v4 {
            let v5: US3 = US3::US3_TableSequences;
            US4::US4_Some(v5)
        } else {
            let v7: bool = "test" == &*v0;
            if v7 {
                let v8: US3 = US3::US3_TableTest;
                US4::US4_Some(v8)
            } else {
                US4::US4_None
            }
        }
    }
}
fn table_routed_7() -> i32 {
    let v0: Rc<str> = subcommand_2();
    let v1: US4 = table_named_9(v0);
    match v1 {
        US4::US4_None => {
            let v4: i32 = kit_table_query::kit_table_run();
            v4
        }
        US4::US4_Some(v2) => {
            let v2: US3 = v2;
            table_ran_8(v2)
        }
    }
}
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        table_routed_7()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_cli_repo_routed() -> i32 {
    closure0()()
}
pub fn kit_cli_table_routed() -> i32 {
    closure1()()
}
