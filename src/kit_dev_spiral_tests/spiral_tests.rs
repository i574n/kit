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
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<UH1>),
}
fn announced_1(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v4: i32 = (v3.clone().borrow().len() as i32);
        let v5: bool = v4 > 0i32;
        let v9: bool = if v5 {
            let v6: i32 = v4.wrapping_sub(1i32);
            let v7: Rc<str> = v3.clone().borrow()[v6 as usize].clone();
            let v8: bool = &*v7 == "";
            v8
        } else {
            false
        };
        let v11: i32 = if v9 {
            let v10: i32 = v4.wrapping_sub(1i32);
            v10
        } else {
            v4
        };
        let v12: bool = v1 < v11;
        if v12 {
            let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v14: bool = v1 >= 0i32;
            let v24: bool = if v14 {
                let v15: i32 = (v13.clone().borrow().len() as i32);
                let v16: bool = v15 > 0i32;
                let v20: bool = if v16 {
                    let v17: i32 = v15.wrapping_sub(1i32);
                    let v18: Rc<str> = v13.clone().borrow()[v17 as usize].clone();
                    let v19: bool = &*v18 == "";
                    v19
                } else {
                    false
                };
                let v22: i32 = if v20 {
                    let v21: i32 = v15.wrapping_sub(1i32);
                    v21
                } else {
                    v15
                };
                let v23: bool = v1 < v22;
                v23
            } else {
                false
            };
            let v37: Rc<str> = if v24 {
                let v25: Rc<str> = v13.clone().borrow()[v1 as usize].clone();
                let v26: i32 = (v25.clone().len() as i32);
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v28: bool = v25.ends_with(&*v27);
                let v29: bool = v28 == false;
                if v29 {
                    v25.clone()
                } else {
                    let v30: bool = v26 == 1i32;
                    if v30 {
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v31.clone()
                    } else {
                        let v32: i32 = v26.wrapping_sub(2i32);
                        let v33: Rc<str> = string_slice(&v25.clone(), 0i32 as i64, v32 as i64);
                        v33.clone()
                    }
                }
            } else {
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v36.clone()
            };
            let v38: i32 = { std::io::Write::write_all(&mut std::io::stdout(),format!("spiral test blocks {}
",&*v37).as_bytes()).unwrap(); 0i32 };
            let v39: i32 = v1.wrapping_add(1i32);
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
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: Rc<UH0> = v4.clone();
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
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: Rc<UH0> = v4.clone();
                let v5: bool = &*v1 == "";
                let v9: Rc<str> = if v5 {
                    v2.clone()
                } else {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v6)), v1));
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
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: Rc<UH0> = v4.clone();
                let v5: i32 = v1.wrapping_add(1i32);
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
            let v2: bool = v1 == 0i32;
            if v2 {
                ()
            } else {
                eprintln!("spiral notebooks: every notebook reported passing, but the batch exited {}", v1);
                ()
            };
            v1
        }
        _ => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<str> = failure_list_4(v0.clone(), v3.clone());
            let v5: i32 = 0i32;
            let v6: i32 = failure_count_5(v0.clone(), v5);
            eprintln!("spiral notebooks failed ({}): {}", v6, &*v4);
            let v7: i32 = 1i32;
            first_code_3(v0.clone(), v7)
        }
    }
}
fn selected_notebooks_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH1>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match &*v3 {
            UH1::UH1_Cons(v5, v6, v7) => {
                let v5: Rc<str> = v5.clone();
                let v6: Rc<str> = v6.clone();
                let v7: Rc<UH1> = v7.clone();
                let v8: bool = &*v1 == "";
                let v14: bool = if v8 {
                    true
                } else {
                    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v5)), v2));
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v1)), v2));
                    let v13: bool = v12.contains(&*v10);
                    v13
                };
                let v16: bool = if v14 {
                    let v15: bool = std::path::Path::new(&*v0).join(&*v6).is_file();
                    v15
                } else {
                    false
                };
                let v22: Rc<str> = if v16 {
                    let v17: bool = &*v4 == "";
                    if v17 {
                        v6.clone()
                    } else {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v18)), v6));
                        v20.clone()
                    }
                } else {
                    v4.clone()
                };
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v7.clone(), v22.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v4.clone();
            }
        }
    }
}
fn run_batch_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> (i32, std::rc::Rc<str>) {
    let v4: (i32, std::rc::Rc<str>) = { let root=std::path::Path::new(&*v0); let target=std::path::Path::new(&*v1); let cells_root=std::env::var_os("KIT_SPIRAL_CELLS").map(std::path::PathBuf::from).or_else(|| std::env::var_os("KIT_SPIRAL_BUNDLE").map(|bundle| std::path::PathBuf::from(bundle).join("..").join("..").join("cells"))).unwrap_or_else(|| root.join("..").join("spiral").join("apps").join("cells")); let relatives=(&*v2).lines().map(str::to_owned).collect::<Vec<_>>(); let reports_dir=target.join("spiral-tests"); let _=std::fs::create_dir_all(&reports_dir); let kit=target.join("debug").join(format!("kit{}",std::env::consts::EXE_SUFFIX)); let port=20000+std::process::id()%10000; let mut command=std::process::Command::new("pwsh"); command.current_dir(root).arg("-NoProfile").arg("-File").arg(cells_root.join("spi").join("run_notebook.ps1")); for relative in &relatives { command.arg("--path").arg(root.join(relative)); } command.arg("--output-dir").arg(&reports_dir).arg("--no-html").arg("--gate").env("KIT_BIN_UNDER_TEST",&kit).env("SPIRAL_CELLS_WARM","0").env("SPIRAL_CELLS_MEMO","0").env_remove("SPIRAL_CELLS_COMPILER").env("SPIRAL_CELLS_COMPILER_PORT",port.to_string()); let bounded=(v3).saturating_mul(relatives.len().max(1) as u64); let stdout_handle=std::io::stdout(); let stderr_handle=std::io::stderr(); let streamed=kit_process_streaming::run_bounded_streaming(&mut command,bounded,None,|kind,bytes| match kind { kit_process_streaming::ProcessStreamKind::Stdout => { let mut out=stdout_handle.lock(); std::io::Write::write_all(&mut out,bytes).and_then(|_| std::io::Write::flush(&mut out)).map_err(|error| error.to_string()) } kit_process_streaming::ProcessStreamKind::Stderr => { let mut err=stderr_handle.lock(); std::io::Write::write_all(&mut err,bytes).and_then(|_| std::io::Write::flush(&mut err)).map_err(|error| error.to_string()) } }); match streamed { Ok(capture) => { let code=if matches!(capture.termination,kit_process_streaming::ProcessTermination::TimedOut) { 124 } else { capture.status.code().unwrap_or(1) }; let quote=char::from(34u8); let failed_key=format!("{quote}failed{quote}:"); let file_key=format!("{quote}file{quote}:{quote}"); let text=String::from_utf8_lossy(&capture.stdout).replace(char::from(92u8),"/").to_lowercase(); let reports=text.lines().filter_map(|line| line.strip_prefix("spiral-tests file ")).map(|json| { let failed=json.split(failed_key.as_str()).nth(1).map(|rest| rest.chars().take_while(|c| c.is_ascii_digit()).collect::<String>()).unwrap_or_default(); let file=json.split(file_key.as_str()).nth(1).and_then(|rest| rest.split(quote).next()).unwrap_or("").to_owned(); (file,failed) }).collect::<Vec<_>>(); let failing=relatives.iter().filter(|relative| { let wanted=relative.replace(char::from(92u8),"/").to_lowercase(); match reports.iter().find(|(file,_)| file.ends_with(&wanted)) { Some((_,failed)) => failed.as_str() != "0", None => true } }).cloned().collect::<Vec<_>>(); (code,std::rc::Rc::<str>::from(failing.join("
"))) } Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); (2,std::rc::Rc::<str>::from(relatives.join("
"))) } } };
    v4.clone()
}
fn failures_of_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v6: i32 = (v5.clone().borrow().len() as i32);
        let v7: bool = v6 > 0i32;
        let v11: bool = if v7 {
            let v8: i32 = v6.wrapping_sub(1i32);
            let v9: Rc<str> = v5.clone().borrow()[v8 as usize].clone();
            let v10: bool = &*v9 == "";
            v10
        } else {
            false
        };
        let v13: i32 = if v11 {
            let v12: i32 = v6.wrapping_sub(1i32);
            v12
        } else {
            v6
        };
        let v14: bool = v1 < v13;
        if v14 {
            let v15: i32 = v1.wrapping_add(1i32);
            let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v17: bool = v1 >= 0i32;
            let v27: bool = if v17 {
                let v18: i32 = (v16.clone().borrow().len() as i32);
                let v19: bool = v18 > 0i32;
                let v23: bool = if v19 {
                    let v20: i32 = v18.wrapping_sub(1i32);
                    let v21: Rc<str> = v16.clone().borrow()[v20 as usize].clone();
                    let v22: bool = &*v21 == "";
                    v22
                } else {
                    false
                };
                let v25: i32 = if v23 {
                    let v24: i32 = v18.wrapping_sub(1i32);
                    v24
                } else {
                    v18
                };
                let v26: bool = v1 < v25;
                v26
            } else {
                false
            };
            let v40: Rc<str> = if v27 {
                let v28: Rc<str> = v16.clone().borrow()[v1 as usize].clone();
                let v29: i32 = (v28.clone().len() as i32);
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v31: bool = v28.ends_with(&*v30);
                let v32: bool = v31 == false;
                if v32 {
                    v28.clone()
                } else {
                    let v33: bool = v29 == 1i32;
                    if v33 {
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v34.clone()
                    } else {
                        let v35: i32 = v29.wrapping_sub(2i32);
                        let v36: Rc<str> = string_slice(&v28.clone(), 0i32 as i64, v35 as i64);
                        v36.clone()
                    }
                }
            } else {
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v39.clone()
            };
            let v41: Rc<UH0> = Rc::new(UH0::UH0_FailedThen(v40.clone(), v2, v3.clone()));
            (v0, v1, v2, v3) = (v0.clone(), v15, v2, v41.clone());
            continue;
        } else {
            return v3.clone();
        }
    }
}
fn run_registered_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64) -> i32 {
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-ci-model"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_ci_model/tools.spi"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-ci-probe"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_ci_probe/probe.spi"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-ci-verdict"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_ci_verdict/verdict.spi"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-ci-parity"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_ci_parity/parity.spi"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-job-rechain"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_job_rechain/rechain.spi"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-job-chain"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_job_chain/chain.spi"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-jobs-doctor-env"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_jobs_doctor_env/environment.spi"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-adoption-habits"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_adoption_habits/habits.spi"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-log-sections"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_log_sections/section_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_log_sections/section_scan.spi"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-planning-rollup"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_planning_rollup/planning_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-scan"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/todo_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-archive-tar-runtime"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_archive_tar_runtime/format.spi"); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-bundle-retention"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_bundle_retention/series.spi"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_bundle_retention/receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-job-host"); } LIT.with(|lit| lit.clone()) };
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_job_host/host_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-bundle-manifest"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_bundle_manifest/bundle_manifest.spi"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-verbs"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_verbs/verbs.spi"); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-machine-crowding"); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_machine_crowding/crowding.spi"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-repo-affected"); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_repo_affected/graph.spi"); } LIT.with(|lit| lit.clone()) };
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-log-sequences-model"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_log_sequences/sequence_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-log-sequences"); } LIT.with(|lit| lit.clone()) };
    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_log_sequences/sequences.spi"); } LIT.with(|lit| lit.clone()) };
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-process-ownership"); } LIT.with(|lit| lit.clone()) };
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_process_table/ownership.spi"); } LIT.with(|lit| lit.clone()) };
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-table-sequences-cli"); } LIT.with(|lit| lit.clone()) };
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_sequences/sequences_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-mirror-model"); } LIT.with(|lit| lit.clone()) };
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_mirror/mirror.spi"); } LIT.with(|lit| lit.clone()) };
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-mirror-cli"); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_mirror/mirror_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-env-pins-model"); } LIT.with(|lit| lit.clone()) };
    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_env_pins/pins.spi"); } LIT.with(|lit| lit.clone()) };
    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-env-pins-cli"); } LIT.with(|lit| lit.clone()) };
    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_env_pins/env_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-repo-accept-review"); } LIT.with(|lit| lit.clone()) };
    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_repo_accept_review/review.spi"); } LIT.with(|lit| lit.clone()) };
    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("command-spec-domain"); } LIT.with(|lit| lit.clone()) };
    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/command_spec_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("canonical-plan-ir-domain"); } LIT.with(|lit| lit.clone()) };
    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/canonical_plan_ir_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-views"); } LIT.with(|lit| lit.clone()) };
    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_views/views_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-edit"); } LIT.with(|lit| lit.clone()) };
    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_edit/edit_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-dev-spiral-tests"); } LIT.with(|lit| lit.clone()) };
    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_dev_spiral_tests/failures.spi"); } LIT.with(|lit| lit.clone()) };
    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-handoff-render"); } LIT.with(|lit| lit.clone()) };
    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_handoff_render/render.spi"); } LIT.with(|lit| lit.clone()) };
    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority-state-domain"); } LIT.with(|lit| lit.clone()) };
    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/authority_state_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-note-model"); } LIT.with(|lit| lit.clone()) };
    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_note_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-machine-watch"); } LIT.with(|lit| lit.clone()) };
    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_machine_watch_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-window-claim"); } LIT.with(|lit| lit.clone()) };
    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_tree_lease/lease.spi"); } LIT.with(|lit| lit.clone()) };
    let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-adoption"); } LIT.with(|lit| lit.clone()) };
    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_adoption/classify.spi"); } LIT.with(|lit| lit.clone()) };
    let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-process-table"); } LIT.with(|lit| lit.clone()) };
    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_process_table/ps_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-cli"); } LIT.with(|lit| lit.clone()) };
    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_cli_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_cli_model/argument_file.spi"); } LIT.with(|lit| lit.clone()) };
    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-jobs"); } LIT.with(|lit| lit.clone()) };
    let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_jobs/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-repo-state"); } LIT.with(|lit| lit.clone()) };
    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_repo_state/state.spi"); } LIT.with(|lit| lit.clone()) };
    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-table-query"); } LIT.with(|lit| lit.clone()) };
    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_query/query.spi"); } LIT.with(|lit| lit.clone()) };
    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-lcov"); } LIT.with(|lit| lit.clone()) };
    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_lcov/lcov.spi"); } LIT.with(|lit| lit.clone()) };
    let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_bundle_retention/receipt_read.spi"); } LIT.with(|lit| lit.clone()) };
    let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-read-query"); } LIT.with(|lit| lit.clone()) };
    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_read_query/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_receipt/receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-program"); } LIT.with(|lit| lit.clone()) };
    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_program/program.spi"); } LIT.with(|lit| lit.clone()) };
    let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-read"); } LIT.with(|lit| lit.clone()) };
    let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_read/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_process_table/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-facts"); } LIT.with(|lit| lit.clone()) };
    let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_facts/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-command-line"); } LIT.with(|lit| lit.clone()) };
    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_command_line/launch.spi"); } LIT.with(|lit| lit.clone()) };
    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-process"); } LIT.with(|lit| lit.clone()) };
    let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_process/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-tables"); } LIT.with(|lit| lit.clone()) };
    let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_tables/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-write"); } LIT.with(|lit| lit.clone()) };
    let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_write/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-git"); } LIT.with(|lit| lit.clone()) };
    let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_git/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-jobs"); } LIT.with(|lit| lit.clone()) };
    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_jobs/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-lab"); } LIT.with(|lit| lit.clone()) };
    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_lab/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/workspace_primitives/toolchain.spi"); } LIT.with(|lit| lit.clone()) };
    let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-contracts"); } LIT.with(|lit| lit.clone()) };
    let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_notes/notes_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_contracts/receipts_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-agile-contracts"); } LIT.with(|lit| lit.clone()) };
    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_agile_contracts/agile_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-archive-contracts"); } LIT.with(|lit| lit.clone()) };
    let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_archive_contracts/archive_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-release-contracts"); } LIT.with(|lit| lit.clone()) };
    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_release_contracts/upgrade_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_cli/windows_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-coverage-prune"); } LIT.with(|lit| lit.clone()) };
    let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/coverage_prune_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_coverage_prune/coverage_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_coverage_prune/evidence_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_release_contracts/hygiene_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-proxy-search"); } LIT.with(|lit| lit.clone()) };
    let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_proxy_search/hash_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_jobs/stop_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-query"); } LIT.with(|lit| lit.clone()) };
    let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_query/query.spi"); } LIT.with(|lit| lit.clone()) };
    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-next"); } LIT.with(|lit| lit.clone()) };
    let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_next/ranking.spi"); } LIT.with(|lit| lit.clone()) };
    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_next/next_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/roots.spi"); } LIT.with(|lit| lit.clone()) };
    let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-roots"); } LIT.with(|lit| lit.clone()) };
    let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_roots/roots_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_query/stats.spi"); } LIT.with(|lit| lit.clone()) };
    let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_query/table_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-record"); } LIT.with(|lit| lit.clone()) };
    let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_record/ledger.spi"); } LIT.with(|lit| lit.clone()) };
    let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_record/record_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-standards"); } LIT.with(|lit| lit.clone()) };
    let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_standards/rules.spi"); } LIT.with(|lit| lit.clone()) };
    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-style-census"); } LIT.with(|lit| lit.clone()) };
    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_style_census/probes.spi"); } LIT.with(|lit| lit.clone()) };
    let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_style_census/census_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-window-run"); } LIT.with(|lit| lit.clone()) };
    let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_window_run/run_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-table-test"); } LIT.with(|lit| lit.clone()) };
    let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_test/table_test_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-resume"); } LIT.with(|lit| lit.clone()) };
    let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_resume/resume_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-memory"); } LIT.with(|lit| lit.clone()) };
    let v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_memory/memory_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-guide"); } LIT.with(|lit| lit.clone()) };
    let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_guide/guide_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-table-logs"); } LIT.with(|lit| lit.clone()) };
    let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_logs/logs_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_logs/logs_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-experiments"); } LIT.with(|lit| lit.clone()) };
    let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_experiment_stats/experiment_stats.spi"); } LIT.with(|lit| lit.clone()) };
    let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_experiments/experiment_plan.spi"); } LIT.with(|lit| lit.clone()) };
    let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_experiments/experiment_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_experiments/experiments_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-machine-windows"); } LIT.with(|lit| lit.clone()) };
    let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_machine_windows/window_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_machine_windows/windows_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_standards/standards_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-memory-links"); } LIT.with(|lit| lit.clone()) };
    let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_memory_links/links_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_memory_links/links_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_read_query/grep_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-crate-gate"); } LIT.with(|lit| lit.clone()) };
    let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_crate_gate/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-table-text"); } LIT.with(|lit| lit.clone()) };
    let v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_text/text.spi"); } LIT.with(|lit| lit.clone()) };
    let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_ci_parity/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_job_after_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-jobs-doctor"); } LIT.with(|lit| lit.clone()) };
    let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_jobs_doctor_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-jobs-doctor-check"); } LIT.with(|lit| lit.clone()) };
    let v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_jobs_doctor/check.spi"); } LIT.with(|lit| lit.clone()) };
    let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-agile-records"); } LIT.with(|lit| lit.clone()) };
    let v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_agile_records/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v198: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v199: Rc<UH1> = Rc::new(UH1::UH1_Cons(v196.clone(), v197.clone(), v198.clone()));
    let v200: Rc<UH1> = Rc::new(UH1::UH1_Cons(v194.clone(), v195.clone(), v199.clone()));
    let v201: Rc<UH1> = Rc::new(UH1::UH1_Cons(v192.clone(), v193.clone(), v200.clone()));
    let v202: Rc<UH1> = Rc::new(UH1::UH1_Cons(v14.clone(), v191.clone(), v201.clone()));
    let v203: Rc<UH1> = Rc::new(UH1::UH1_Cons(v12.clone(), v190.clone(), v202.clone()));
    let v204: Rc<UH1> = Rc::new(UH1::UH1_Cons(v188.clone(), v189.clone(), v203.clone()));
    let v205: Rc<UH1> = Rc::new(UH1::UH1_Cons(v186.clone(), v187.clone(), v204.clone()));
    let v206: Rc<UH1> = Rc::new(UH1::UH1_Cons(v99.clone(), v185.clone(), v205.clone()));
    let v207: Rc<UH1> = Rc::new(UH1::UH1_Cons(v182.clone(), v184.clone(), v206.clone()));
    let v208: Rc<UH1> = Rc::new(UH1::UH1_Cons(v182.clone(), v183.clone(), v207.clone()));
    let v209: Rc<UH1> = Rc::new(UH1::UH1_Cons(v155.clone(), v181.clone(), v208.clone()));
    let v210: Rc<UH1> = Rc::new(UH1::UH1_Cons(v178.clone(), v180.clone(), v209.clone()));
    let v211: Rc<UH1> = Rc::new(UH1::UH1_Cons(v178.clone(), v179.clone(), v210.clone()));
    let v212: Rc<UH1> = Rc::new(UH1::UH1_Cons(v173.clone(), v177.clone(), v211.clone()));
    let v213: Rc<UH1> = Rc::new(UH1::UH1_Cons(v173.clone(), v176.clone(), v212.clone()));
    let v214: Rc<UH1> = Rc::new(UH1::UH1_Cons(v173.clone(), v175.clone(), v213.clone()));
    let v215: Rc<UH1> = Rc::new(UH1::UH1_Cons(v173.clone(), v174.clone(), v214.clone()));
    let v216: Rc<UH1> = Rc::new(UH1::UH1_Cons(v170.clone(), v172.clone(), v215.clone()));
    let v217: Rc<UH1> = Rc::new(UH1::UH1_Cons(v170.clone(), v171.clone(), v216.clone()));
    let v218: Rc<UH1> = Rc::new(UH1::UH1_Cons(v168.clone(), v169.clone(), v217.clone()));
    let v219: Rc<UH1> = Rc::new(UH1::UH1_Cons(v166.clone(), v167.clone(), v218.clone()));
    let v220: Rc<UH1> = Rc::new(UH1::UH1_Cons(v164.clone(), v165.clone(), v219.clone()));
    let v221: Rc<UH1> = Rc::new(UH1::UH1_Cons(v162.clone(), v163.clone(), v220.clone()));
    let v222: Rc<UH1> = Rc::new(UH1::UH1_Cons(v160.clone(), v161.clone(), v221.clone()));
    let v223: Rc<UH1> = Rc::new(UH1::UH1_Cons(v157.clone(), v159.clone(), v222.clone()));
    let v224: Rc<UH1> = Rc::new(UH1::UH1_Cons(v157.clone(), v158.clone(), v223.clone()));
    let v225: Rc<UH1> = Rc::new(UH1::UH1_Cons(v155.clone(), v156.clone(), v224.clone()));
    let v226: Rc<UH1> = Rc::new(UH1::UH1_Cons(v152.clone(), v154.clone(), v225.clone()));
    let v227: Rc<UH1> = Rc::new(UH1::UH1_Cons(v152.clone(), v153.clone(), v226.clone()));
    let v228: Rc<UH1> = Rc::new(UH1::UH1_Cons(v94.clone(), v151.clone(), v227.clone()));
    let v229: Rc<UH1> = Rc::new(UH1::UH1_Cons(v94.clone(), v150.clone(), v228.clone()));
    let v230: Rc<UH1> = Rc::new(UH1::UH1_Cons(v148.clone(), v149.clone(), v229.clone()));
    let v231: Rc<UH1> = Rc::new(UH1::UH1_Cons(v27.clone(), v147.clone(), v230.clone()));
    let v232: Rc<UH1> = Rc::new(UH1::UH1_Cons(v144.clone(), v146.clone(), v231.clone()));
    let v233: Rc<UH1> = Rc::new(UH1::UH1_Cons(v144.clone(), v145.clone(), v232.clone()));
    let v234: Rc<UH1> = Rc::new(UH1::UH1_Cons(v142.clone(), v143.clone(), v233.clone()));
    let v235: Rc<UH1> = Rc::new(UH1::UH1_Cons(v90.clone(), v141.clone(), v234.clone()));
    let v236: Rc<UH1> = Rc::new(UH1::UH1_Cons(v139.clone(), v140.clone(), v235.clone()));
    let v237: Rc<UH1> = Rc::new(UH1::UH1_Cons(v131.clone(), v138.clone(), v236.clone()));
    let v238: Rc<UH1> = Rc::new(UH1::UH1_Cons(v134.clone(), v137.clone(), v237.clone()));
    let v239: Rc<UH1> = Rc::new(UH1::UH1_Cons(v134.clone(), v136.clone(), v238.clone()));
    let v240: Rc<UH1> = Rc::new(UH1::UH1_Cons(v134.clone(), v135.clone(), v239.clone()));
    let v241: Rc<UH1> = Rc::new(UH1::UH1_Cons(v87.clone(), v133.clone(), v240.clone()));
    let v242: Rc<UH1> = Rc::new(UH1::UH1_Cons(v131.clone(), v132.clone(), v241.clone()));
    let v243: Rc<UH1> = Rc::new(UH1::UH1_Cons(v129.clone(), v130.clone(), v242.clone()));
    let v244: Rc<UH1> = Rc::new(UH1::UH1_Cons(v127.clone(), v128.clone(), v243.clone()));
    let v245: Rc<UH1> = Rc::new(UH1::UH1_Cons(v124.clone(), v126.clone(), v244.clone()));
    let v246: Rc<UH1> = Rc::new(UH1::UH1_Cons(v124.clone(), v125.clone(), v245.clone()));
    let v247: Rc<UH1> = Rc::new(UH1::UH1_Cons(v87.clone(), v123.clone(), v246.clone()));
    let v248: Rc<UH1> = Rc::new(UH1::UH1_Cons(v121.clone(), v122.clone(), v247.clone()));
    let v249: Rc<UH1> = Rc::new(UH1::UH1_Cons(v119.clone(), v120.clone(), v248.clone()));
    let v250: Rc<UH1> = Rc::new(UH1::UH1_Cons(v117.clone(), v118.clone(), v249.clone()));
    let v251: Rc<UH1> = Rc::new(UH1::UH1_Cons(v115.clone(), v116.clone(), v250.clone()));
    let v252: Rc<UH1> = Rc::new(UH1::UH1_Cons(v113.clone(), v114.clone(), v251.clone()));
    let v253: Rc<UH1> = Rc::new(UH1::UH1_Cons(v111.clone(), v112.clone(), v252.clone()));
    let v254: Rc<UH1> = Rc::new(UH1::UH1_Cons(v109.clone(), v110.clone(), v253.clone()));
    let v255: Rc<UH1> = Rc::new(UH1::UH1_Cons(v107.clone(), v108.clone(), v254.clone()));
    let v256: Rc<UH1> = Rc::new(UH1::UH1_Cons(v85.clone(), v106.clone(), v255.clone()));
    let v257: Rc<UH1> = Rc::new(UH1::UH1_Cons(v104.clone(), v105.clone(), v256.clone()));
    let v258: Rc<UH1> = Rc::new(UH1::UH1_Cons(v102.clone(), v103.clone(), v257.clone()));
    let v259: Rc<UH1> = Rc::new(UH1::UH1_Cons(v99.clone(), v101.clone(), v258.clone()));
    let v260: Rc<UH1> = Rc::new(UH1::UH1_Cons(v99.clone(), v100.clone(), v259.clone()));
    let v261: Rc<UH1> = Rc::new(UH1::UH1_Cons(v32.clone(), v98.clone(), v260.clone()));
    let v262: Rc<UH1> = Rc::new(UH1::UH1_Cons(v96.clone(), v97.clone(), v261.clone()));
    let v263: Rc<UH1> = Rc::new(UH1::UH1_Cons(v94.clone(), v95.clone(), v262.clone()));
    let v264: Rc<UH1> = Rc::new(UH1::UH1_Cons(v92.clone(), v93.clone(), v263.clone()));
    let v265: Rc<UH1> = Rc::new(UH1::UH1_Cons(v90.clone(), v91.clone(), v264.clone()));
    let v266: Rc<UH1> = Rc::new(UH1::UH1_Cons(v87.clone(), v89.clone(), v265.clone()));
    let v267: Rc<UH1> = Rc::new(UH1::UH1_Cons(v87.clone(), v88.clone(), v266.clone()));
    let v268: Rc<UH1> = Rc::new(UH1::UH1_Cons(v85.clone(), v86.clone(), v267.clone()));
    let v269: Rc<UH1> = Rc::new(UH1::UH1_Cons(v83.clone(), v84.clone(), v268.clone()));
    let v270: Rc<UH1> = Rc::new(UH1::UH1_Cons(v81.clone(), v82.clone(), v269.clone()));
    let v271: Rc<UH1> = Rc::new(UH1::UH1_Cons(v79.clone(), v80.clone(), v270.clone()));
    let v272: Rc<UH1> = Rc::new(UH1::UH1_Cons(v77.clone(), v78.clone(), v271.clone()));
    let v273: Rc<UH1> = Rc::new(UH1::UH1_Cons(v75.clone(), v76.clone(), v272.clone()));
    let v274: Rc<UH1> = Rc::new(UH1::UH1_Cons(v73.clone(), v74.clone(), v273.clone()));
    let v275: Rc<UH1> = Rc::new(UH1::UH1_Cons(v71.clone(), v72.clone(), v274.clone()));
    let v276: Rc<UH1> = Rc::new(UH1::UH1_Cons(v69.clone(), v70.clone(), v275.clone()));
    let v277: Rc<UH1> = Rc::new(UH1::UH1_Cons(v67.clone(), v68.clone(), v276.clone()));
    let v278: Rc<UH1> = Rc::new(UH1::UH1_Cons(v65.clone(), v66.clone(), v277.clone()));
    let v279: Rc<UH1> = Rc::new(UH1::UH1_Cons(v63.clone(), v64.clone(), v278.clone()));
    let v280: Rc<UH1> = Rc::new(UH1::UH1_Cons(v61.clone(), v62.clone(), v279.clone()));
    let v281: Rc<UH1> = Rc::new(UH1::UH1_Cons(v59.clone(), v60.clone(), v280.clone()));
    let v282: Rc<UH1> = Rc::new(UH1::UH1_Cons(v57.clone(), v58.clone(), v281.clone()));
    let v283: Rc<UH1> = Rc::new(UH1::UH1_Cons(v55.clone(), v56.clone(), v282.clone()));
    let v284: Rc<UH1> = Rc::new(UH1::UH1_Cons(v53.clone(), v54.clone(), v283.clone()));
    let v285: Rc<UH1> = Rc::new(UH1::UH1_Cons(v51.clone(), v52.clone(), v284.clone()));
    let v286: Rc<UH1> = Rc::new(UH1::UH1_Cons(v49.clone(), v50.clone(), v285.clone()));
    let v287: Rc<UH1> = Rc::new(UH1::UH1_Cons(v47.clone(), v48.clone(), v286.clone()));
    let v288: Rc<UH1> = Rc::new(UH1::UH1_Cons(v45.clone(), v46.clone(), v287.clone()));
    let v289: Rc<UH1> = Rc::new(UH1::UH1_Cons(v43.clone(), v44.clone(), v288.clone()));
    let v290: Rc<UH1> = Rc::new(UH1::UH1_Cons(v41.clone(), v42.clone(), v289.clone()));
    let v291: Rc<UH1> = Rc::new(UH1::UH1_Cons(v39.clone(), v40.clone(), v290.clone()));
    let v292: Rc<UH1> = Rc::new(UH1::UH1_Cons(v37.clone(), v38.clone(), v291.clone()));
    let v293: Rc<UH1> = Rc::new(UH1::UH1_Cons(v35.clone(), v36.clone(), v292.clone()));
    let v294: Rc<UH1> = Rc::new(UH1::UH1_Cons(v32.clone(), v34.clone(), v293.clone()));
    let v295: Rc<UH1> = Rc::new(UH1::UH1_Cons(v32.clone(), v33.clone(), v294.clone()));
    let v296: Rc<UH1> = Rc::new(UH1::UH1_Cons(v30.clone(), v31.clone(), v295.clone()));
    let v297: Rc<UH1> = Rc::new(UH1::UH1_Cons(v27.clone(), v29.clone(), v296.clone()));
    let v298: Rc<UH1> = Rc::new(UH1::UH1_Cons(v27.clone(), v28.clone(), v297.clone()));
    let v299: Rc<UH1> = Rc::new(UH1::UH1_Cons(v25.clone(), v26.clone(), v298.clone()));
    let v300: Rc<UH1> = Rc::new(UH1::UH1_Cons(v22.clone(), v24.clone(), v299.clone()));
    let v301: Rc<UH1> = Rc::new(UH1::UH1_Cons(v22.clone(), v23.clone(), v300.clone()));
    let v302: Rc<UH1> = Rc::new(UH1::UH1_Cons(v20.clone(), v21.clone(), v301.clone()));
    let v303: Rc<UH1> = Rc::new(UH1::UH1_Cons(v18.clone(), v19.clone(), v302.clone()));
    let v304: Rc<UH1> = Rc::new(UH1::UH1_Cons(v16.clone(), v17.clone(), v303.clone()));
    let v305: Rc<UH1> = Rc::new(UH1::UH1_Cons(v14.clone(), v15.clone(), v304.clone()));
    let v306: Rc<UH1> = Rc::new(UH1::UH1_Cons(v12.clone(), v13.clone(), v305.clone()));
    let v307: Rc<UH1> = Rc::new(UH1::UH1_Cons(v10.clone(), v11.clone(), v306.clone()));
    let v308: Rc<UH1> = Rc::new(UH1::UH1_Cons(v8.clone(), v9.clone(), v307.clone()));
    let v309: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6.clone(), v7.clone(), v308.clone()));
    let v310: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v311: Rc<str> = selected_notebooks_6(v0.clone(), v2.clone(), v3.clone(), v309.clone(), v310.clone());
    if v4 {
        let v312: i32 = 0i32;
        announced_1(v311.clone(), v312)
    } else {
        let v314: bool = &*v311 == "";
        if v314 {
            0i32
        } else {
            let v315: (i32, std::rc::Rc<str>) = run_batch_7(v0.clone(), v1.clone(), v311.clone(), v5);
            let v316: i32 = v315.clone().0;
            let v317: Rc<str> = v315.1.clone();
            let v318: i32 = 0i32;
            let v319: bool = v316 == 0i32;
            let v320: i32 = if v319 {
                1i32
            } else {
                v316
            };
            let v321: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFailures); } CASE.with(|case| case.clone()) };
            let v322: Rc<UH0> = failures_of_8(v317.clone(), v318, v320, v321.clone());
            reported_2(v322.clone(), v316)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64| -> i32 {
        run_registered_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_dev_spiral_tests_run(v0: &str, v1: &str, v2: &str, v3: &str, v4: bool, v5: u64) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, v5)
}
