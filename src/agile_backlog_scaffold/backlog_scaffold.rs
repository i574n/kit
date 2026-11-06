#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn agile_init_entry_0() -> i32 {
    let v0: i32 = 1i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) }));
    let v4: bool = std::path::Path::new(&*v3).exists();
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi already exists"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) }, v1));
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v5));
        let v9: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v8 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v9
    } else {
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/package.spiproj"); } LIT.with(|lit| lit.clone()) }));
        let v11: bool = std::path::Path::new(&*v10).exists();
        if v11 {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj already exists"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) }, v1));
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v12));
            let v16: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v15 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v16
        } else {
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/prompt.spi"); } LIT.with(|lit| lit.clone()) }));
            let v18: bool = std::path::Path::new(&*v17).exists();
            if v18 {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/prompt.spi already exists"); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) }, v1));
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v19));
                let v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v23
            } else {
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) }));
                let mut v26: Result<(), std::io::Error> = std::fs::create_dir_all(&*v24);
                let v28: bool = v26.is_err();
                let v29: bool = v28 == false;
                if v29 {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    agile\n    prompt\n"); } LIT.with(|lit| lit.clone()) };
                    let v31: bool = std::fs::write(&*v10, &*v30).is_ok();
                    if v31 {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\nunion agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                        let v33: bool = std::fs::write(&*v3, &*v32).is_ok();
                        if v33 {
                            let v34: u64 = v2.wrapping_sub(5100000u64);
                            let v35: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
                            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, "); } LIT.with(|lit| lit.clone()) }, v35));
                            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) }));
                            let v38: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
                            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, LeaseActive)\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) }));
                            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\n"); } LIT.with(|lit| lit.clone()) }, v40));
                            let v42: bool = std::fs::write(&*v17, &*v41).is_ok();
                            if v42 {
                                let v43: u64 = v30.len() as u64;
                                let v44: u64 = v32.len() as u64;
                                let v45: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
                                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, "); } LIT.with(|lit| lit.clone()) }, v45));
                                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) }));
                                let v48: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
                                let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, LeaseActive)\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) }));
                                let v51: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\n"); } LIT.with(|lit| lit.clone()) }, v50));
                                let v52: u64 = v51.len() as u64;
                                let v53: u64 = v43 + v44 + v52;
                                let v54: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
                                let v55: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile init ok root="); } LIT.with(|lit| lit.clone()) }, v1));
                                let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files=3 bytes="); } LIT.with(|lit| lit.clone()) }));
                                let v57: Rc<str> = std::rc::Rc::<str>::from((v53).to_string());
                                let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
                                let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
                                let v60: Rc<str> = std::rc::Rc::<str>::from((v54).to_string());
                                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v60));
                                let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" next=kit agile begin <root> <title>"); } LIT.with(|lit| lit.clone()) }));
                                let v63: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v62.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                v63
                            } else {
                                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/prompt.spi"); } LIT.with(|lit| lit.clone()) };
                                let v65: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) }, v1));
                                let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                                let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v64));
                                let v68: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v67 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v68
                            }
                        } else {
                            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                            let v71: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) }, v1));
                            let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                            let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v70));
                            let v74: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v73 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v74
                        }
                    } else {
                        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
                        let v77: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) }, v1));
                        let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                        let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v76));
                        let v80: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v79 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v80
                    }
                } else {
                    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not create the state directory"); } LIT.with(|lit| lit.clone()) };
                    let v83: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) }, v1));
                    let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v83, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                    let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v82));
                    let v86: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v85 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v86
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_init_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_init_run() -> i32 {
    closure0()()
}
