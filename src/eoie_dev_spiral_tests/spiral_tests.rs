#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum UH0 {
    UH0_NoFailures,
    UH0_FailedThen(Rc<str>, i32, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_NoMoreNotebooks,
    UH1_SpiralTestNotebook(Rc<str>, Rc<str>, Rc<UH1>),
}
fn announced_1(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v4: i32 = (v3.clone().borrow().len() as i32);
        let mut v5: bool = v4 > 0i32;
        let mut v9: bool = if v5 {
            let mut v6: i32 = v4.wrapping_sub(1i32);
            let mut v7: Rc<str> = v3.clone().borrow()[v6 as usize].clone();
            let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8
        } else {
            false
        };
        let mut v11: i32 = if v9 {
            let mut v10: i32 = v4.wrapping_sub(1i32);
            v10
        } else {
            v4
        };
        let mut v12: bool = v1 < v11;
        if v12 {
            let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let mut v14: bool = v1 >= 0i32;
            let mut v24: bool = if v14 {
                let mut v15: i32 = (v13.clone().borrow().len() as i32);
                let mut v16: bool = v15 > 0i32;
                let mut v20: bool = if v16 {
                    let mut v17: i32 = v15.wrapping_sub(1i32);
                    let mut v18: Rc<str> = v13.clone().borrow()[v17 as usize].clone();
                    let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v19
                } else {
                    false
                };
                let mut v22: i32 = if v20 {
                    let mut v21: i32 = v15.wrapping_sub(1i32);
                    v21
                } else {
                    v15
                };
                let mut v23: bool = v1 < v22;
                v23
            } else {
                false
            };
            let mut v37: Rc<str> = if v24 {
                let mut v25: Rc<str> = v13.clone().borrow()[v1 as usize].clone();
                let mut v26: i32 = (v25.clone().len() as i32);
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let mut v28: bool = v25.ends_with(&*v27);
                let mut v29: bool = v28 == false;
                if v29 {
                    v25.clone()
                } else {
                    let mut v30: bool = v26 == 1i32;
                    if v30 {
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v31.clone()
                    } else {
                        let mut v32: i32 = v26.wrapping_sub(2i32);
                        let mut v33: Rc<str> = string_slice(&v25.clone(), 0i32 as i64, v32 as i64);
                        v33.clone()
                    }
                }
            } else {
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v36.clone()
            };
            let mut v38: i32 = { std::io::Write::write_all(&mut std::io::stdout(),format!("spiral test blocks {}
",&*v37).as_bytes()).unwrap(); 0i32 };
            let mut v39: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v39);
            continue;
        } else {
            return 0i32;
        }
    }
}
fn first_code_3(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_FailedThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let mut v4: Rc<UH0> = v4.clone();
                (v0, v1) = (v4.clone(), v3);
                continue;
            }
            UH0::UH0_NoFailures => {
                return v1;
            }
        }
    }
}
fn failure_list_4(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_FailedThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = if v5 {
                    v2.clone()
                } else {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
                    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
                    v8.clone()
                };
                (v0, v1) = (v4.clone(), v9.clone());
                continue;
            }
            UH0::UH0_NoFailures => {
                return v1.clone();
            }
        }
    }
}
fn failure_count_5(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_FailedThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_NoFailures => {
                return v1;
            }
        }
    }
}
fn reported_2(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    match &*v0 {
        UH0::UH0_NoFailures => {
            let mut v2: bool = v1 == 0i32;
            if v2 {
                ()
            } else {
                eprintln!("spiral notebooks: every notebook reported passing, but the batch exited {}", v1);
                ()
            };
            v1
        }
        _ => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = failure_list_4(v0.clone(), v3.clone());
            let mut v5: i32 = 0i32;
            let mut v6: i32 = failure_count_5(v0.clone(), v5);
            eprintln!("spiral notebooks failed ({}): {}", v6, &*v4);
            let mut v7: i32 = 1i32;
            first_code_3(v0.clone(), v7)
        }
    }
}
fn selected_notebooks_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH1>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match &*v3 {
            UH1::UH1_NoMoreNotebooks => {
                return v4.clone();
            }
            UH1::UH1_SpiralTestNotebook(v5, v6, v7) => {
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let mut v8: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v14: bool = if v8 {
                    true
                } else {
                    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v5));
                    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v2));
                    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
                    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v2));
                    let mut v13: bool = v12.contains(&*v10);
                    v13
                };
                let mut v16: bool = if v14 {
                    let mut v15: bool = std::path::Path::new(&*v0).join(&*v6).is_file();
                    v15
                } else {
                    false
                };
                let mut v22: Rc<str> = if v16 {
                    let mut v17: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v17 {
                        v6.clone()
                    } else {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v18));
                        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v6));
                        v20.clone()
                    }
                } else {
                    v4.clone()
                };
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v7.clone(), v22.clone());
                continue;
            }
        }
    }
}
fn run_batch_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> (i32, std::rc::Rc<str>) {
    let mut v4: (i32, std::rc::Rc<str>) = { let root=std::path::Path::new(&*v0); let target=std::path::Path::new(&*v1); let cells_root=std::env::var_os("EOIE_SPIRAL_CELLS").map(std::path::PathBuf::from).or_else(|| std::env::var_os("EOIE_SPIRAL_BUNDLE").map(|bundle| std::path::PathBuf::from(bundle).join("..").join("..").join("cells"))).unwrap_or_else(|| root.join("..").join("spiral").join("apps").join("cells")); let relatives=(&*v2).lines().map(str::to_owned).collect::<Vec<_>>(); let reports_dir=target.join("spiral-tests"); let _=std::fs::create_dir_all(&reports_dir); let eoie=target.join("debug").join(format!("eoie{}",std::env::consts::EXE_SUFFIX)); let port=20000+std::process::id()%10000; let mut command=std::process::Command::new("pwsh"); command.current_dir(root).arg("-NoProfile").arg("-File").arg(cells_root.join("spi").join("run_notebook.ps1")); for relative in &relatives { command.arg("--path").arg(root.join(relative)); } command.arg("--output-dir").arg(&reports_dir).arg("--no-html").arg("--gate").env("EOIE_BIN_UNDER_TEST",&eoie).env("SPIRAL_CELLS_WARM","0").env("SPIRAL_CELLS_MEMO","0").env_remove("SPIRAL_CELLS_COMPILER").env("SPIRAL_CELLS_COMPILER_PORT",port.to_string()); let bounded=(v3).saturating_mul(relatives.len().max(1) as u64); let stdout_handle=std::io::stdout(); let stderr_handle=std::io::stderr(); let streamed=eoie_process_streaming::run_bounded_streaming(&mut command,bounded,None,|kind,bytes| match kind { eoie_process_streaming::ProcessStreamKind::Stdout => { let mut out=stdout_handle.lock(); std::io::Write::write_all(&mut out,bytes).and_then(|_| std::io::Write::flush(&mut out)).map_err(|error| error.to_string()) } eoie_process_streaming::ProcessStreamKind::Stderr => { let mut err=stderr_handle.lock(); std::io::Write::write_all(&mut err,bytes).and_then(|_| std::io::Write::flush(&mut err)).map_err(|error| error.to_string()) } }); match streamed { Ok(capture) => { let code=if matches!(capture.termination,eoie_process_streaming::ProcessTermination::TimedOut) { 124 } else { capture.status.code().unwrap_or(1) }; let quote=char::from(34u8); let failed_key=format!("{quote}failed{quote}:"); let file_key=format!("{quote}file{quote}:{quote}"); let text=String::from_utf8_lossy(&capture.stdout).replace(char::from(92u8),"/").to_lowercase(); let reports=text.lines().filter_map(|line| line.strip_prefix("spiral-tests file ")).map(|json| { let failed=json.split(failed_key.as_str()).nth(1).map(|rest| rest.chars().take_while(|c| c.is_ascii_digit()).collect::<String>()).unwrap_or_default(); let file=json.split(file_key.as_str()).nth(1).and_then(|rest| rest.split(quote).next()).unwrap_or("").to_owned(); (file,failed) }).collect::<Vec<_>>(); let failing=relatives.iter().filter(|relative| { let wanted=relative.replace(char::from(92u8),"/").to_lowercase(); match reports.iter().find(|(file,_)| file.ends_with(&wanted)) { Some((_,failed)) => failed.as_str() != "0", None => true } }).cloned().collect::<Vec<_>>(); (code,std::rc::Rc::<str>::from(failing.join("
"))) } Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); (2,std::rc::Rc::<str>::from(relatives.join("
"))) } } };
    v4.clone()
}
fn failures_of_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v6: i32 = (v5.clone().borrow().len() as i32);
        let mut v7: bool = v6 > 0i32;
        let mut v11: bool = if v7 {
            let mut v8: i32 = v6.wrapping_sub(1i32);
            let mut v9: Rc<str> = v5.clone().borrow()[v8 as usize].clone();
            let mut v10: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10
        } else {
            false
        };
        let mut v13: i32 = if v11 {
            let mut v12: i32 = v6.wrapping_sub(1i32);
            v12
        } else {
            v6
        };
        let mut v14: bool = v1 < v13;
        if v14 {
            let mut v15: i32 = v1.wrapping_add(1i32);
            let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let mut v17: bool = v1 >= 0i32;
            let mut v27: bool = if v17 {
                let mut v18: i32 = (v16.clone().borrow().len() as i32);
                let mut v19: bool = v18 > 0i32;
                let mut v23: bool = if v19 {
                    let mut v20: i32 = v18.wrapping_sub(1i32);
                    let mut v21: Rc<str> = v16.clone().borrow()[v20 as usize].clone();
                    let mut v22: bool = v21.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v22
                } else {
                    false
                };
                let mut v25: i32 = if v23 {
                    let mut v24: i32 = v18.wrapping_sub(1i32);
                    v24
                } else {
                    v18
                };
                let mut v26: bool = v1 < v25;
                v26
            } else {
                false
            };
            let mut v40: Rc<str> = if v27 {
                let mut v28: Rc<str> = v16.clone().borrow()[v1 as usize].clone();
                let mut v29: i32 = (v28.clone().len() as i32);
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let mut v31: bool = v28.ends_with(&*v30);
                let mut v32: bool = v31 == false;
                if v32 {
                    v28.clone()
                } else {
                    let mut v33: bool = v29 == 1i32;
                    if v33 {
                        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v34.clone()
                    } else {
                        let mut v35: i32 = v29.wrapping_sub(2i32);
                        let mut v36: Rc<str> = string_slice(&v28.clone(), 0i32 as i64, v35 as i64);
                        v36.clone()
                    }
                }
            } else {
                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v39.clone()
            };
            let mut v41: Rc<UH0> = Rc::new(UH0::UH0_FailedThen(v40.clone(), v2, v3.clone()));
            (v0, v1, v2, v3) = (v0.clone(), v15, v2, v41.clone());
            continue;
        } else {
            return v3.clone();
        }
    }
}
fn run_registered_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64) -> i32 {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-planning-rollup"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_planning_rollup/planning_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-scan"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/todo_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-archive-tar-runtime"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_archive_tar_runtime/format.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-bundle-retention"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_bundle_retention/series.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_bundle_retention/receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-job-host"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_job_host/host_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-bundle-manifest"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_bundle_manifest/bundle_manifest.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-verbs"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_verbs/verbs.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-machine-crowding"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_machine_crowding/crowding.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-repo-affected"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_repo_affected/graph.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-log-sequences-model"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_log_sequences/sequence_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-log-sequences"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_log_sequences/sequences.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-process-ownership"); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_process_table/ownership.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-table-sequences-cli"); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_sequences/sequences_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-mirror-model"); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_mirror/mirror.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-mirror-cli"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_mirror/mirror_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-env-pins-model"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_env_pins/pins.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-env-pins-cli"); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_env_pins/env_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-repo-accept-review"); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_repo_accept_review/review.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("command-spec-domain"); } LIT.with(|lit| lit.clone()) };
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/command_spec_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("canonical-plan-ir-domain"); } LIT.with(|lit| lit.clone()) };
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/canonical_plan_ir_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-views"); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_views/views_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-edit"); } LIT.with(|lit| lit.clone()) };
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_edit/edit_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-dev-spiral-tests"); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_dev_spiral_tests/failures.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-handoff-render"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_handoff_render/render.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority-state-domain"); } LIT.with(|lit| lit.clone()) };
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/authority_state_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-note-model"); } LIT.with(|lit| lit.clone()) };
    let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_note_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-machine-watch"); } LIT.with(|lit| lit.clone()) };
    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_machine_watch/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-window-claim"); } LIT.with(|lit| lit.clone()) };
    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_tree_lease/lease.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-adoption"); } LIT.with(|lit| lit.clone()) };
    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_adoption/classify.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-process-table"); } LIT.with(|lit| lit.clone()) };
    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_process_table/ps_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-cli"); } LIT.with(|lit| lit.clone()) };
    let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_cli_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_cli_model/argument_file.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-jobs"); } LIT.with(|lit| lit.clone()) };
    let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_jobs/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-repo-state"); } LIT.with(|lit| lit.clone()) };
    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_repo_state/state.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-table-query"); } LIT.with(|lit| lit.clone()) };
    let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_query/query.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-lcov"); } LIT.with(|lit| lit.clone()) };
    let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_lcov/lcov.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_bundle_retention/receipt_read.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-read-query"); } LIT.with(|lit| lit.clone()) };
    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_read_query/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_receipt/receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-program"); } LIT.with(|lit| lit.clone()) };
    let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_program/program.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-read"); } LIT.with(|lit| lit.clone()) };
    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_read/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_process_table/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-facts"); } LIT.with(|lit| lit.clone()) };
    let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_facts/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-command-line"); } LIT.with(|lit| lit.clone()) };
    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_command_line/launch.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-process"); } LIT.with(|lit| lit.clone()) };
    let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_process/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-tables"); } LIT.with(|lit| lit.clone()) };
    let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_tables/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-write"); } LIT.with(|lit| lit.clone()) };
    let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_write/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-git"); } LIT.with(|lit| lit.clone()) };
    let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_git/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-jobs"); } LIT.with(|lit| lit.clone()) };
    let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_jobs/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-lab"); } LIT.with(|lit| lit.clone()) };
    let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_lab/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/workspace_primitives/toolchain.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-contracts"); } LIT.with(|lit| lit.clone()) };
    let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_notes/notes_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_contracts/receipts_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-agile-contracts"); } LIT.with(|lit| lit.clone()) };
    let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_agile_contracts/agile_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-archive-contracts"); } LIT.with(|lit| lit.clone()) };
    let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_archive_contracts/archive_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-release-contracts"); } LIT.with(|lit| lit.clone()) };
    let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_release_contracts/upgrade_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_cli/windows_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-coverage-prune"); } LIT.with(|lit| lit.clone()) };
    let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/coverage_prune_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_coverage_prune/coverage_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_coverage_prune/evidence_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_release_contracts/hygiene_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-proxy-search"); } LIT.with(|lit| lit.clone()) };
    let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_proxy_search/hash_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_jobs/stop_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-query"); } LIT.with(|lit| lit.clone()) };
    let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_query/query.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-next"); } LIT.with(|lit| lit.clone()) };
    let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_next/ranking.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_next/next_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/roots.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-roots"); } LIT.with(|lit| lit.clone()) };
    let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_roots/roots_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_query/stats.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_query/table_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-record"); } LIT.with(|lit| lit.clone()) };
    let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_record/ledger.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_record/record_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-standards"); } LIT.with(|lit| lit.clone()) };
    let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_standards/rules.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-style-census"); } LIT.with(|lit| lit.clone()) };
    let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_style_census/probes.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_style_census/census_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-window-run"); } LIT.with(|lit| lit.clone()) };
    let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_window_run/run_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-table-test"); } LIT.with(|lit| lit.clone()) };
    let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_test/table_test_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-resume"); } LIT.with(|lit| lit.clone()) };
    let mut v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_resume/resume_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-memory"); } LIT.with(|lit| lit.clone()) };
    let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_memory/memory_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-guide"); } LIT.with(|lit| lit.clone()) };
    let mut v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_guide/guide_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-table-logs"); } LIT.with(|lit| lit.clone()) };
    let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_logs/logs_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_logs/logs_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-experiments"); } LIT.with(|lit| lit.clone()) };
    let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_experiment_stats/experiment_stats.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_experiments/experiment_plan.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_experiments/experiment_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_experiments/experiments_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-machine-windows"); } LIT.with(|lit| lit.clone()) };
    let mut v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_machine_windows/window_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_machine_windows/windows_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_standards/standards_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-memory-links"); } LIT.with(|lit| lit.clone()) };
    let mut v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_memory_links/links_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_memory_links/links_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_read_query/grep_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-crate-gate"); } LIT.with(|lit| lit.clone()) };
    let mut v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_crate_gate/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-table-text"); } LIT.with(|lit| lit.clone()) };
    let mut v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_text/text.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-ci-parity"); } LIT.with(|lit| lit.clone()) };
    let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_ci_parity/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-job-rechain"); } LIT.with(|lit| lit.clone()) };
    let mut v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_job_rechain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-jobs-doctor"); } LIT.with(|lit| lit.clone()) };
    let mut v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_jobs_doctor/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-jobs-doctor-check"); } LIT.with(|lit| lit.clone()) };
    let mut v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_jobs_doctor/check.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-agile-records"); } LIT.with(|lit| lit.clone()) };
    let mut v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_agile_records/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v181: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoMoreNotebooks); } CASE.with(|case| case.clone()) };
    let mut v182: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v179.clone(), v180.clone(), v181.clone()));
    let mut v183: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v177.clone(), v178.clone(), v182.clone()));
    let mut v184: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v175.clone(), v176.clone(), v183.clone()));
    let mut v185: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v173.clone(), v174.clone(), v184.clone()));
    let mut v186: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v171.clone(), v172.clone(), v185.clone()));
    let mut v187: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v169.clone(), v170.clone(), v186.clone()));
    let mut v188: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v167.clone(), v168.clone(), v187.clone()));
    let mut v189: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v80.clone(), v166.clone(), v188.clone()));
    let mut v190: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v163.clone(), v165.clone(), v189.clone()));
    let mut v191: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v163.clone(), v164.clone(), v190.clone()));
    let mut v192: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v136.clone(), v162.clone(), v191.clone()));
    let mut v193: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v159.clone(), v161.clone(), v192.clone()));
    let mut v194: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v159.clone(), v160.clone(), v193.clone()));
    let mut v195: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v154.clone(), v158.clone(), v194.clone()));
    let mut v196: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v154.clone(), v157.clone(), v195.clone()));
    let mut v197: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v154.clone(), v156.clone(), v196.clone()));
    let mut v198: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v154.clone(), v155.clone(), v197.clone()));
    let mut v199: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v151.clone(), v153.clone(), v198.clone()));
    let mut v200: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v151.clone(), v152.clone(), v199.clone()));
    let mut v201: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v149.clone(), v150.clone(), v200.clone()));
    let mut v202: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v147.clone(), v148.clone(), v201.clone()));
    let mut v203: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v145.clone(), v146.clone(), v202.clone()));
    let mut v204: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v143.clone(), v144.clone(), v203.clone()));
    let mut v205: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v141.clone(), v142.clone(), v204.clone()));
    let mut v206: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v138.clone(), v140.clone(), v205.clone()));
    let mut v207: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v138.clone(), v139.clone(), v206.clone()));
    let mut v208: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v136.clone(), v137.clone(), v207.clone()));
    let mut v209: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v133.clone(), v135.clone(), v208.clone()));
    let mut v210: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v133.clone(), v134.clone(), v209.clone()));
    let mut v211: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v75.clone(), v132.clone(), v210.clone()));
    let mut v212: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v75.clone(), v131.clone(), v211.clone()));
    let mut v213: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v129.clone(), v130.clone(), v212.clone()));
    let mut v214: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v8.clone(), v128.clone(), v213.clone()));
    let mut v215: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v125.clone(), v127.clone(), v214.clone()));
    let mut v216: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v125.clone(), v126.clone(), v215.clone()));
    let mut v217: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v123.clone(), v124.clone(), v216.clone()));
    let mut v218: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v71.clone(), v122.clone(), v217.clone()));
    let mut v219: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v120.clone(), v121.clone(), v218.clone()));
    let mut v220: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v112.clone(), v119.clone(), v219.clone()));
    let mut v221: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v115.clone(), v118.clone(), v220.clone()));
    let mut v222: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v115.clone(), v117.clone(), v221.clone()));
    let mut v223: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v115.clone(), v116.clone(), v222.clone()));
    let mut v224: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v68.clone(), v114.clone(), v223.clone()));
    let mut v225: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v112.clone(), v113.clone(), v224.clone()));
    let mut v226: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v110.clone(), v111.clone(), v225.clone()));
    let mut v227: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v108.clone(), v109.clone(), v226.clone()));
    let mut v228: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v105.clone(), v107.clone(), v227.clone()));
    let mut v229: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v105.clone(), v106.clone(), v228.clone()));
    let mut v230: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v68.clone(), v104.clone(), v229.clone()));
    let mut v231: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v102.clone(), v103.clone(), v230.clone()));
    let mut v232: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v100.clone(), v101.clone(), v231.clone()));
    let mut v233: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v98.clone(), v99.clone(), v232.clone()));
    let mut v234: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v96.clone(), v97.clone(), v233.clone()));
    let mut v235: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v94.clone(), v95.clone(), v234.clone()));
    let mut v236: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v92.clone(), v93.clone(), v235.clone()));
    let mut v237: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v90.clone(), v91.clone(), v236.clone()));
    let mut v238: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v88.clone(), v89.clone(), v237.clone()));
    let mut v239: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v66.clone(), v87.clone(), v238.clone()));
    let mut v240: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v85.clone(), v86.clone(), v239.clone()));
    let mut v241: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v83.clone(), v84.clone(), v240.clone()));
    let mut v242: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v80.clone(), v82.clone(), v241.clone()));
    let mut v243: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v80.clone(), v81.clone(), v242.clone()));
    let mut v244: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v13.clone(), v79.clone(), v243.clone()));
    let mut v245: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v77.clone(), v78.clone(), v244.clone()));
    let mut v246: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v75.clone(), v76.clone(), v245.clone()));
    let mut v247: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v73.clone(), v74.clone(), v246.clone()));
    let mut v248: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v71.clone(), v72.clone(), v247.clone()));
    let mut v249: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v68.clone(), v70.clone(), v248.clone()));
    let mut v250: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v68.clone(), v69.clone(), v249.clone()));
    let mut v251: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v66.clone(), v67.clone(), v250.clone()));
    let mut v252: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v64.clone(), v65.clone(), v251.clone()));
    let mut v253: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v62.clone(), v63.clone(), v252.clone()));
    let mut v254: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v60.clone(), v61.clone(), v253.clone()));
    let mut v255: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v58.clone(), v59.clone(), v254.clone()));
    let mut v256: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v56.clone(), v57.clone(), v255.clone()));
    let mut v257: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v54.clone(), v55.clone(), v256.clone()));
    let mut v258: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v52.clone(), v53.clone(), v257.clone()));
    let mut v259: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v50.clone(), v51.clone(), v258.clone()));
    let mut v260: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v48.clone(), v49.clone(), v259.clone()));
    let mut v261: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v46.clone(), v47.clone(), v260.clone()));
    let mut v262: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v44.clone(), v45.clone(), v261.clone()));
    let mut v263: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v42.clone(), v43.clone(), v262.clone()));
    let mut v264: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v40.clone(), v41.clone(), v263.clone()));
    let mut v265: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v38.clone(), v39.clone(), v264.clone()));
    let mut v266: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v36.clone(), v37.clone(), v265.clone()));
    let mut v267: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v34.clone(), v35.clone(), v266.clone()));
    let mut v268: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v32.clone(), v33.clone(), v267.clone()));
    let mut v269: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v30.clone(), v31.clone(), v268.clone()));
    let mut v270: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v28.clone(), v29.clone(), v269.clone()));
    let mut v271: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v26.clone(), v27.clone(), v270.clone()));
    let mut v272: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v24.clone(), v25.clone(), v271.clone()));
    let mut v273: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v22.clone(), v23.clone(), v272.clone()));
    let mut v274: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v20.clone(), v21.clone(), v273.clone()));
    let mut v275: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v18.clone(), v19.clone(), v274.clone()));
    let mut v276: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v16.clone(), v17.clone(), v275.clone()));
    let mut v277: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v13.clone(), v15.clone(), v276.clone()));
    let mut v278: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v13.clone(), v14.clone(), v277.clone()));
    let mut v279: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v11.clone(), v12.clone(), v278.clone()));
    let mut v280: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v8.clone(), v10.clone(), v279.clone()));
    let mut v281: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v8.clone(), v9.clone(), v280.clone()));
    let mut v282: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v6.clone(), v7.clone(), v281.clone()));
    let mut v283: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v284: Rc<str> = selected_notebooks_6(v0.clone(), v2.clone(), v3.clone(), v282.clone(), v283.clone());
    if v4 {
        let mut v285: i32 = 0i32;
        announced_1(v284.clone(), v285)
    } else {
        let mut v287: bool = v284.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v287 {
            0i32
        } else {
            let mut v288: (i32, std::rc::Rc<str>) = run_batch_7(v0.clone(), v1.clone(), v284.clone(), v5);
            let mut v289: i32 = v288.clone().0;
            let mut v290: Rc<str> = v288.1.clone();
            let mut v291: i32 = 0i32;
            let mut v292: bool = v289 == 0i32;
            let mut v293: i32 = if v292 {
                1i32
            } else {
                v289
            };
            let mut v294: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFailures); } CASE.with(|case| case.clone()) };
            let mut v295: Rc<UH0> = failures_of_8(v290.clone(), v291, v293, v294.clone());
            reported_2(v295.clone(), v289)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64| -> i32 {
        run_registered_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_dev_spiral_tests_run(v0: &str, v1: &str, v2: &str, v3: &str, v4: bool, v5: u64) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, v5)
}
