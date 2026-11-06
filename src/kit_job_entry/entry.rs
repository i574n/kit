#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_JobStart,
    US0_JobRun,
    US0_JobSupervise,
    US0_JobHost,
    US0_JobStatus,
    US0_JobWait,
    US0_JobList,
    US0_JobTail,
    US0_JobStop,
    US0_JobAfter,
    US0_JobChain,
    US0_JobAmend,
    US0_JobDoctor,
    US0_JobCoordinator,
    US0_JobUnknown,
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
pub struct Mut0 { pub l0: i32, pub l1: US2 }
fn concat5_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn stderr_line_5(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn rejected_3(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = concat5_4(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_5(v3);
    2i32
}
fn shared_summary_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = kit_job_summary::kit_job_summary(&*v0, &*v1);
    v2
}
fn stdout_line_8(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn started_code_6(mut v0: bool) -> i32 {
    let v1: Rc<RefCell<Vec<(i32, Rc<str>)>>> = kit_job_launch::kit_job_start(v0);
    let (mut v2, mut v3): (i32, Rc<str>) = v1.borrow()[0i32 as usize].clone();
    let v5: bool = if v0 {
        let v4: bool = &*v3 != "";
        v4
    } else {
        false
    };
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = shared_summary_7(v6, v3);
        stdout_line_8(v7)
    };
    v2
}
fn jobs_root_10() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn dispatched_9() -> i32 {
    let v0: Rc<str> = jobs_root_10();
    let v1: i32 = kit_job_dispatch::kit_job_dispatch_run(&v0);
    v1
}
fn job_ran_2(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    match v0 {
        US0::US0_JobAfter => {
            let v33: i32 = kit_job_rechain::kit_job_rechain_run();
            v33
        }
        US0::US0_JobAmend => {
            let v38: i32 = kit_job_control::kit_job_amend_run();
            v38
        }
        US0::US0_JobChain => {
            let v34: bool = &*v1 == "--run-serial";
            if v34 {
                let v35: i32 = kit_job_serial::kit_job_serial_run();
                v35
            } else {
                let v36: i32 = kit_job_chain::kit_job_chain_run();
                v36
            }
        }
        US0::US0_JobCoordinator => {
            let v40: Rc<str> = jobs_root_10();
            let v41: i32 = kit_job_coordinator::kit_job_coordinator_run(&v40);
            v41
        }
        US0::US0_JobDoctor => {
            let v39: i32 = kit_jobs_doctor::kit_job_doctor_run();
            v39
        }
        US0::US0_JobHost => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v19: bool = v1.starts_with(&*v18);
            if v19 {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host is started by kit itself and takes no flags; see job --help"); } LIT.with(|lit| lit.clone()) };
                rejected_3(v20)
            } else {
                let v22: i32 = kit_job_host::kit_job_host_run();
                v22
            }
        }
        US0::US0_JobList => {
            let v26: bool = &*v1 == "--on";
            if v26 {
                let v27: Rc<str> = jobs_root_10();
                let v28: i32 = kit_job_dispatch::kit_job_list_on(&v27);
                v28
            } else {
                let v29: i32 = kit_job_control::kit_job_list_run();
                v29
            }
        }
        US0::US0_JobRun => {
            let v8: bool = true;
            started_code_6(v8)
        }
        US0::US0_JobStart => {
            let v3: bool = &*v1 == "--on";
            if v3 {
                dispatched_9()
            } else {
                let v5: bool = false;
                started_code_6(v5)
            }
        }
        US0::US0_JobStatus => {
            let v24: i32 = kit_job_query::kit_job_status_placed(&v1);
            v24
        }
        US0::US0_JobStop => {
            let v32: i32 = kit_job_query::kit_job_stop_placed(&v1);
            v32
        }
        US0::US0_JobSupervise => {
            let v10: bool = &*v1 == "";
            let v13: bool = if v10 {
                true
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let v12: bool = v1.starts_with(&*v11);
                v12
            };
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervise is started by kit itself for one job id and takes no flags; see job --help"); } LIT.with(|lit| lit.clone()) };
                rejected_3(v14)
            } else {
                let v16: i32 = kit_job_supervise::kit_job_supervise(&v1);
                v16
            }
        }
        US0::US0_JobTail => {
            let v31: i32 = kit_job_query::kit_job_tail_placed(&v1);
            v31
        }
        US0::US0_JobUnknown => {
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job coordinator build --spiral <dir> --kit <dir> (builds the resident job coordinator through Cells --build-only and records its receipt) | job coordinator start --spool <dir> (starts it as a resident job that takes <name>.sub files from the spool) | job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already settled badly prints 'job start failed' with its state, exit and detail and exits with that code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
            rejected_3(v42)
        }
        US0::US0_JobWait => {
            let v25: i32 = kit_job_query::kit_job_wait_placed(&v1, &v2);
            v25
        }
    }
}
fn job_operands_1(mut v0: US0, mut v1: Rc<UH0>) -> i32 {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    job_ran_2(v0, v2, v4)
                }
                UH0::UH0_Nil => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    job_ran_2(v0, v2, v7)
                }
            }
        }
        UH0::UH0_Nil => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            job_ran_2(v0, v11.clone(), v11)
        }
    }
}
fn command_line_argument_count_12() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_13(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_11(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_12();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US1 = if v4 {
        let v5: Rc<str> = command_line_argument_13(v0);
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
            let v12: Rc<UH0> = arguments_from_11(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn method14(mut v0: &Rc<RefCell<Mut0>>) -> bool {
    let v1: i32 = v0.borrow().l0;
    let v2: bool = v1 > 0i32;
    v2
}
fn job_run_0() -> i32 {
    let v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_11(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) }, v4));
                    ;
                    ;
                    ;
                    ;
                    ;
                    ;
                    ;
                    ;
                    ;
                    ;
                    ;
                    ;
                    ;
                    ;
                    ;
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobUnknown"); } LIT.with(|lit| lit.clone()) };
                    let v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobCoordinator"); } LIT.with(|lit| lit.clone()) };
                    let v10: Rc<str> = Rc::<str>::from(v9.to_lowercase());
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobDoctor"); } LIT.with(|lit| lit.clone()) };
                    let v12: Rc<str> = Rc::<str>::from(v11.to_lowercase());
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobAmend"); } LIT.with(|lit| lit.clone()) };
                    let v14: Rc<str> = Rc::<str>::from(v13.to_lowercase());
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobChain"); } LIT.with(|lit| lit.clone()) };
                    let v16: Rc<str> = Rc::<str>::from(v15.to_lowercase());
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobAfter"); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<str> = Rc::<str>::from(v17.to_lowercase());
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobStop"); } LIT.with(|lit| lit.clone()) };
                    let v20: Rc<str> = Rc::<str>::from(v19.to_lowercase());
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobTail"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(v21.to_lowercase());
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobList"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = Rc::<str>::from(v23.to_lowercase());
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobWait"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = Rc::<str>::from(v25.to_lowercase());
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobStatus"); } LIT.with(|lit| lit.clone()) };
                    let v28: Rc<str> = Rc::<str>::from(v27.to_lowercase());
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobHost"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = Rc::<str>::from(v29.to_lowercase());
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobSupervise"); } LIT.with(|lit| lit.clone()) };
                    let v32: Rc<str> = Rc::<str>::from(v31.to_lowercase());
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobRun"); } LIT.with(|lit| lit.clone()) };
                    let v34: Rc<str> = Rc::<str>::from(v33.to_lowercase());
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobStart"); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = Rc::<str>::from(v35.to_lowercase());
                    let mut v37: Rc<RefCell<Vec<(Rc<str>, US0)>>> = Rc::new(RefCell::new(Vec::new()));
                    let v38: US0 = US0::US0_JobStart;
                    v37.borrow_mut().push((v35, v38));
                    let v39: US0 = US0::US0_JobRun;
                    v37.borrow_mut().push((v33, v39));
                    let v40: US0 = US0::US0_JobSupervise;
                    v37.borrow_mut().push((v31, v40));
                    let v41: US0 = US0::US0_JobHost;
                    v37.borrow_mut().push((v29, v41));
                    let v42: US0 = US0::US0_JobStatus;
                    v37.borrow_mut().push((v27, v42));
                    let v43: US0 = US0::US0_JobWait;
                    v37.borrow_mut().push((v25, v43));
                    let v44: US0 = US0::US0_JobList;
                    v37.borrow_mut().push((v23, v44));
                    let v45: US0 = US0::US0_JobTail;
                    v37.borrow_mut().push((v21, v45));
                    let v46: US0 = US0::US0_JobStop;
                    v37.borrow_mut().push((v19, v46));
                    let v47: US0 = US0::US0_JobAfter;
                    v37.borrow_mut().push((v17, v47));
                    let v48: US0 = US0::US0_JobChain;
                    v37.borrow_mut().push((v15, v48));
                    let v49: US0 = US0::US0_JobAmend;
                    v37.borrow_mut().push((v13, v49));
                    let v50: US0 = US0::US0_JobDoctor;
                    v37.borrow_mut().push((v11, v50));
                    let v51: US0 = US0::US0_JobCoordinator;
                    v37.borrow_mut().push((v9, v51));
                    let v52: US0 = US0::US0_JobUnknown;
                    v37.borrow_mut().push((v7, v52));
                    let v53: US0 = US0::US0_JobStart;
                    v37.borrow_mut().push((v36, v53));
                    let v54: US0 = US0::US0_JobRun;
                    v37.borrow_mut().push((v34, v54));
                    let v55: US0 = US0::US0_JobSupervise;
                    v37.borrow_mut().push((v32, v55));
                    let v56: US0 = US0::US0_JobHost;
                    v37.borrow_mut().push((v30, v56));
                    let v57: US0 = US0::US0_JobStatus;
                    v37.borrow_mut().push((v28, v57));
                    let v58: US0 = US0::US0_JobWait;
                    v37.borrow_mut().push((v26, v58));
                    let v59: US0 = US0::US0_JobList;
                    v37.borrow_mut().push((v24, v59));
                    let v60: US0 = US0::US0_JobTail;
                    v37.borrow_mut().push((v22, v60));
                    let v61: US0 = US0::US0_JobStop;
                    v37.borrow_mut().push((v20, v61));
                    let v62: US0 = US0::US0_JobAfter;
                    v37.borrow_mut().push((v18, v62));
                    let v63: US0 = US0::US0_JobChain;
                    v37.borrow_mut().push((v16, v63));
                    let v64: US0 = US0::US0_JobAmend;
                    v37.borrow_mut().push((v14, v64));
                    let v65: US0 = US0::US0_JobDoctor;
                    v37.borrow_mut().push((v12, v65));
                    let v66: US0 = US0::US0_JobCoordinator;
                    v37.borrow_mut().push((v10, v66));
                    let v67: US0 = US0::US0_JobUnknown;
                    v37.borrow_mut().push((v8, v67));
                    let v68: Rc<Vec<(Rc<str>, US0)>> = Rc::new(v37.borrow().clone());
                    let v69: Rc<RefCell<Vec<(Rc<str>, US0)>>> = Rc::new(RefCell::new((v68).as_ref().clone()));
                    let v70: i32 = (v69.clone().borrow().len() as i32);
                    let v71: US2 = US2::US2_None;
                    let mut v72: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v70, l1: v71 }));
                    while method14(&v72) {
                        let v74: i32 = v72.borrow().l0;
                        let v75: i32 = v74.wrapping_sub(1i32);
                        let v76: US2 = v72.borrow().l1.clone();
                        let (mut v77, mut v78): (Rc<str>, US0) = v69.borrow()[v75 as usize].clone();
                        let v85: US2 = match &v76 {
                            US2::US2_None => {
                                let v80: bool = v77 == v6 ;
                                if v80 {
                                    US2::US2_Some(v78)
                                } else {
                                    US2::US2_None
                                }
                            }
                            US2::US2_Some(v79) => {
                                let v79: US0 = v79.clone();
                                v76
                            }
                        };
                        v72.borrow_mut().l0 = v75;
                        v72.borrow_mut().l1 = v85;
                        ()
                    };
                    let v86: US2 = v72.borrow().l1.clone();
                    let v90: US0 = match v86 {
                        US2::US2_None => {
                            US0::US0_JobUnknown
                        }
                        US2::US2_Some(v87) => {
                            let v87: US0 = v87;
                            v87
                        }
                    };
                    job_operands_1(v90, v5)
                }
                _ => {
                    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job coordinator build --spiral <dir> --kit <dir> (builds the resident job coordinator through Cells --build-only and records its receipt) | job coordinator start --spool <dir> (starts it as a resident job that takes <name>.sub files from the spool) | job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already settled badly prints 'job start failed' with its state, exit and detail and exits with that code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
                    rejected_3(v92)
                }
            }
        }
        _ => {
            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job coordinator build --spiral <dir> --kit <dir> (builds the resident job coordinator through Cells --build-only and records its receipt) | job coordinator start --spool <dir> (starts it as a resident job that takes <name>.sub files from the spool) | job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already settled badly prints 'job start failed' with its state, exit and detail and exits with that code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
            rejected_3(v95)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_run() -> i32 {
    closure0()()
}
pub type SpiralShape_4f6749bfb325 = Mut0;
