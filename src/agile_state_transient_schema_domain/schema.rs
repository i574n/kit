#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn agile_transient_schema_code_0(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch_resume.spi\n"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PatchResumeReceipt ("); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.contains(&*v3);
        let v7: bool = if v4 {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl main () : i32 = 0i32"); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v0.contains(&*v5);
            v6
        } else {
            false
        };
        if v7 {
            1u64
        } else {
            0u64
        }
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain_process_receipt.spi\n"); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v0.starts_with(&*v9);
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n// toolchain-process-receipt|"); } LIT.with(|lit| lit.clone()) };
            let v12: bool = v0.contains(&*v11);
            let v15: bool = if v12 {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ToolchainProcessReceipt ("); } LIT.with(|lit| lit.clone()) };
                let v14: bool = v0.contains(&*v13);
                v14
            } else {
                false
            };
            let v18: bool = if v15 {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl main () : i32 = 0i32"); } LIT.with(|lit| lit.clone()) };
                let v17: bool = v0.contains(&*v16);
                v17
            } else {
                false
            };
            if v18 {
                1u64
            } else {
                0u64
            }
        } else {
            0u64
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_transient_schema_code_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn state_path_candidate_code_2(mut v0: Rc<str>) -> u64 {
    let v1: bool = &*v0 == "typecheck_receipts.spi";
    if v1 {
        0u64
    } else {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v3: bool = v0.ends_with(&*v2);
        if v3 {
            1u64
        } else {
            0u64
        }
    }
}
fn agile_state_path_candidate_code_1(mut v0: Rc<str>) -> u64 {
    state_path_candidate_code_2(v0)
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        agile_state_path_candidate_code_1(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_state_transient_schema_code(v0: &str) -> u64 {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_agile_state_path_candidate_code(v0: &str) -> u64 {
    closure1()(Rc::<str>::from(v0))
}
