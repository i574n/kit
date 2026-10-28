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
                let mut v36: Result<(), std::io::Error> = std::fs::create_dir_all(&*v34);
                let v38: bool = v36.is_err();
                let v39: bool = v38 == false;
                if v39 {
                    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v13));
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    agile\n    prompt\n"); } LIT.with(|lit| lit.clone()) };
                    let v42: bool = std::fs::write(&*v40, &*v41).is_ok();
                    if v42 {
                        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v3));
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\n"); } LIT.with(|lit| lit.clone()) };
                        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                        let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\nunion agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                        let v53: bool = std::fs::write(&*v43, &*v52).is_ok();
                        if v53 {
                            let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v23));
                            let v55: u64 = v2.wrapping_sub(5100000u64);
                            let v56: Rc<str> = std::rc::Rc::<str>::from((v55).to_string());
                            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, "); } LIT.with(|lit| lit.clone()) };
                            let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v56));
                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) };
                            let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
                            let v61: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, LeaseActive)\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v63));
                            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\n"); } LIT.with(|lit| lit.clone()) };
                            let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v64));
                            let v67: bool = std::fs::write(&*v54, &*v66).is_ok();
                            if v67 {
                                let v68: u64 = v41.len() as u64;
                                let v72: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v44, v45)), v47)), v49)), v51));
                                let v73: u64 = v72.len() as u64;
                                let v74: Rc<str> = std::rc::Rc::<str>::from((v55).to_string());
                                let v76: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v57, v74)), v59));
                                let v77: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                                let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v65, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v76, v77)), v63))));
                                let v81: u64 = v80.len() as u64;
                                let v82: u64 = v68 + v73 + v81;
                                let v83: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
                                let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile init ok root="); } LIT.with(|lit| lit.clone()) };
                                let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v1));
                                let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files=3 bytes="); } LIT.with(|lit| lit.clone()) };
                                let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v86));
                                let v88: Rc<str> = std::rc::Rc::<str>::from((v82).to_string());
                                let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v88));
                                let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v90));
                                let v92: Rc<str> = std::rc::Rc::<str>::from((v83).to_string());
                                let v93: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v92));
                                let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" next=kit agile begin <root> <title>"); } LIT.with(|lit| lit.clone()) };
                                let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v94));
                                let v96: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v95.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                v96
                            } else {
                                let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/prompt.spi"); } LIT.with(|lit| lit.clone()) };
                                let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                                let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v1));
                                let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let v102: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v99, v100)), v97));
                                let v103: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v102 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v103
                            }
                        } else {
                            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                            let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                            let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v1));
                            let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let v110: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v107, v108)), v105));
                            let v111: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v110 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v111
                        }
                    } else {
                        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
                        let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                        let v115: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v1));
                        let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let v118: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v115, v116)), v113));
                        let v119: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v118 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v119
                    }
                } else {
                    let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not create the state directory"); } LIT.with(|lit| lit.clone()) };
                    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                    let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v1));
                    let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v126: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v123, v124)), v121));
                    let v127: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v126 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v127
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
