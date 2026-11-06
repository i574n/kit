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
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_JobPending,
    US2_JobRunning,
}
#[derive(Clone)]
enum US3 {
    US3_JobFinished(i32),
    US3_JobTimedOut,
    US3_JobStopped,
    US3_JobSpawnFailed(Rc<str>),
    US3_JobSkipped(Rc<str>),
    US3_JobLost,
    US3_JobMemoryCapped,
    US3_JobExpiredInQueue,
}
#[derive(Clone)]
enum US1 {
    US1_Live(US2),
    US1_Settled(US3),
}
#[derive(Clone)]
enum US4 {
    US4_Ok,
    US4_Error(Rc<str>),
}
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_5() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_5();
    joined_3(v1, v0)
}
fn job_file_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_4(v0);
    joined_3(v2, v1)
}
fn text_or_empty_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method8(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_7(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method8(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let mut v6: US0 = US0::US0_None;
    let v7: US0 = v6.clone();
    let v8: US0 = v4.unwrap_or(v7);
    match v8 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn now_ms_9() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_1(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_2(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_6(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = job_file_2(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_6(v5);
    let v7: u64 = decimal_u64_7(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_2(v0, v8);
    let v10: Rc<str> = text_or_empty_6(v9);
    let v11: u64 = decimal_u64_7(v10);
    let v12: u64 = now_ms_9();
    (v3, v7, v11, v12)
}
fn state_of_parts_11(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US1 {
    let mut v3: bool = &*v0 == "pending";
    if v3 {
        let mut v4: US2 = US2::US2_JobPending;
        US1::US1_Live(v4)
    } else {
        let mut v6: bool = &*v0 == "running";
        if v6 {
            let v7: US2 = US2::US2_JobRunning;
            US1::US1_Live(v7)
        } else {
            let v9: bool = &*v0 == "finished";
            if v9 {
                let v10: US3 = US3::US3_JobFinished(v1);
                US1::US1_Settled(v10)
            } else {
                let v12: bool = &*v0 == "timed-out";
                if v12 {
                    let v13: US3 = US3::US3_JobTimedOut;
                    US1::US1_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "stopped";
                    if v15 {
                        let v16: US3 = US3::US3_JobStopped;
                        US1::US1_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "lost";
                        if v18 {
                            let v19: US3 = US3::US3_JobLost;
                            US1::US1_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "memory-capped";
                            if v21 {
                                let v22: US3 = US3::US3_JobMemoryCapped;
                                US1::US1_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "expired-in-queue";
                                if v24 {
                                    let v25: US3 = US3::US3_JobExpiredInQueue;
                                    US1::US1_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "skipped";
                                    if v27 {
                                        let v28: US3 = US3::US3_JobSkipped(v2);
                                        US1::US1_Settled(v28)
                                    } else {
                                        let v30: US3 = US3::US3_JobSpawnFailed(v2);
                                        US1::US1_Settled(v30)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn job_state_of_10(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US1 {
    let mut v4: bool = &*v0 != "";
    if v4 {
        let v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
        let v7: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
        state_of_parts_11(v5, v6, v7)
    } else {
        let v9: bool = v1 > 0u64;
        if v9 {
            let v10: u64 = v1.wrapping_add(15000u64);
            let v11: bool = v10 < v3;
            if v11 {
                let v12: US3 = US3::US3_JobLost;
                US1::US1_Settled(v12)
            } else {
                let v14: US2 = US2::US2_JobRunning;
                US1::US1_Live(v14)
            }
        } else {
            let v17: u64 = v2.wrapping_add(15000u64);
            let v18: bool = v17 < v3;
            if v18 {
                let v19: US3 = US3::US3_JobLost;
                US1::US1_Settled(v19)
            } else {
                let v21: US2 = US2::US2_JobPending;
                US1::US1_Live(v21)
            }
        }
    }
}
fn process_alive_12(mut v0: u64) -> bool {
    let mut v1: bool = kit_process_alive::kit_process_alive(v0);
    v1
}
fn concat5_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn text_written_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn file_exists_15(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn file_exists_16(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn closure2() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let mut v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure2()
}
fn ok_c3() -> Rc<dyn Fn() -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US4> = Rc::new(move || -> US4 {
        US4::US4_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18() -> Rc<dyn Fn() -> US4> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(Rc<str>) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US4> = Rc::new(move |mut v0: Rc<str>| -> US4 {
        US4::US4_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method19() -> Rc<dyn Fn(Rc<str>) -> US4> {
    error_c4()
}
fn method21(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method22(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn hold_orphaned_20(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_2(v0, v1);
    let mut v3: Rc<str> = text_or_empty_6(v2);
    let mut v4: i32 = (v3.clone().len() as i32);
    let v5: i32 = 0i32;
    let mut v6: i32 = method21(v3.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
    let v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method22(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
    let v12: Rc<str> = jobs_root_5();
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = joined_3(v12, v13);
    let v15: Rc<str> = joined_3(v14, v11.clone());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = joined_3(v15, v16);
    let v18: Rc<str> = text_or_empty_6(v17);
    let v19: u64 = decimal_u64_7(v18);
    let v20: bool = kit_process_alive::kit_process_home_abandoned(&v11, v19);
    v20
}
fn orphan_settled_0(mut v0: Rc<str>) -> bool {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_1(v0.clone());
    let v5: US1 = job_state_of_10(v1.clone(), v2, v3, v4);
    match v5 {
        US1::US1_Settled(v6) => {
            let mut v6: US3 = v6;
            match v6 {
                US3::US3_JobLost => {
                    let v7: bool = &*v1 == "";
                    if v7 {
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                        let v9: Rc<str> = job_file_2(v0.clone(), v8);
                        let v10: Rc<str> = text_or_empty_6(v9);
                        let v11: u64 = decimal_u64_7(v10);
                        let v12: Rc<str> = jobs_root_5();
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                        let v14: Rc<str> = joined_3(v12, v13);
                        let v15: Rc<str> = joined_3(v14, v0.clone());
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = job_file_2(v0.clone(), v16);
                        let v18: Rc<str> = text_or_empty_6(v17);
                        let v19: u64 = decimal_u64_7(v18);
                        let v20: bool = v11 == 0u64;
                        let v23: bool = if v20 {
                            true
                        } else {
                            let v21: bool = process_alive_12(v11);
                            let v22: bool = v21 == false;
                            v22
                        };
                        let v24: bool = v19 > 0u64;
                        let v27: bool = if v24 {
                            let v25: u64 = v19.wrapping_add(60000u64);
                            let v26: bool = v25 < v4;
                            v26
                        } else {
                            false
                        };
                        let v28: bool = v27 && v23;
                        if v28 {
                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                            let v30: Rc<str> = job_file_2(v0, v29);
                            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop requested and no live supervisor or host acted on it within 60 s"); } LIT.with(|lit| lit.clone()) };
                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                            let v33: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
                            let v34: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v38: Rc<str> = concat5_13(v32, v35.clone(), v31, v36, v37);
                            let v39: Rc<str> = concat5_13(v33, v35.clone(), v34, v35.clone(), v38);
                            let v40: Rc<str> = text_written_14(v30, v39);
                            let v41: bool = file_exists_15(v15.clone());
                            if v41 {
                                let v42: bool = file_exists_16(v15.clone());
                                let mut v46: Result<(), std::io::Error> = if v42 {
                                    let v44: Result<(), std::io::Error> = std::fs::remove_file(&*v15);
                                    v44
                                } else {
                                    let v45: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
                                    v45
                                };
                                let mut v47: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method17();
                                let v49: Result<(), Rc<str>> = v46.map_err(|x| v47(x));
                                let mut v50: Rc<dyn Fn() -> US4> = method18();
                                let mut v51: Rc<dyn Fn(Rc<str>) -> US4> = method19();
                                let v52: US4 = match v49 { Ok(()) => v50(), Err(e) => v51(e) };
                                match v52 {
                                    US4::US4_Error(v53) => {
                                        let v53: Rc<str> = v53;
                                        let v54: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.file_delete: "); } LIT.with(|lit| lit.clone()) }, v53));
                                        eprintln!("{}", v54);
                                        ()
                                    }
                                    US4::US4_Ok => {
                                        ()
                                    }
                                }
                            };
                            true
                        } else {
                            let v55: bool = file_exists_15(v15.clone());
                            let v57: bool = if v55 {
                                hold_orphaned_20(v0.clone())
                            } else {
                                false
                            };
                            if v57 {
                                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                let v59: Rc<str> = job_file_2(v0, v58);
                                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held by a host whose owner session and host process are gone; it never started"); } LIT.with(|lit| lit.clone()) };
                                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                                let v62: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                                let v63: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
                                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v67: Rc<str> = concat5_13(v61, v64.clone(), v60, v65, v66);
                                let v68: Rc<str> = concat5_13(v62, v64.clone(), v63, v64.clone(), v67);
                                let v69: Rc<str> = text_written_14(v59, v68);
                                let v70: bool = file_exists_16(v15.clone());
                                let mut v74: Result<(), std::io::Error> = if v70 {
                                    let v72: Result<(), std::io::Error> = std::fs::remove_file(&*v15);
                                    v72
                                } else {
                                    let v73: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
                                    v73
                                };
                                let mut v75: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method17();
                                let v77: Result<(), Rc<str>> = v74.map_err(|x| v75(x));
                                let mut v78: Rc<dyn Fn() -> US4> = method18();
                                let mut v79: Rc<dyn Fn(Rc<str>) -> US4> = method19();
                                let v80: US4 = match v77 { Ok(()) => v78(), Err(e) => v79(e) };
                                match v80 {
                                    US4::US4_Error(v81) => {
                                        let v81: Rc<str> = v81;
                                        let v82: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.file_delete: "); } LIT.with(|lit| lit.clone()) }, v81));
                                        eprintln!("{}", v82);
                                        ()
                                    }
                                    US4::US4_Ok => {
                                        ()
                                    }
                                };
                                true
                            } else {
                                false
                            }
                        }
                    } else {
                        false
                    }
                }
                _ => {
                    false
                }
            }
        }
        _ => {
            false
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        orphan_settled_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_orphan_settled(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
