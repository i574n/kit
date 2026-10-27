#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn agile_init_entry_0() -> i32 {
    let v0: i32 = 1i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v3));
    let v5: bool = std::path::Path::new(&*v4).exists();
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi already exists"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v8, v9)), v6));
        let v12: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v11 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v12
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v13));
        let v15: bool = std::path::Path::new(&*v14).exists();
        if v15 {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj already exists"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v1));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v19)), v16));
            let v22: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v21 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v22
        } else {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/prompt.spi"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v23));
            let v25: bool = std::path::Path::new(&*v24).exists();
            if v25 {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/prompt.spi already exists"); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v1));
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v28, v29)), v26));
                let v32: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v31 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v32
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v33));
                let v35: bool = std::fs::create_dir_all(&*v34).is_ok();
                if v35 {
                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v13));
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    agile\n    prompt\n"); } LIT.with(|lit| lit.clone()) };
                    let v38: bool = std::fs::write(&*v36, &*v37).is_ok();
                    if v38 {
                        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v3));
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\nunion agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                        let v49: bool = std::fs::write(&*v39, &*v48).is_ok();
                        if v49 {
                            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v23));
                            let v51: u64 = v2.wrapping_sub(5100000u64);
                            let v52: Rc<str> = std::rc::Rc::<str>::from((v51).to_string());
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, "); } LIT.with(|lit| lit.clone()) };
                            let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v52));
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) };
                            let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v55));
                            let v57: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, LeaseActive)\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                            let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\n"); } LIT.with(|lit| lit.clone()) };
                            let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v60));
                            let v63: bool = std::fs::write(&*v50, &*v62).is_ok();
                            if v63 {
                                let v64: u64 = v37.len() as u64;
                                let v68: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v40, v41)), v43)), v45)), v47));
                                let v69: u64 = v68.len() as u64;
                                let v70: Rc<str> = std::rc::Rc::<str>::from((v51).to_string());
                                let v72: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v53, v70)), v55));
                                let v73: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                                let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v61, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v72, v73)), v59))));
                                let v77: u64 = v76.len() as u64;
                                let v78: u64 = v64 + v69 + v77;
                                let v79: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
                                let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile init ok root="); } LIT.with(|lit| lit.clone()) };
                                let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v1));
                                let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files=3 bytes="); } LIT.with(|lit| lit.clone()) };
                                let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                                let v84: Rc<str> = std::rc::Rc::<str>::from((v78).to_string());
                                let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v84));
                                let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v86));
                                let v88: Rc<str> = std::rc::Rc::<str>::from((v79).to_string());
                                let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v88));
                                let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" next=kit agile begin <root> <title>"); } LIT.with(|lit| lit.clone()) };
                                let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v90));
                                let v92: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v91.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                v92
                            } else {
                                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/prompt.spi"); } LIT.with(|lit| lit.clone()) };
                                let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                                let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v1));
                                let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let v98: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v95, v96)), v93));
                                let v99: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v98 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v99
                            }
                        } else {
                            let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                            let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                            let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v1));
                            let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let v106: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v103, v104)), v101));
                            let v107: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v106 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v107
                        }
                    } else {
                        let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
                        let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                        let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v1));
                        let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let v114: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v111, v112)), v109));
                        let v115: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v114 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v115
                    }
                } else {
                    let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not create the state directory"); } LIT.with(|lit| lit.clone()) };
                    let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                    let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v1));
                    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v122: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v119, v120)), v117));
                    let v123: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v122 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v123
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
