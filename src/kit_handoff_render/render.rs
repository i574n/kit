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
struct Mut0 { l0: Rc<str> }
fn state_host_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (i32, std::rc::Rc<str>) {
    let v2: (i32, std::rc::Rc<str>) = { let path = std::path::Path::new(&*v0).join(&*v1); match std::fs::read_to_string(&path) { Ok(source) => (1, std::rc::Rc::<str>::from(source)), Err(error) if error.kind() == std::io::ErrorKind::NotFound => (0, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }), Err(error) => (2, std::rc::Rc::<str>::from(format!("read {}: {error}", path.display()))) } };
    v2.clone()
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
    let v8: US0 = if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff sections projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
        US0::US0_Refused(v5.clone())
    } else {
        US0::US0_Rendered(v2.clone())
    };
    match &v8 {
        US0::US0_Refused(v9) => {
            let v9: Rc<str> = v9.clone();
            US0::US0_Refused(v9.clone())
        }
        US0::US0_Rendered(v11) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            let v13: bool = v11.starts_with(&*v12);
            if v13 {
                let v14: Rc<str> = std::rc::Rc::<str>::from(&(&*v11)[2i32 as usize..]);
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
                US0::US0_Rendered(v16.clone())
            } else {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff sections projection returned invalid envelope"); } LIT.with(|lit| lit.clone()) };
                US0::US0_Refused(v18.clone())
            }
        }
    }
}
fn method8(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_7(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
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
        US0::US0_Refused(v8.clone())
    } else {
        let v10: bool = v4 == 1i32;
        let v13: Rc<str> = if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exclusive"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        };
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nauthority="); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" blocker_mask="); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
        let v18: Rc<str> = format_real_7(v3);
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v2));
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v19, Rc::<str>::from(format!("{}{}", v21, v22))));
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n## Recorded state\n\nSaved ratings, census and receipts are shown below with compiled authority declarations. This command does not revalidate their freshness or certify a release.\n"); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v25, v22)), v1)), v22)), v24));
        let v30: US0 = US0::US0_Rendered(v29.clone());
        let v61: US0 = match &v30 {
            US0::US0_Refused(v31) => {
                let v31: Rc<str> = v31.clone();
                US0::US0_Refused(v31.clone())
            }
            US0::US0_Rendered(v33) => {
                let v33: Rc<str> = v33.clone();
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/toolchain_identity.spi"); } LIT.with(|lit| lit.clone()) };
                let v35: US1 = state_read_of_2(v0.clone(), v34.clone());
                match &v35 {
                    US1::US1_StateMissing => {
                        US0::US0_Rendered(v33.clone())
                    }
                    US1::US1_StateText(v39) => {
                        let v39: Rc<str> = v39.clone();
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = handoff_evidence_projection_domain::kit_handoff_toolchain_projection(&*v39);
                        let v42: bool = &*v41 == "";
                        let v47: US0 = if v42 {
                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                            US0::US0_Refused(v44.clone())
                        } else {
                            US0::US0_Rendered(v41.clone())
                        };
                        match &v47 {
                            US0::US0_Refused(v48) => {
                                let v48: Rc<str> = v48.clone();
                                US0::US0_Refused(v48.clone())
                            }
                            US0::US0_Rendered(v50) => {
                                let v50: Rc<str> = v50.clone();
                                let v53: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v33, v22)), v50)), v22));
                                US0::US0_Rendered(v53.clone())
                            }
                        }
                    }
                    US1::US1_StateUnreadable(v37) => {
                        let v37: Rc<str> = v37.clone();
                        US0::US0_Refused(v37.clone())
                    }
                }
            }
        };
        match &v61 {
            US0::US0_Refused(v62) => {
                let v62: Rc<str> = v62.clone();
                US0::US0_Refused(v62.clone())
            }
            US0::US0_Rendered(v64) => {
                let v64: Rc<str> = v64.clone();
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/cold_proof.spi"); } LIT.with(|lit| lit.clone()) };
                let v66: US1 = state_read_of_2(v0.clone(), v65.clone());
                match &v66 {
                    US1::US1_StateMissing => {
                        US0::US0_Rendered(v64.clone())
                    }
                    US1::US1_StateText(v70) => {
                        let v70: Rc<str> = v70.clone();
                        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cold proof"); } LIT.with(|lit| lit.clone()) };
                        let v72: Rc<str> = handoff_evidence_projection_domain::kit_handoff_cold_projection(&*v70);
                        let v73: bool = &*v72 == "";
                        let v78: US0 = if v73 {
                            let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cold proof projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                            US0::US0_Refused(v75.clone())
                        } else {
                            US0::US0_Rendered(v72.clone())
                        };
                        match &v78 {
                            US0::US0_Refused(v79) => {
                                let v79: Rc<str> = v79.clone();
                                US0::US0_Refused(v79.clone())
                            }
                            US0::US0_Rendered(v81) => {
                                let v81: Rc<str> = v81.clone();
                                let v84: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v64, v22)), v81)), v22));
                                US0::US0_Rendered(v84.clone())
                            }
                        }
                    }
                    US1::US1_StateUnreadable(v68) => {
                        let v68: Rc<str> = v68.clone();
                        US0::US0_Refused(v68.clone())
                    }
                }
            }
        }
    }
}
fn recorded_block_5(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/ratings.spi"); } LIT.with(|lit| lit.clone()) };
    let v2: US1 = state_read_of_2(v0.clone(), v1.clone());
    match &v2 {
        US1::US1_StateMissing => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            US0::US0_Rendered(v3.clone())
        }
        US1::US1_StateText(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
            let v9: US1 = state_read_of_2(v0.clone(), v8.clone());
            match &v9 {
                US1::US1_StateMissing => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    US0::US0_Rendered(v10.clone())
                }
                US1::US1_StateText(v14) => {
                    let v14: Rc<str> = v14.clone();
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ratings"); } LIT.with(|lit| lit.clone()) };
                    let v16: Rc<str> = handoff_state_projection_domain::kit_handoff_ratings_projection(&*v7);
                    let v17: bool = &*v16 == "";
                    let v22: US0 = if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ratings projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                        US0::US0_Refused(v19.clone())
                    } else {
                        US0::US0_Rendered(v16.clone())
                    };
                    match &v22 {
                        US0::US0_Refused(v23) => {
                            let v23: Rc<str> = v23.clone();
                            US0::US0_Refused(v23.clone())
                        }
                        US0::US0_Rendered(v25) => {
                            let v25: Rc<str> = v25.clone();
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority census"); } LIT.with(|lit| lit.clone()) };
                            let v27: Rc<str> = handoff_state_projection_domain::kit_handoff_census_projection(&*v14);
                            let v28: bool = &*v27 == "";
                            let v33: US0 = if v28 {
                                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority census projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                                US0::US0_Refused(v30.clone())
                            } else {
                                US0::US0_Rendered(v27.clone())
                            };
                            match &v33 {
                                US0::US0_Refused(v34) => {
                                    let v34: Rc<str> = v34.clone();
                                    US0::US0_Refused(v34.clone())
                                }
                                US0::US0_Rendered(v36) => {
                                    let v36: Rc<str> = v36.clone();
                                    recorded_lines_6(v0.clone(), v25.clone(), v36.clone())
                                }
                            }
                        }
                    }
                }
                US1::US1_StateUnreadable(v12) => {
                    let v12: Rc<str> = v12.clone();
                    US0::US0_Refused(v12.clone())
                }
            }
        }
        US1::US1_StateUnreadable(v5) => {
            let v5: Rc<str> = v5.clone();
            US0::US0_Refused(v5.clone())
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
            let v8: Rc<str> = v4.clone().borrow()[v7 as usize].clone();
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
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n- "); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v15));
            let v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v18: bool = v1 >= 0i32;
            let v28: bool = if v18 {
                let v19: i32 = (v17.clone().borrow().len() as i32);
                let v20: bool = v19 > 0i32;
                let v24: bool = if v20 {
                    let v21: i32 = v19.wrapping_sub(1i32);
                    let v22: Rc<str> = v17.clone().borrow()[v21 as usize].clone();
                    let v23: bool = &*v22 == "";
                    v23
                } else {
                    false
                };
                let v26: i32 = if v24 {
                    let v25: i32 = v19.wrapping_sub(1i32);
                    v25
                } else {
                    v19
                };
                let v27: bool = v1 < v26;
                v27
            } else {
                false
            };
            let v41: Rc<str> = if v28 {
                let v29: Rc<str> = v17.clone().borrow()[v1 as usize].clone();
                let v30: i32 = (v29.clone().len() as i32);
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v32: bool = v29.ends_with(&*v31);
                let v33: bool = v32 == false;
                if v33 {
                    v29.clone()
                } else {
                    let v34: bool = v30 == 1i32;
                    if v34 {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v35.clone()
                    } else {
                        let v36: i32 = v30.wrapping_sub(2i32);
                        let v37: Rc<str> = string_slice(&v29.clone(), 0i32 as i64, v36 as i64);
                        v37.clone()
                    }
                }
            } else {
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v40.clone()
            };
            let v43: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v16, v41)), v3));
            (v0, v1, v2) = (v0.clone(), v14, v43.clone());
            continue;
        } else {
            return v2.clone();
        }
    }
}
fn active_block_9(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v2: US1 = state_read_of_2(v0.clone(), v1.clone());
    match &v2 {
        US1::US1_StateMissing => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            US0::US0_Rendered(v3.clone())
        }
        US1::US1_StateText(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active work"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = handoff_active_work_projection_domain::kit_handoff_active_work_projection(&*kit_agile_records::kit_agile_records_as_lines(&*v7));
            let v10: bool = &*v9 == "";
            let v15: US0 = if v10 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active work projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                US0::US0_Refused(v12.clone())
            } else {
                US0::US0_Rendered(v9.clone())
            };
            match &v15 {
                US0::US0_Refused(v16) => {
                    let v16: Rc<str> = v16.clone();
                    US0::US0_Refused(v16.clone())
                }
                US0::US0_Rendered(v18) => {
                    let v18: Rc<str> = v18.clone();
                    let v19: bool = &*v18 == "ok";
                    if v19 {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        US0::US0_Rendered(v20.clone())
                    } else {
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok\n"); } LIT.with(|lit| lit.clone()) };
                        let v23: bool = v18.starts_with(&*v22);
                        if v23 {
                            let v24: Rc<str> = std::rc::Rc::<str>::from(&(&*v18)[3i32 as usize..]);
                            let v25: Rc<str> = { let mut items = (&*v24).lines().filter(|line| !line.is_empty()).collect::<Vec<_>>(); items.sort(); std::rc::Rc::<str>::from(items.join("
")) };
                            let v26: bool = &*v25 == "";
                            let v31: Rc<str> = if v26 {
                                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v27.clone()
                            } else {
                                let v28: i32 = 0i32;
                                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n## Active work\n"); } LIT.with(|lit| lit.clone()) };
                                bulleted_10(v25.clone(), v28, v29.clone())
                            };
                            US0::US0_Rendered(v31.clone())
                        } else {
                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active work projection returned invalid envelope"); } LIT.with(|lit| lit.clone()) };
                            US0::US0_Refused(v33.clone())
                        }
                    }
                }
            }
        }
        US1::US1_StateUnreadable(v5) => {
            let v5: Rc<str> = v5.clone();
            US0::US0_Refused(v5.clone())
        }
    }
}
fn handoff_text_1(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/handoff.spi"); } LIT.with(|lit| lit.clone()) };
    let v2: US1 = state_read_of_2(v0.clone(), v1.clone());
    match &v2 {
        US1::US1_StateMissing => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/handoff.spi is missing"); } LIT.with(|lit| lit.clone()) };
            US0::US0_Refused(v3.clone())
        }
        US1::US1_StateText(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: US0 = sections_block_4(v7.clone());
            match &v8 {
                US0::US0_Refused(v9) => {
                    let v9: Rc<str> = v9.clone();
                    US0::US0_Refused(v9.clone())
                }
                US0::US0_Rendered(v11) => {
                    let v11: Rc<str> = v11.clone();
                    let v12: US0 = recorded_block_5(v0.clone());
                    match &v12 {
                        US0::US0_Refused(v13) => {
                            let v13: Rc<str> = v13.clone();
                            US0::US0_Refused(v13.clone())
                        }
                        US0::US0_Rendered(v15) => {
                            let v15: Rc<str> = v15.clone();
                            let v16: US0 = active_block_9(v0.clone());
                            match &v16 {
                                US0::US0_Refused(v17) => {
                                    let v17: Rc<str> = v17.clone();
                                    US0::US0_Refused(v17.clone())
                                }
                                US0::US0_Rendered(v19) => {
                                    let v19: Rc<str> = v19.clone();
                                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# KIT handoff\n"); } LIT.with(|lit| lit.clone()) };
                                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v20, v15)), v11)), v19));
                                    US0::US0_Rendered(v23.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
        US1::US1_StateUnreadable(v5) => {
            let v5: Rc<str> = v5.clone();
            US0::US0_Refused(v5.clone())
        }
    }
}
fn handoff_render_owned_0(mut v0: Rc<str>) -> i32 {
    let v1: US0 = handoff_text_1(v0.clone());
    match &v1 {
        US0::US0_Refused(v2) => {
            let v2: Rc<str> = v2.clone();
            eprintln!("kit error: {}", &*v2);
            2i32
        }
        US0::US0_Rendered(v3) => {
            let v3: Rc<str> = v3.clone();
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
        handoff_render_owned_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn handoff_render_owned(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
