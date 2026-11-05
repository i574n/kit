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
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, Rc<UH2>),
}
struct Mut0 { l0: i32, l1: Rc<UH1> }
fn announced_1(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v4: i32 = (v3.clone().borrow().len() as i32);
        let v5: bool = v4 > 0i32;
        let mut v9: bool = if v5 {
            let v6: i32 = v4.wrapping_sub(1i32);
            let v7: Rc<str> = v3.borrow()[v6 as usize].clone();
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
                    let v18: Rc<str> = v13.borrow()[v17 as usize].clone();
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
                let v25: Rc<str> = v13.borrow()[v1 as usize].clone();
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
            v1 = v39;
            continue;
        } else {
            return 0i32;
        }
    }
}
fn first_code_4(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn failure_list_5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_FailedThen(v2, v3, v4) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: Rc<UH0> = v4.clone();
                let v5: bool = &*v1 == "";
                let mut v9: Rc<str> = if v5 {
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
fn failure_count_6(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn reported_3(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
            let v4: Rc<str> = failure_list_5(v0.clone(), v3);
            let v5: i32 = 0i32;
            let v6: i32 = failure_count_6(v0.clone(), v5);
            eprintln!("spiral notebooks failed ({}): {}", v6, &*v4);
            let v7: i32 = 1i32;
            first_code_4(v0, v7)
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
            let mut v9: Rc<str> = v5.borrow()[v8 as usize].clone();
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
                    let v21: Rc<str> = v16.borrow()[v20 as usize].clone();
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
                let v28: Rc<str> = v16.borrow()[v1 as usize].clone();
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
            (v1, v3) = (v15, v41.clone());
            continue;
        } else {
            return v3.clone();
        }
    }
}
fn batches_run_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH1>, mut v4: Rc<UH0>, mut v5: i32) -> i32 {
    loop {
        match &*v3 {
            UH1::UH1_Cons(v7, v8) => {
                let v7: Rc<str> = v7.clone();
                let v8: Rc<UH1> = v8.clone();
                let mut v9: (i32, std::rc::Rc<str>) = run_batch_7(v0.clone(), v1.clone(), v7.clone(), v2);
                let v10: i32 = v9.clone().0;
                let v11: Rc<str> = v9.1.clone();
                let v12: i32 = 0i32;
                let v13: bool = v10 == 0i32;
                let v14: i32 = if v13 {
                    1i32
                } else {
                    v10
                };
                let v15: Rc<UH0> = failures_of_8(v11.clone(), v12, v14, v4.clone());
                let v16: bool = v5 == 0i32;
                let v17: i32 = if v16 {
                    v10
                } else {
                    v5
                };
                (v3, v4, v5) = (v8.clone(), v15.clone(), v17);
                continue;
            }
            UH1::UH1_Nil => {
                return reported_3(v4.clone(), v5);
            }
        }
    }
}
fn selected_notebooks_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH2>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match &*v3 {
            UH2::UH2_Cons(v5, v6, v7) => {
                let v5: Rc<str> = v5.clone();
                let v6: Rc<str> = v6.clone();
                let v7: Rc<UH2> = v7.clone();
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
                (v3, v4) = (v7.clone(), v22.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v4.clone();
            }
        }
    }
}
fn method11(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method12(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method12(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method11(v16.clone(), v17.clone())
    }
}
fn chunks_from_13(mut v0: i32, mut v1: Rc<UH1>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH1> = v9.clone();
                let v10: bool = v3 >= v0;
                if v10 {
                    let v11: i32 = 1i32;
                    let v12: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v4.clone()));
                    (v1, v2, v3, v4) = (v9.clone(), v8.clone(), v11, v12.clone());
                    continue;
                } else {
                    let v14: bool = &*v2 == "";
                    let v18: Rc<str> = if v14 {
                        v8.clone()
                    } else {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v15)), v8));
                        v17.clone()
                    };
                    let v19: i32 = v3.wrapping_add(1i32);
                    (v1, v2, v3) = (v9.clone(), v18.clone(), v19);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                let v5: bool = &*v2 == "";
                if v5 {
                    return v4.clone();
                } else {
                    return Rc::new(UH1::UH1_Cons(v2.clone(), v4.clone()));
                }
            }
        }
    }
}
fn method14(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn run_registered_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64) -> i32 {
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-ci-model"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_ci_model/tools.spi"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-ci-probe"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_ci_probe/probe.spi"); } LIT.with(|lit| lit.clone()) };
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
    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_dev_spiral_tests/batches.spi"); } LIT.with(|lit| lit.clone()) };
    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-handoff-render"); } LIT.with(|lit| lit.clone()) };
    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_handoff_render/render.spi"); } LIT.with(|lit| lit.clone()) };
    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority-state-domain"); } LIT.with(|lit| lit.clone()) };
    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/authority_state_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-note-model"); } LIT.with(|lit| lit.clone()) };
    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_note_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-machine-watch"); } LIT.with(|lit| lit.clone()) };
    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_machine_watch_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-window-claim"); } LIT.with(|lit| lit.clone()) };
    let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_tree_lease/lease.spi"); } LIT.with(|lit| lit.clone()) };
    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-adoption"); } LIT.with(|lit| lit.clone()) };
    let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_adoption/classify.spi"); } LIT.with(|lit| lit.clone()) };
    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-process-table"); } LIT.with(|lit| lit.clone()) };
    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_process_table/ps_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-cli"); } LIT.with(|lit| lit.clone()) };
    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_cli_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_cli_model/argument_file.spi"); } LIT.with(|lit| lit.clone()) };
    let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-jobs"); } LIT.with(|lit| lit.clone()) };
    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_jobs/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_os_process/process.spi"); } LIT.with(|lit| lit.clone()) };
    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-process-alive"); } LIT.with(|lit| lit.clone()) };
    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_process_alive/alive.spi"); } LIT.with(|lit| lit.clone()) };
    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-repo-state"); } LIT.with(|lit| lit.clone()) };
    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_repo_state/state.spi"); } LIT.with(|lit| lit.clone()) };
    let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-table-query"); } LIT.with(|lit| lit.clone()) };
    let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_query/query.spi"); } LIT.with(|lit| lit.clone()) };
    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-lcov"); } LIT.with(|lit| lit.clone()) };
    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_lcov/lcov.spi"); } LIT.with(|lit| lit.clone()) };
    let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_bundle_retention/receipt_read.spi"); } LIT.with(|lit| lit.clone()) };
    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-read-query"); } LIT.with(|lit| lit.clone()) };
    let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_read_query/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_receipt/receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-program"); } LIT.with(|lit| lit.clone()) };
    let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_program/program.spi"); } LIT.with(|lit| lit.clone()) };
    let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-read"); } LIT.with(|lit| lit.clone()) };
    let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_read/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_process_table/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-facts"); } LIT.with(|lit| lit.clone()) };
    let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_facts/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-command-line"); } LIT.with(|lit| lit.clone()) };
    let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_command_line/launch.spi"); } LIT.with(|lit| lit.clone()) };
    let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-process"); } LIT.with(|lit| lit.clone()) };
    let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_process/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-tables"); } LIT.with(|lit| lit.clone()) };
    let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_tables/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-write"); } LIT.with(|lit| lit.clone()) };
    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_write/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-grep-query"); } LIT.with(|lit| lit.clone()) };
    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_grep_query/grep.spi"); } LIT.with(|lit| lit.clone()) };
    let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-lint-args"); } LIT.with(|lit| lit.clone()) };
    let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_lint_args/args.spi"); } LIT.with(|lit| lit.clone()) };
    let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-experiments"); } LIT.with(|lit| lit.clone()) };
    let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_experiments/experiments.spi"); } LIT.with(|lit| lit.clone()) };
    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-file-steps"); } LIT.with(|lit| lit.clone()) };
    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_file_steps/steps.spi"); } LIT.with(|lit| lit.clone()) };
    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-claim-lease"); } LIT.with(|lit| lit.clone()) };
    let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_lease/lease.spi"); } LIT.with(|lit| lit.clone()) };
    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-git"); } LIT.with(|lit| lit.clone()) };
    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_git/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-jobs"); } LIT.with(|lit| lit.clone()) };
    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_jobs/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-lab"); } LIT.with(|lit| lit.clone()) };
    let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_lab/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/workspace_primitives/toolchain.spi"); } LIT.with(|lit| lit.clone()) };
    let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-contracts"); } LIT.with(|lit| lit.clone()) };
    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_notes/notes_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_contracts/receipts_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-agile-contracts"); } LIT.with(|lit| lit.clone()) };
    let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_agile_contracts/agile_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-archive-contracts"); } LIT.with(|lit| lit.clone()) };
    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_archive_contracts/archive_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-release-contracts"); } LIT.with(|lit| lit.clone()) };
    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_release_contracts/upgrade_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_cli/windows_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-coverage-prune"); } LIT.with(|lit| lit.clone()) };
    let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/coverage_prune_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_coverage_prune/coverage_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_coverage_prune/evidence_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_release_contracts/hygiene_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-proxy-search"); } LIT.with(|lit| lit.clone()) };
    let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_proxy_search/hash_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_jobs/stop_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-query"); } LIT.with(|lit| lit.clone()) };
    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_query/query.spi"); } LIT.with(|lit| lit.clone()) };
    let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-next"); } LIT.with(|lit| lit.clone()) };
    let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_next/ranking.spi"); } LIT.with(|lit| lit.clone()) };
    let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_next/next_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/roots.spi"); } LIT.with(|lit| lit.clone()) };
    let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-roots"); } LIT.with(|lit| lit.clone()) };
    let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_roots/roots_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_query/stats.spi"); } LIT.with(|lit| lit.clone()) };
    let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_query/table_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-record"); } LIT.with(|lit| lit.clone()) };
    let v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_record/ledger.spi"); } LIT.with(|lit| lit.clone()) };
    let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_record/record_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-standards"); } LIT.with(|lit| lit.clone()) };
    let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_standards/rules.spi"); } LIT.with(|lit| lit.clone()) };
    let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-style-probes"); } LIT.with(|lit| lit.clone()) };
    let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_style_probes/probes.spi"); } LIT.with(|lit| lit.clone()) };
    let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-style-baseline"); } LIT.with(|lit| lit.clone()) };
    let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_style_baseline/baseline.spi"); } LIT.with(|lit| lit.clone()) };
    let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-style-measure"); } LIT.with(|lit| lit.clone()) };
    let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_style_measure/measure.spi"); } LIT.with(|lit| lit.clone()) };
    let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-grep-found"); } LIT.with(|lit| lit.clone()) };
    let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_grep_found/found.spi"); } LIT.with(|lit| lit.clone()) };
    let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-lint-growth"); } LIT.with(|lit| lit.clone()) };
    let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_lint_growth/growth.spi"); } LIT.with(|lit| lit.clone()) };
    let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-lint-cli"); } LIT.with(|lit| lit.clone()) };
    let v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_lint_cli/cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-claim-request"); } LIT.with(|lit| lit.clone()) };
    let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_claim_request/request.spi"); } LIT.with(|lit| lit.clone()) };
    let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-style-census"); } LIT.with(|lit| lit.clone()) };
    let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_style_census/census_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-window-run"); } LIT.with(|lit| lit.clone()) };
    let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_window_run/run_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-table-test"); } LIT.with(|lit| lit.clone()) };
    let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_test/table_test_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-resume"); } LIT.with(|lit| lit.clone()) };
    let v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_resume/resume_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-memory"); } LIT.with(|lit| lit.clone()) };
    let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_memory/memory_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-guide"); } LIT.with(|lit| lit.clone()) };
    let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_guide/guide_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-table-logs"); } LIT.with(|lit| lit.clone()) };
    let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_logs/logs_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v199: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_logs/logs_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_experiment_stats/experiment_stats.spi"); } LIT.with(|lit| lit.clone()) };
    let v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_experiments/experiment_plan.spi"); } LIT.with(|lit| lit.clone()) };
    let v202: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_experiments/experiment_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_experiments/experiments_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-machine-windows"); } LIT.with(|lit| lit.clone()) };
    let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_machine_windows/window_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_machine_windows/windows_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_standards/standards_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-memory-links"); } LIT.with(|lit| lit.clone()) };
    let v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_memory_links/links_model.spi"); } LIT.with(|lit| lit.clone()) };
    let v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_memory_links/links_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_read_query/grep_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-crate-gate"); } LIT.with(|lit| lit.clone()) };
    let v213: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_crate_gate/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-table-text"); } LIT.with(|lit| lit.clone()) };
    let v215: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_table_text/text.spi"); } LIT.with(|lit| lit.clone()) };
    let v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_ci_parity/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_job_after_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-jobs-doctor"); } LIT.with(|lit| lit.clone()) };
    let v219: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_jobs_doctor_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-jobs-doctor-check"); } LIT.with(|lit| lit.clone()) };
    let v221: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_jobs_doctor/check.spi"); } LIT.with(|lit| lit.clone()) };
    let v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-agile-records"); } LIT.with(|lit| lit.clone()) };
    let v223: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_agile_records/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v224: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-lint-macros"); } LIT.with(|lit| lit.clone()) };
    let v225: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_lint_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-lint-flow"); } LIT.with(|lit| lit.clone()) };
    let v227: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_lint_pattern/pattern.spi"); } LIT.with(|lit| lit.clone()) };
    let v228: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_lint_flow/flow.spi"); } LIT.with(|lit| lit.clone()) };
    let v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-lint-flow-constructs"); } LIT.with(|lit| lit.clone()) };
    let v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_lint_flow_constructs/constructs.spi"); } LIT.with(|lit| lit.clone()) };
    let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-lint-flow-idioms"); } LIT.with(|lit| lit.clone()) };
    let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_lint_flow_idioms/idioms.spi"); } LIT.with(|lit| lit.clone()) };
    let v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_git/git_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-machine-sample"); } LIT.with(|lit| lit.clone()) };
    let v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_machine_sample/sample.spi"); } LIT.with(|lit| lit.clone()) };
    let v236: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-do-walk"); } LIT.with(|lit| lit.clone()) };
    let v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/kit_do_walk/walk.spi"); } LIT.with(|lit| lit.clone()) };
    let v238: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v239: Rc<UH2> = Rc::new(UH2::UH2_Cons(v236, v237, v238));
    let v240: Rc<UH2> = Rc::new(UH2::UH2_Cons(v234, v235, v239));
    let v241: Rc<UH2> = Rc::new(UH2::UH2_Cons(v131.clone(), v233, v240));
    let v242: Rc<UH2> = Rc::new(UH2::UH2_Cons(v231, v232, v241));
    let v243: Rc<UH2> = Rc::new(UH2::UH2_Cons(v229, v230, v242));
    let v244: Rc<UH2> = Rc::new(UH2::UH2_Cons(v226.clone(), v228, v243));
    let v245: Rc<UH2> = Rc::new(UH2::UH2_Cons(v226, v227, v244));
    let v246: Rc<UH2> = Rc::new(UH2::UH2_Cons(v224, v225, v245));
    let v247: Rc<UH2> = Rc::new(UH2::UH2_Cons(v222, v223, v246));
    let v248: Rc<UH2> = Rc::new(UH2::UH2_Cons(v220, v221, v247));
    let v249: Rc<UH2> = Rc::new(UH2::UH2_Cons(v218, v219, v248));
    let v250: Rc<UH2> = Rc::new(UH2::UH2_Cons(v14.clone(), v217, v249));
    let v251: Rc<UH2> = Rc::new(UH2::UH2_Cons(v12.clone(), v216, v250));
    let v252: Rc<UH2> = Rc::new(UH2::UH2_Cons(v214, v215, v251));
    let v253: Rc<UH2> = Rc::new(UH2::UH2_Cons(v212, v213, v252));
    let v254: Rc<UH2> = Rc::new(UH2::UH2_Cons(v103.clone(), v211, v253));
    let v255: Rc<UH2> = Rc::new(UH2::UH2_Cons(v208.clone(), v210, v254));
    let v256: Rc<UH2> = Rc::new(UH2::UH2_Cons(v208, v209, v255));
    let v257: Rc<UH2> = Rc::new(UH2::UH2_Cons(v169.clone(), v207, v256));
    let v258: Rc<UH2> = Rc::new(UH2::UH2_Cons(v204.clone(), v206, v257));
    let v259: Rc<UH2> = Rc::new(UH2::UH2_Cons(v204, v205, v258));
    let v260: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125.clone(), v203, v259));
    let v261: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125.clone(), v202, v260));
    let v262: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125.clone(), v201, v261));
    let v263: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125.clone(), v200, v262));
    let v264: Rc<UH2> = Rc::new(UH2::UH2_Cons(v197.clone(), v199, v263));
    let v265: Rc<UH2> = Rc::new(UH2::UH2_Cons(v197, v198, v264));
    let v266: Rc<UH2> = Rc::new(UH2::UH2_Cons(v195, v196, v265));
    let v267: Rc<UH2> = Rc::new(UH2::UH2_Cons(v193, v194, v266));
    let v268: Rc<UH2> = Rc::new(UH2::UH2_Cons(v191, v192, v267));
    let v269: Rc<UH2> = Rc::new(UH2::UH2_Cons(v189, v190, v268));
    let v270: Rc<UH2> = Rc::new(UH2::UH2_Cons(v187, v188, v269));
    let v271: Rc<UH2> = Rc::new(UH2::UH2_Cons(v185, v186, v270));
    let v272: Rc<UH2> = Rc::new(UH2::UH2_Cons(v183, v184, v271));
    let v273: Rc<UH2> = Rc::new(UH2::UH2_Cons(v181, v182, v272));
    let v274: Rc<UH2> = Rc::new(UH2::UH2_Cons(v179, v180, v273));
    let v275: Rc<UH2> = Rc::new(UH2::UH2_Cons(v177, v178, v274));
    let v276: Rc<UH2> = Rc::new(UH2::UH2_Cons(v175, v176, v275));
    let v277: Rc<UH2> = Rc::new(UH2::UH2_Cons(v173, v174, v276));
    let v278: Rc<UH2> = Rc::new(UH2::UH2_Cons(v171, v172, v277));
    let v279: Rc<UH2> = Rc::new(UH2::UH2_Cons(v169, v170, v278));
    let v280: Rc<UH2> = Rc::new(UH2::UH2_Cons(v166.clone(), v168, v279));
    let v281: Rc<UH2> = Rc::new(UH2::UH2_Cons(v166, v167, v280));
    let v282: Rc<UH2> = Rc::new(UH2::UH2_Cons(v98.clone(), v165, v281));
    let v283: Rc<UH2> = Rc::new(UH2::UH2_Cons(v98.clone(), v164, v282));
    let v284: Rc<UH2> = Rc::new(UH2::UH2_Cons(v162, v163, v283));
    let v285: Rc<UH2> = Rc::new(UH2::UH2_Cons(v27.clone(), v161, v284));
    let v286: Rc<UH2> = Rc::new(UH2::UH2_Cons(v158.clone(), v160, v285));
    let v287: Rc<UH2> = Rc::new(UH2::UH2_Cons(v158, v159, v286));
    let v288: Rc<UH2> = Rc::new(UH2::UH2_Cons(v156, v157, v287));
    let v289: Rc<UH2> = Rc::new(UH2::UH2_Cons(v91.clone(), v155, v288));
    let v290: Rc<UH2> = Rc::new(UH2::UH2_Cons(v153, v154, v289));
    let v291: Rc<UH2> = Rc::new(UH2::UH2_Cons(v145.clone(), v152, v290));
    let v292: Rc<UH2> = Rc::new(UH2::UH2_Cons(v148.clone(), v151, v291));
    let v293: Rc<UH2> = Rc::new(UH2::UH2_Cons(v148.clone(), v150, v292));
    let v294: Rc<UH2> = Rc::new(UH2::UH2_Cons(v148, v149, v293));
    let v295: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v147, v294));
    let v296: Rc<UH2> = Rc::new(UH2::UH2_Cons(v145, v146, v295));
    let v297: Rc<UH2> = Rc::new(UH2::UH2_Cons(v143, v144, v296));
    let v298: Rc<UH2> = Rc::new(UH2::UH2_Cons(v141, v142, v297));
    let v299: Rc<UH2> = Rc::new(UH2::UH2_Cons(v138.clone(), v140, v298));
    let v300: Rc<UH2> = Rc::new(UH2::UH2_Cons(v138, v139, v299));
    let v301: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v137, v300));
    let v302: Rc<UH2> = Rc::new(UH2::UH2_Cons(v135, v136, v301));
    let v303: Rc<UH2> = Rc::new(UH2::UH2_Cons(v133, v134, v302));
    let v304: Rc<UH2> = Rc::new(UH2::UH2_Cons(v131, v132, v303));
    let v305: Rc<UH2> = Rc::new(UH2::UH2_Cons(v129, v130, v304));
    let v306: Rc<UH2> = Rc::new(UH2::UH2_Cons(v127, v128, v305));
    let v307: Rc<UH2> = Rc::new(UH2::UH2_Cons(v125, v126, v306));
    let v308: Rc<UH2> = Rc::new(UH2::UH2_Cons(v123, v124, v307));
    let v309: Rc<UH2> = Rc::new(UH2::UH2_Cons(v121, v122, v308));
    let v310: Rc<UH2> = Rc::new(UH2::UH2_Cons(v119, v120, v309));
    let v311: Rc<UH2> = Rc::new(UH2::UH2_Cons(v117, v118, v310));
    let v312: Rc<UH2> = Rc::new(UH2::UH2_Cons(v115, v116, v311));
    let v313: Rc<UH2> = Rc::new(UH2::UH2_Cons(v113, v114, v312));
    let v314: Rc<UH2> = Rc::new(UH2::UH2_Cons(v111, v112, v313));
    let v315: Rc<UH2> = Rc::new(UH2::UH2_Cons(v86.clone(), v110, v314));
    let v316: Rc<UH2> = Rc::new(UH2::UH2_Cons(v108, v109, v315));
    let v317: Rc<UH2> = Rc::new(UH2::UH2_Cons(v106, v107, v316));
    let v318: Rc<UH2> = Rc::new(UH2::UH2_Cons(v103.clone(), v105, v317));
    let v319: Rc<UH2> = Rc::new(UH2::UH2_Cons(v103, v104, v318));
    let v320: Rc<UH2> = Rc::new(UH2::UH2_Cons(v32.clone(), v102, v319));
    let v321: Rc<UH2> = Rc::new(UH2::UH2_Cons(v100, v101, v320));
    let v322: Rc<UH2> = Rc::new(UH2::UH2_Cons(v98, v99, v321));
    let v323: Rc<UH2> = Rc::new(UH2::UH2_Cons(v96, v97, v322));
    let v324: Rc<UH2> = Rc::new(UH2::UH2_Cons(v94, v95, v323));
    let v325: Rc<UH2> = Rc::new(UH2::UH2_Cons(v91.clone(), v93, v324));
    let v326: Rc<UH2> = Rc::new(UH2::UH2_Cons(v91, v92, v325));
    let v327: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v90, v326));
    let v328: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88, v89, v327));
    let v329: Rc<UH2> = Rc::new(UH2::UH2_Cons(v86, v87, v328));
    let v330: Rc<UH2> = Rc::new(UH2::UH2_Cons(v84, v85, v329));
    let v331: Rc<UH2> = Rc::new(UH2::UH2_Cons(v82, v83, v330));
    let v332: Rc<UH2> = Rc::new(UH2::UH2_Cons(v80, v81, v331));
    let v333: Rc<UH2> = Rc::new(UH2::UH2_Cons(v78, v79, v332));
    let v334: Rc<UH2> = Rc::new(UH2::UH2_Cons(v76, v77, v333));
    let v335: Rc<UH2> = Rc::new(UH2::UH2_Cons(v74, v75, v334));
    let v336: Rc<UH2> = Rc::new(UH2::UH2_Cons(v71.clone(), v73, v335));
    let v337: Rc<UH2> = Rc::new(UH2::UH2_Cons(v71, v72, v336));
    let v338: Rc<UH2> = Rc::new(UH2::UH2_Cons(v69, v70, v337));
    let v339: Rc<UH2> = Rc::new(UH2::UH2_Cons(v67, v68, v338));
    let v340: Rc<UH2> = Rc::new(UH2::UH2_Cons(v65, v66, v339));
    let v341: Rc<UH2> = Rc::new(UH2::UH2_Cons(v63, v64, v340));
    let v342: Rc<UH2> = Rc::new(UH2::UH2_Cons(v61, v62, v341));
    let v343: Rc<UH2> = Rc::new(UH2::UH2_Cons(v59, v60, v342));
    let v344: Rc<UH2> = Rc::new(UH2::UH2_Cons(v57, v58, v343));
    let v345: Rc<UH2> = Rc::new(UH2::UH2_Cons(v55, v56, v344));
    let v346: Rc<UH2> = Rc::new(UH2::UH2_Cons(v53, v54, v345));
    let v347: Rc<UH2> = Rc::new(UH2::UH2_Cons(v51, v52, v346));
    let v348: Rc<UH2> = Rc::new(UH2::UH2_Cons(v49, v50, v347));
    let v349: Rc<UH2> = Rc::new(UH2::UH2_Cons(v47, v48, v348));
    let v350: Rc<UH2> = Rc::new(UH2::UH2_Cons(v45, v46, v349));
    let v351: Rc<UH2> = Rc::new(UH2::UH2_Cons(v43, v44, v350));
    let v352: Rc<UH2> = Rc::new(UH2::UH2_Cons(v41, v42, v351));
    let v353: Rc<UH2> = Rc::new(UH2::UH2_Cons(v39, v40, v352));
    let v354: Rc<UH2> = Rc::new(UH2::UH2_Cons(v37, v38, v353));
    let v355: Rc<UH2> = Rc::new(UH2::UH2_Cons(v35, v36, v354));
    let v356: Rc<UH2> = Rc::new(UH2::UH2_Cons(v32.clone(), v34, v355));
    let v357: Rc<UH2> = Rc::new(UH2::UH2_Cons(v32, v33, v356));
    let v358: Rc<UH2> = Rc::new(UH2::UH2_Cons(v30, v31, v357));
    let v359: Rc<UH2> = Rc::new(UH2::UH2_Cons(v27.clone(), v29, v358));
    let v360: Rc<UH2> = Rc::new(UH2::UH2_Cons(v27, v28, v359));
    let v361: Rc<UH2> = Rc::new(UH2::UH2_Cons(v25, v26, v360));
    let v362: Rc<UH2> = Rc::new(UH2::UH2_Cons(v22.clone(), v24, v361));
    let v363: Rc<UH2> = Rc::new(UH2::UH2_Cons(v22, v23, v362));
    let v364: Rc<UH2> = Rc::new(UH2::UH2_Cons(v20, v21, v363));
    let v365: Rc<UH2> = Rc::new(UH2::UH2_Cons(v18, v19, v364));
    let v366: Rc<UH2> = Rc::new(UH2::UH2_Cons(v16, v17, v365));
    let v367: Rc<UH2> = Rc::new(UH2::UH2_Cons(v14, v15, v366));
    let v368: Rc<UH2> = Rc::new(UH2::UH2_Cons(v12, v13, v367));
    let v369: Rc<UH2> = Rc::new(UH2::UH2_Cons(v10, v11, v368));
    let v370: Rc<UH2> = Rc::new(UH2::UH2_Cons(v8, v9, v369));
    let v371: Rc<UH2> = Rc::new(UH2::UH2_Cons(v6, v7, v370));
    let v372: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v373: Rc<str> = selected_notebooks_9(v0.clone(), v2, v3, v371, v372.clone());
    if v4 {
        let v374: i32 = 0i32;
        announced_1(v373.clone(), v374)
    } else {
        let v376: bool = &*v373 == "";
        if v376 {
            0i32
        } else {
            let v377: i32 = 32i32;
            let v378: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v379: Rc<UH1> = split_list_10(v378, v373);
            let v380: i32 = 0i32;
            let v381: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v382: Rc<UH1> = chunks_from_13(v377, v379, v372, v380, v381);
            let v383: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v384: Rc<UH1> = method14(v382, v383);
            let v385: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFailures); } CASE.with(|case| case.clone()) };
            let v386: i32 = 0i32;
            batches_run_2(v0, v1, v5, v384, v385, v386)
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
