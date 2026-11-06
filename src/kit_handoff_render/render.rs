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
    US0_Rendered(Rc<str>),
    US0_Refused(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_StateMissing,
    US1_StateText(Rc<str>),
    US1_StateUnreadable(Rc<str>),
}
pub struct Mut0 { pub l0: Rc<str> }
fn state_host_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (i32, std::rc::Rc<str>) {
    let v2: (i32, std::rc::Rc<str>) = { let path = std::path::Path::new(&*v0).join(&*v1); match std::fs::read_to_string(&path) { Ok(source) => (1, std::rc::Rc::<str>::from(source)), Err(error) if error.kind() == std::io::ErrorKind::NotFound => (0, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }), Err(error) => (2, std::rc::Rc::<str>::from(format!("read {}: {error}", path.display()))) } };
    v2
}
fn state_read_of_2(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let v2: (i32, std::rc::Rc<str>) = state_host_of_3(v0, v1);
    let v3: i32 = v2.clone().0;
    let v4: bool = v3 == 1i32;
    if v4 {
        let v5: Rc<str> = v2.1.clone();
        US1::US1_StateText(v5)
    } else {
        let v7: bool = v3 == 0i32;
        if v7 {
            US1::US1_StateMissing
        } else {
            let v9: Rc<str> = v2.1.clone();
            US1::US1_StateUnreadable(v9)
        }
    }
}
fn sections_block_4(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff sections"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = handoff_sections_projection_domain::kit_handoff_sections_projection(&*v0);
    let v3: bool = &*v2 == "";
    let v7: US0 = if v3 {
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) }));
        US0::US0_Refused(v4)
    } else {
        US0::US0_Rendered(v2)
    };
    match v7 {
        US0::US0_Refused(v8) => {
            let v8: Rc<str> = v8;
            US0::US0_Refused(v8)
        }
        US0::US0_Rendered(v10) => {
            let v10: Rc<str> = v10;
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            let v12: bool = v10.starts_with(&*v11);
            if v12 {
                let v13: Rc<str> = std::rc::Rc::<str>::from(&(&*v10)[2i32 as usize..]);
                let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                US0::US0_Rendered(v14)
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff sections projection returned invalid envelope"); } LIT.with(|lit| lit.clone()) };
                US0::US0_Refused(v16)
            }
        }
    }
}
fn method8(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_7(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn recorded_lines_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let v3: i32 = authority_state_domain::kit_authority_declared_mask(0, 0);
    let v4: i32 = authority_state_domain::kit_authority_state(v3, 0);
    let v5: bool = v3 < 0i32;
    let v7: bool = if v5 {
        true
    } else {
        let v6: bool = v4 < 0i32;
        v6
    };
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiled authority projection rejected"); } LIT.with(|lit| lit.clone()) };
        US0::US0_Refused(v8)
    } else {
        let v10: bool = v4 == 1i32;
        let v13: Rc<str> = if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exclusive"); } LIT.with(|lit| lit.clone()) };
            v11
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
            v12
        };
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nauthority="); } LIT.with(|lit| lit.clone()) }, v13));
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" blocker_mask="); } LIT.with(|lit| lit.clone()) }));
        let v16: Rc<str> = format_real_7(v3);
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }, v2));
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v19));
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n## Recorded state\n\nSaved ratings, census and receipts are shown below with compiled authority declarations. This command does not revalidate their freshness or certify a release.\n\n"); } LIT.with(|lit| lit.clone()) }, v1));
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v20));
        let v24: US0 = US0::US0_Rendered(v23);
        let v54: US0 = match v24 {
            US0::US0_Refused(v25) => {
                let v25: Rc<str> = v25;
                US0::US0_Refused(v25)
            }
            US0::US0_Rendered(v27) => {
                let v27: Rc<str> = v27;
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/toolchain_identity.spi"); } LIT.with(|lit| lit.clone()) };
                let v29: US1 = state_read_of_2(v0.clone(), v28);
                match v29 {
                    US1::US1_StateMissing => {
                        US0::US0_Rendered(v27)
                    }
                    US1::US1_StateText(v33) => {
                        let v33: Rc<str> = v33;
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) };
                        let v35: Rc<str> = handoff_evidence_projection_domain::kit_handoff_toolchain_projection(&*v33);
                        let v36: bool = &*v35 == "";
                        let v40: US0 = if v36 {
                            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v34, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) }));
                            US0::US0_Refused(v37)
                        } else {
                            US0::US0_Rendered(v35)
                        };
                        match v40 {
                            US0::US0_Refused(v41) => {
                                let v41: Rc<str> = v41;
                                US0::US0_Refused(v41)
                            }
                            US0::US0_Rendered(v43) => {
                                let v43: Rc<str> = v43;
                                let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v27, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                                let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v43));
                                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                                US0::US0_Rendered(v46)
                            }
                        }
                    }
                    US1::US1_StateUnreadable(v31) => {
                        let v31: Rc<str> = v31;
                        US0::US0_Refused(v31)
                    }
                }
            }
        };
        match v54 {
            US0::US0_Refused(v55) => {
                let v55: Rc<str> = v55;
                US0::US0_Refused(v55)
            }
            US0::US0_Rendered(v57) => {
                let v57: Rc<str> = v57;
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/cold_proof.spi"); } LIT.with(|lit| lit.clone()) };
                let v59: US1 = state_read_of_2(v0, v58);
                match v59 {
                    US1::US1_StateMissing => {
                        US0::US0_Rendered(v57)
                    }
                    US1::US1_StateText(v63) => {
                        let v63: Rc<str> = v63;
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cold proof"); } LIT.with(|lit| lit.clone()) };
                        let v65: Rc<str> = handoff_evidence_projection_domain::kit_handoff_cold_projection(&*v63);
                        let v66: bool = &*v65 == "";
                        let v70: US0 = if v66 {
                            let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v64, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) }));
                            US0::US0_Refused(v67)
                        } else {
                            US0::US0_Rendered(v65)
                        };
                        match v70 {
                            US0::US0_Refused(v71) => {
                                let v71: Rc<str> = v71;
                                US0::US0_Refused(v71)
                            }
                            US0::US0_Rendered(v73) => {
                                let v73: Rc<str> = v73;
                                let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v57, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                                let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v73));
                                let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                                US0::US0_Rendered(v76)
                            }
                        }
                    }
                    US1::US1_StateUnreadable(v61) => {
                        let v61: Rc<str> = v61;
                        US0::US0_Refused(v61)
                    }
                }
            }
        }
    }
}
fn recorded_block_5(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/ratings.spi"); } LIT.with(|lit| lit.clone()) };
    let v2: US1 = state_read_of_2(v0.clone(), v1);
    match v2 {
        US1::US1_StateMissing => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            US0::US0_Rendered(v3)
        }
        US1::US1_StateText(v7) => {
            let v7: Rc<str> = v7;
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
            let v9: US1 = state_read_of_2(v0.clone(), v8);
            match v9 {
                US1::US1_StateMissing => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    US0::US0_Rendered(v10)
                }
                US1::US1_StateText(v14) => {
                    let v14: Rc<str> = v14;
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ratings"); } LIT.with(|lit| lit.clone()) };
                    let v16: Rc<str> = handoff_state_projection_domain::kit_handoff_ratings_projection(&*v7);
                    let v17: bool = &*v16 == "";
                    let v21: US0 = if v17 {
                        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) }));
                        US0::US0_Refused(v18)
                    } else {
                        US0::US0_Rendered(v16)
                    };
                    match v21 {
                        US0::US0_Refused(v22) => {
                            let v22: Rc<str> = v22;
                            US0::US0_Refused(v22)
                        }
                        US0::US0_Rendered(v24) => {
                            let v24: Rc<str> = v24;
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority census"); } LIT.with(|lit| lit.clone()) };
                            let v26: Rc<str> = handoff_state_projection_domain::kit_handoff_census_projection(&*v14);
                            let v27: bool = &*v26 == "";
                            let v31: US0 = if v27 {
                                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) }));
                                US0::US0_Refused(v28)
                            } else {
                                US0::US0_Rendered(v26)
                            };
                            match v31 {
                                US0::US0_Refused(v32) => {
                                    let v32: Rc<str> = v32;
                                    US0::US0_Refused(v32)
                                }
                                US0::US0_Rendered(v34) => {
                                    let v34: Rc<str> = v34;
                                    recorded_lines_6(v0, v24, v34)
                                }
                            }
                        }
                    }
                }
                US1::US1_StateUnreadable(v12) => {
                    let v12: Rc<str> = v12;
                    US0::US0_Refused(v12)
                }
            }
        }
        US1::US1_StateUnreadable(v5) => {
            let v5: Rc<str> = v5;
            US0::US0_Refused(v5)
        }
    }
}
fn bulleted_10(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v5: i32 = (v4.clone().borrow().len() as i32);
        let v6: bool = v5 > 0i32;
        let v10: bool = if v6 {
            let v7: i32 = v5.wrapping_sub(1i32);
            let v8: Rc<str> = v4.borrow()[v7 as usize].clone();
            let v9: bool = &*v8 == "";
            v9
        } else {
            false
        };
        let v12: i32 = if v10 {
            let v11: i32 = v5.wrapping_sub(1i32);
            v11
        } else {
            v5
        };
        let v13: bool = v1 < v12;
        if v13 {
            let v14: i32 = v1.wrapping_add(1i32);
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n- "); } LIT.with(|lit| lit.clone()) }));
            let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
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
                    v28
                } else {
                    let v33: bool = v29 == 1i32;
                    if v33 {
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v34
                    } else {
                        let v35: i32 = v29.wrapping_sub(2i32);
                        let v36: Rc<str> = string_slice(&v28, 0i32 as i64, v35 as i64);
                        v36
                    }
                }
            } else {
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v39
            };
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v40));
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            (v1, v2) = (v14, v42);
            continue;
        } else {
            return v2;
        }
    }
}
fn active_block_9(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v2: US1 = state_read_of_2(v0, v1);
    match v2 {
        US1::US1_StateMissing => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            US0::US0_Rendered(v3)
        }
        US1::US1_StateText(v7) => {
            let v7: Rc<str> = v7;
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active work"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = handoff_active_work_projection_domain::kit_handoff_active_work_projection(&*kit_agile_records::kit_agile_records_as_lines(&*v7));
            let v10: bool = &*v9 == "";
            let v14: US0 = if v10 {
                let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) }));
                US0::US0_Refused(v11)
            } else {
                US0::US0_Rendered(v9)
            };
            match v14 {
                US0::US0_Refused(v15) => {
                    let v15: Rc<str> = v15;
                    US0::US0_Refused(v15)
                }
                US0::US0_Rendered(v17) => {
                    let v17: Rc<str> = v17;
                    let v18: bool = &*v17 == "ok";
                    if v18 {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        US0::US0_Rendered(v19)
                    } else {
                        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok\n"); } LIT.with(|lit| lit.clone()) };
                        let v22: bool = v17.starts_with(&*v21);
                        if v22 {
                            let v23: Rc<str> = std::rc::Rc::<str>::from(&(&*v17)[3i32 as usize..]);
                            let v24: Rc<str> = { let mut items = (&*v23).lines().filter(|line| !line.is_empty()).collect::<Vec<_>>(); items.sort(); std::rc::Rc::<str>::from(items.join("
")) };
                            let v25: bool = &*v24 == "";
                            let v30: Rc<str> = if v25 {
                                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v26
                            } else {
                                let v27: i32 = 0i32;
                                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n## Active work\n"); } LIT.with(|lit| lit.clone()) };
                                bulleted_10(v24, v27, v28)
                            };
                            US0::US0_Rendered(v30)
                        } else {
                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active work projection returned invalid envelope"); } LIT.with(|lit| lit.clone()) };
                            US0::US0_Refused(v32)
                        }
                    }
                }
            }
        }
        US1::US1_StateUnreadable(v5) => {
            let v5: Rc<str> = v5;
            US0::US0_Refused(v5)
        }
    }
}
fn handoff_text_1(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/handoff.spi"); } LIT.with(|lit| lit.clone()) };
    let v2: US1 = state_read_of_2(v0.clone(), v1);
    match v2 {
        US1::US1_StateMissing => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/handoff.spi is missing"); } LIT.with(|lit| lit.clone()) };
            US0::US0_Refused(v3)
        }
        US1::US1_StateText(v7) => {
            let v7: Rc<str> = v7;
            let v8: US0 = sections_block_4(v7);
            match v8 {
                US0::US0_Refused(v9) => {
                    let v9: Rc<str> = v9;
                    US0::US0_Refused(v9)
                }
                US0::US0_Rendered(v11) => {
                    let v11: Rc<str> = v11;
                    let v12: US0 = recorded_block_5(v0.clone());
                    match v12 {
                        US0::US0_Refused(v13) => {
                            let v13: Rc<str> = v13;
                            US0::US0_Refused(v13)
                        }
                        US0::US0_Rendered(v15) => {
                            let v15: Rc<str> = v15;
                            let v16: US0 = active_block_9(v0);
                            match v16 {
                                US0::US0_Refused(v17) => {
                                    let v17: Rc<str> = v17;
                                    US0::US0_Refused(v17)
                                }
                                US0::US0_Rendered(v19) => {
                                    let v19: Rc<str> = v19;
                                    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# KIT handoff\n"); } LIT.with(|lit| lit.clone()) }, v15));
                                    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v11));
                                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v19));
                                    US0::US0_Rendered(v22)
                                }
                            }
                        }
                    }
                }
            }
        }
        US1::US1_StateUnreadable(v5) => {
            let v5: Rc<str> = v5;
            US0::US0_Refused(v5)
        }
    }
}
fn handoff_render_owned_0(mut v0: Rc<str>) -> i32 {
    let v1: US0 = handoff_text_1(v0);
    match v1 {
        US0::US0_Refused(v2) => {
            let v2: Rc<str> = v2;
            eprintln!("kit error: {}", &*v2);
            2i32
        }
        US0::US0_Rendered(v3) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = { let mut stdout = std::io::stdout().lock(); match std::io::Write::write_all(&mut stdout, (&*v3).as_bytes()).and_then(|()| std::io::Write::flush(&mut stdout)) { Err(error) if error.kind() != std::io::ErrorKind::BrokenPipe => std::rc::Rc::<str>::from(format!("write handoff: {error}")), _ => { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) } } };
            let v5: bool = &*v4 == "";
            if v5 {
                0i32
            } else {
                eprintln!("kit error: {}", &*v4);
                2i32
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        handoff_render_owned_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn handoff_render_owned(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
