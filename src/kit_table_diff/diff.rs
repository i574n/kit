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
    UH0_NoPairs,
    UH0_PairThen(Rc<str>, i32, i32, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_NoDifferences,
    UH1_DifferenceThen(Rc<str>, i32, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US0_TableUnreadable(Rc<str>),
}
fn concat5_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn method7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn loop_5(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let v4: bool = v3 >= v2;
        if v4 {
            return false;
        } else {
            let mut v5: Rc<str> = v1.borrow()[v3 as usize].clone();
            let v6: i32 = (v5.clone().len() as i32);
            let v7: i32 = 0i32;
            let v8: i32 = method6(v5.clone(), v6, v7);
            let v9: i32 = v6.wrapping_sub(1i32);
            let v10: Rc<str> = string_slice(&v5, v8 as i64, v9 as i64);
            let v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = method7(v10.clone(), v11);
            let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
            let v14: bool = &*v13 == &*v0;
            if v14 {
                return true;
            } else {
                let v15: i32 = v3.wrapping_add(1i32);
                v3 = v15;
                continue;
            }
        }
    }
}
fn shared_columns_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = &*v1 == "";
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let v7: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let v8: Rc<str> = shared_columns_3(v0.clone(), v7, v2.clone());
        let v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v10: i32 = (v9.clone().borrow().len() as i32);
        let v11: i32 = 0i32;
        let v12: bool = loop_5(v6.clone(), &v9, v10, v11);
        let v15: bool = if v12 {
            true
        } else {
            let v13: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
            let v14: bool = v13 < 0i32;
            v14
        };
        if v15 {
            v8
        } else {
            let v16: bool = &*v6 == "";
            if v16 {
                v8
            } else {
                let v17: bool = &*v8 == "";
                if v17 {
                    v6
                } else {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_4(v6, v5, v8, v18.clone(), v18.clone())
                }
            }
        }
    }
}
fn absent_from_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = &*v2 == "";
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v2).split(&*v5).next().unwrap_or(""));
        let v7: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let v8: Rc<str> = absent_from_8(v0.clone(), v1.clone(), v7);
        let v9: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
        let v10: bool = v9 < 0i32;
        let v13: bool = if v10 {
            true
        } else {
            let v11: i32 = (v1).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
            let v12: bool = v11 < 0i32;
            v12
        };
        if v13 {
            let v14: bool = &*v6 == "";
            if v14 {
                v8
            } else {
                let v15: bool = &*v8 == "";
                if v15 {
                    v6
                } else {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_4(v6, v5, v8, v16.clone(), v16.clone())
                }
            }
        } else {
            v8
        }
    }
}
fn indices_from_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v1 == "";
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v4).next().unwrap_or(""));
        let v6: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v5)).map(|index| index as i32).unwrap_or(-1);
        let v7: Rc<str> = std::rc::Rc::<str>::from((v6).to_string().as_str());
        let v8: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v4).map(|(_, rest)| rest).unwrap_or(""));
        let v9: Rc<str> = indices_from_9(v0, v8);
        let v10: bool = &*v7 == "";
        if v10 {
            v9
        } else {
            let v11: bool = &*v9 == "";
            if v11 {
                v7
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                concat5_4(v7, v4, v9, v12.clone(), v12.clone())
            }
        }
    }
}
fn winners_from_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>) -> std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> {
    loop {
        let v6: bool = v4 < v3;
        if v6 {
            return v5;
        } else {
            let v7: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(44u8)).map(|index| index.parse::<usize>().ok().and_then(|index| (v0).get(v3 as usize).and_then(|cells| cells.get(index))).map(|cell| cell.to_string()).unwrap_or_default()).collect::<Vec<_>>().join("|").as_str());
            let v8: i32 = (v5).borrow().get(&*v7).copied().unwrap_or(-1);
            let v9: bool = v8 < 0i32;
            let v11: bool = if v9 {
                true
            } else {
                let v10: bool = v2 < 0i32;
                v10
            };
            let v23: bool = if v11 {
                true
            } else {
                let v12: Rc<str> = std::rc::Rc::<str>::from((v0).get(v3 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
                let v13: Rc<str> = std::rc::Rc::<str>::from((v0).get(v8 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
                let v14: bool = (v12).trim().parse::<f64>().map(|value| value.is_finite()).unwrap_or(false);
                let v16: bool = if v14 {
                    let v15: bool = (v13).trim().parse::<f64>().map(|value| value.is_finite()).unwrap_or(false);
                    v15
                } else {
                    false
                };
                if v16 {
                    let v17: f64 = (v13).trim().parse::<f64>().unwrap_or(0.0);
                    let v18: f64 = (v12).trim().parse::<f64>().unwrap_or(0.0);
                    let v19: bool = v17 <= v18;
                    v19
                } else {
                    let v20: bool = (v12) < (v13);
                    let v21: bool = v20 == false;
                    v21
                }
            };
            if v23 {
                { (v5).borrow_mut().insert(v7.clone(), v3); };
                ()
            };
            let v24: i32 = v3.wrapping_add(1i32);
            v3 = v24;
            continue;
        }
    }
}
fn pairs_from_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: Rc<str>) -> Rc<UH0> {
    let mut v3: bool = &*v2 == "";
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoPairs); } CASE.with(|case| case.clone()) }
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v2).split(&*v5).next().unwrap_or(""));
        let v7: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
        let v8: i32 = (v1).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
        let v9: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let v10: Rc<UH0> = pairs_from_11(v0, v1, v9);
        Rc::new(UH0::UH0_PairThen(v6, v7, v8, v10))
    }
}
fn difference_counted_14(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_DifferenceThen(v4, v5, v6) => {
            let v4: Rc<str> = v4.clone();
            let mut v5: i32 = *v5;
            let v6: Rc<UH1> = v6.clone();
            let v7: bool = &*v4 == &*v1;
            if v7 {
                let v8: i32 = v5.wrapping_add(1i32);
                Rc::new(UH1::UH1_DifferenceThen(v4, v8, v6))
            } else {
                let v10: bool = (v1.clone()) < (v4.clone());
                if v10 {
                    Rc::new(UH1::UH1_DifferenceThen(v1, 1i32, v0))
                } else {
                    let v12: Rc<UH1> = difference_counted_14(v6, v1);
                    Rc::new(UH1::UH1_DifferenceThen(v4, v5, v12))
                }
            }
        }
        UH1::UH1_NoDifferences => {
            let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoDifferences); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_DifferenceThen(v1, 1i32, v2))
        }
    }
}
fn pairs_compared_13(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: u64, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: Rc<str>, mut v11: Rc<UH1>) -> (i32, i32, i32, Rc<str>, Rc<UH1>) {
    let (mut v55, mut v56, mut v57, mut v58, mut v59): (i32, i32, i32, Rc<str>, Rc<UH1>) = match &*v0 {
        UH0::UH0_NoPairs => {
            (v7, v8, v9, v10, v11)
        }
        _ => {
            let (mut v50, mut v51, mut v52, mut v53, mut v54): (i32, i32, i32, Rc<str>, Rc<UH1>) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_PairThen(v12, v13, v14, v15) => {
                    let v12: Rc<str> = v12;
                    let v13: i32 = v13;
                    let v14: i32 = v14;
                    let v15: Rc<UH0> = v15;
                    let v16: Rc<str> = std::rc::Rc::<str>::from((v1).get(v3 as usize).and_then(|cells| cells.get(v13 as usize)).map(|cell| &**cell).unwrap_or(""));
                    let v17: Rc<str> = std::rc::Rc::<str>::from((v2).get(v4 as usize).and_then(|cells| cells.get(v14 as usize)).map(|cell| &**cell).unwrap_or(""));
                    let v18: bool = &*v16 == &*v17;
                    if v18 {
                        let (mut v19, mut v20, mut v21, mut v22, mut v23): (i32, i32, i32, Rc<str>, Rc<UH1>) = pairs_compared_13(v15, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11);
                        (v19, v20, v21, v22, v23)
                    } else {
                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) }));
                        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v12.clone()));
                        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) }));
                        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) }));
                        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v17));
                        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v28));
                        let v30: bool = (v9 as u64) < v6;
                        let v31: Rc<UH1> = difference_counted_14(v11, v12);
                        let v33: i32 = if v30 {
                            let v32: i32 = v9.wrapping_add(1i32);
                            v32
                        } else {
                            v9
                        };
                        let v39: Rc<str> = if v30 {
                            let v34: bool = &*v10 == "";
                            if v34 {
                                v29
                            } else {
                                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                concat5_4(v10, v35, v29, v36.clone(), v36.clone())
                            }
                        } else {
                            v10
                        };
                        let (mut v40, mut v41, mut v42, mut v43, mut v44): (i32, i32, i32, Rc<str>, Rc<UH1>) = pairs_compared_13(v15, v1, v2, v3, v4, v5, v6, v7, v8, v33, v39, v31);
                        (v40, v41, v42, v43, v44)
                    }
                }
                _ => unreachable!(),
            };
            (v50, v51, v52, v53, v54)
        }
    };
    (v55, v56, v57, v58, v59)
}
fn left_compared_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: Rc<str>, mut v3: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v4: bool, mut v5: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v6: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v7: Rc<UH0>, mut v8: u64, mut v9: i32, mut v10: i32, mut v11: i32, mut v12: i32, mut v13: Rc<str>, mut v14: Rc<UH1>) -> (i32, i32, i32, Rc<str>, Rc<UH1>) {
    let v15: i32 = ((v0).len().max(1) - 1) as i32;
    let v16: bool = v15 < v9;
    let (mut v52, mut v53, mut v54, mut v55, mut v56): (i32, i32, i32, Rc<str>, Rc<UH1>) = if v16 {
        (v10, v11, v12, v13, v14)
    } else {
        let v17: Rc<str> = std::rc::Rc::<str>::from((v2).split(char::from(44u8)).map(|index| index.parse::<usize>().ok().and_then(|index| (v0).get(v9 as usize).and_then(|cells| cells.get(index))).map(|cell| cell.to_string()).unwrap_or_default()).collect::<Vec<_>>().join("|").as_str());
        let v20: bool = if v4 {
            let v18: i32 = (v3).borrow().get(&*v17).copied().unwrap_or(-1);
            let v19: bool = v18 != v9;
            v19
        } else {
            false
        };
        let (mut v47, mut v48, mut v49, mut v50, mut v51): (i32, i32, i32, Rc<str>, Rc<UH1>) = if v20 {
            let v21: i32 = v9.wrapping_add(1i32);
            let (mut v22, mut v23, mut v24, mut v25, mut v26): (i32, i32, i32, Rc<str>, Rc<UH1>) = left_compared_12(v0, v1, v2, v3, v4, v5, v6, v7, v8, v21, v10, v11, v12, v13, v14);
            (v22, v23, v24, v25, v26)
        } else {
            { (v6).borrow_mut().insert(v17.clone(), v9); };
            let v27: i32 = (v5).borrow().get(&*v17).copied().unwrap_or(-1);
            let v28: i32 = v10.wrapping_add(1i32);
            let v29: bool = v27 < 0i32;
            let (mut v36, mut v37, mut v38, mut v39, mut v40): (i32, i32, i32, Rc<str>, Rc<UH1>) = if v29 {
                let v30: i32 = v11.wrapping_add(1i32);
                (v28, v30, v12, v13, v14)
            } else {
                let (mut v31, mut v32, mut v33, mut v34, mut v35): (i32, i32, i32, Rc<str>, Rc<UH1>) = pairs_compared_13(v7.clone(), v0.clone(), v1.clone(), v9, v27, v17, v8, v28, v11, v12, v13, v14);
                (v31, v32, v33, v34, v35)
            };
            let v41: i32 = v9.wrapping_add(1i32);
            let (mut v42, mut v43, mut v44, mut v45, mut v46): (i32, i32, i32, Rc<str>, Rc<UH1>) = left_compared_12(v0, v1, v2, v3, v4, v5, v6, v7, v8, v41, v36, v37, v38, v39, v40);
            (v42, v43, v44, v45, v46)
        };
        (v47, v48, v49, v50, v51)
    };
    (v52, v53, v54, v55, v56)
}
fn right_unseen_15(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v3: bool, mut v4: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v5: i32, mut v6: i32, mut v7: i32) -> (i32, i32) {
    loop {
        let v8: i32 = ((v0).len().max(1) - 1) as i32;
        let v9: bool = v8 < v5;
        if v9 {
            return (v6, v7);
        } else {
            let v10: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(44u8)).map(|index| index.parse::<usize>().ok().and_then(|index| (v0).get(v5 as usize).and_then(|cells| cells.get(index))).map(|cell| cell.to_string()).unwrap_or_default()).collect::<Vec<_>>().join("|").as_str());
            let v13: bool = if v3 {
                let v11: i32 = (v2).borrow().get(&*v10).copied().unwrap_or(-1);
                let v12: bool = v11 != v5;
                v12
            } else {
                false
            };
            if v13 {
                let v14: i32 = v5.wrapping_add(1i32);
                v5 = v14;
                continue;
            } else {
                let v17: i32 = (v4).borrow().get(&*v10).copied().unwrap_or(-1);
                let v18: bool = v17 < 0i32;
                let v19: i32 = if v18 {
                    1i32
                } else {
                    0i32
                };
                let v20: i32 = v5.wrapping_add(1i32);
                let v21: i32 = v6.wrapping_add(1i32);
                let v22: i32 = v7.wrapping_add(v19);
                (v5, v6, v7) = (v20, v21, v22);
                continue;
            }
        }
    }
}
fn differences_printed_16(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_DifferenceThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let mut v3: i32 = v3;
                let v4: Rc<UH1> = v4;
                let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("column "); } LIT.with(|lit| lit.clone()) }, v2));
                let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" differs="); } LIT.with(|lit| lit.clone()) }));
                let v7: Rc<str> = std::rc::Rc::<str>::from((v3).to_string().as_str());
                let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
                println!("{}", v8);
                let v9: i32 = v1.wrapping_add(v3);
                (v0, v1) = (v4, v9);
                continue;
            }
            UH1::UH1_NoDifferences => {
                return v1;
            }
        }
    }
}
fn diff_tables_2(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool) -> i32 {
    let v10: bool = &*v3 == "";
    let v14: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from((v0).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v11)).unwrap_or_default().as_str());
        shared_columns_3(v1.clone(), v12, v5.clone())
    } else {
        v3
    };
    let v15: Rc<str> = absent_from_8(v0.clone(), v1.clone(), v5.clone());
    let v16: Rc<str> = absent_from_8(v0.clone(), v1.clone(), v14.clone());
    let v17: bool = &*v15 == "";
    let v23: Rc<str> = if v17 {
        v16
    } else {
        let v18: bool = &*v16 == "";
        if v18 {
            v15
        } else {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_4(v15, v19, v16, v20.clone(), v20.clone())
        }
    };
    let v24: bool = &*v8 != "";
    let v30: bool = if v24 {
        let v25: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v8)).map(|index| index as i32).unwrap_or(-1);
        let v26: bool = v25 < 0i32;
        if v26 {
            true
        } else {
            let v27: i32 = (v1).first().and_then(|header| header.iter().position(|item| &**item == &*v8)).map(|index| index as i32).unwrap_or(-1);
            let v28: bool = v27 < 0i32;
            v28
        }
    } else {
        false
    };
    let v32: Rc<str> = if v30 {
        v8.clone()
    } else {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v31
    };
    let v33: bool = &*v5 == "";
    if v33 {
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff needs --key a,b"); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v34));
        eprintln!("{}", v35);
        2i32
    } else {
        let v36: bool = &*v23 == "";
        let v42: Rc<str> = if v36 {
            v32.clone()
        } else {
            let v37: bool = &*v32 == "";
            if v37 {
                v23.clone()
            } else {
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                concat5_4(v23.clone(), v38, v32.clone(), v39.clone(), v39.clone())
            }
        };
        let v43: bool = &*v42 != "";
        if v43 {
            let v49: Rc<str> = if v36 {
                v32
            } else {
                let v44: bool = &*v32 == "";
                if v44 {
                    v23
                } else {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_4(v23, v45, v32, v46.clone(), v46.clone())
                }
            };
            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff unknown columns: "); } LIT.with(|lit| lit.clone()) }, v49));
            let v51: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v50));
            eprintln!("{}", v51);
            2i32
        } else {
            let v52: Rc<str> = indices_from_9(v0.clone(), v5.clone());
            let v53: Rc<str> = indices_from_9(v1.clone(), v5);
            let v55: i32 = if v24 {
                let v54: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v8)).map(|index| index as i32).unwrap_or(-1);
                v54
            } else {
                -1i32
            };
            let v56: i32 = 1i32;
            let v57: i32 = ((v0).len().max(1) - 1) as i32;
            let v58: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
            let v59: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = winners_from_10(v0.clone(), v52.clone(), v55, v56, v57, v58);
            let v61: i32 = if v24 {
                let v60: i32 = (v1).first().and_then(|header| header.iter().position(|item| &**item == &*v8)).map(|index| index as i32).unwrap_or(-1);
                v60
            } else {
                -1i32
            };
            let v62: i32 = 1i32;
            let v63: i32 = ((v1).len().max(1) - 1) as i32;
            let v64: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
            let v65: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = winners_from_10(v1.clone(), v53.clone(), v61, v62, v63, v64);
            let v66: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
            let v67: Rc<UH0> = pairs_from_11(v0.clone(), v1.clone(), v14);
            let v68: i32 = 1i32;
            let v69: i32 = 0i32;
            let v70: i32 = 0i32;
            let v71: i32 = 0i32;
            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v73: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoDifferences); } CASE.with(|case| case.clone()) };
            let (mut v74, mut v75, mut v76, mut v77, mut v78): (i32, i32, i32, Rc<str>, Rc<UH1>) = left_compared_12(v0, v1.clone(), v52, v59, v24, v65.clone(), v66.clone(), v67, v6, v68, v69, v70, v71, v72.clone(), v73);
            let v79: i32 = 1i32;
            let v80: i32 = 0i32;
            let v81: i32 = 0i32;
            let (mut v82, mut v83): (i32, i32) = right_unseen_15(v1, v53, v65, v24, v66, v79, v80, v81);
            let v84: i32 = 0i32;
            let v85: i32 = differences_printed_16(v78, v84);
            let v86: bool = &*v77 != "";
            if v86 {
                let v87: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("key\tcolumn\tleft\tright\n"); } LIT.with(|lit| lit.clone()) }, v77));
                println!("{}", v87);
                ()
            };
            let v88: Rc<str> = std::rc::Rc::<str>::from((v74).to_string().as_str());
            let v89: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table diff ok left_rows="); } LIT.with(|lit| lit.clone()) }, v88));
            let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" right_rows="); } LIT.with(|lit| lit.clone()) }));
            let v91: Rc<str> = std::rc::Rc::<str>::from((v82).to_string().as_str());
            let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
            let v93: Rc<str> = Rc::<str>::from(format!("{}{}", v92, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" only_left="); } LIT.with(|lit| lit.clone()) }));
            let v94: Rc<str> = std::rc::Rc::<str>::from((v75).to_string().as_str());
            let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v94, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" only_right="); } LIT.with(|lit| lit.clone()) }));
            let v96: Rc<str> = std::rc::Rc::<str>::from((v83).to_string().as_str());
            let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v96));
            let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v97, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" differing_cells="); } LIT.with(|lit| lit.clone()) }));
            let v99: Rc<str> = std::rc::Rc::<str>::from((v85).to_string().as_str());
            let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v99));
            let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v100));
            let v103: Rc<str> = if v24 {
                let v102: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" latest_by="); } LIT.with(|lit| lit.clone()) }, v8));
                v102
            } else {
                v72
            };
            let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v103));
            println!("{}", v104);
            0i32
        }
    }
}
fn diff_files_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool) -> i32 {
    let v10: Rc<str> = kit_table_text::kit_table_problem(&*v0);
    let v11: bool = &*v10 != "";
    let v15: US0 = if v11 {
        US0::US0_TableUnreadable(v10)
    } else {
        let v13: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        US0::US0_TableLoaded(v13)
    };
    let v16: Rc<str> = kit_table_text::kit_table_problem(&*v1);
    let v17: bool = &*v16 != "";
    let v21: US0 = if v17 {
        US0::US0_TableUnreadable(v16)
    } else {
        let v19: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v1).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        US0::US0_TableLoaded(v19)
    };
    match &v15 {
        US0::US0_TableUnreadable(v22) => {
            let v22: Rc<str> = v22.clone();
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v22));
            eprintln!("{}", v23);
            2i32
        }
        _ => {
            match &v21 {
                US0::US0_TableUnreadable(v24) => {
                    let v24: Rc<str> = v24.clone();
                    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v24));
                    eprintln!("{}", v25);
                    2i32
                }
                _ => {
                    match v15 {
                        US0::US0_TableLoaded(v26) => {
                            let v26: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v26;
                            match v21 {
                                US0::US0_TableLoaded(v27) => {
                                    let v27: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v27;
                                    diff_tables_2(v26, v27, v2, v3, v4, v5, v6, v7, v8, v9)
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
fn no_request_17() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0, false)
}
fn table_diff_run_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64) -> i32 {
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_17();
    diff_files_1(v0, v1, v6, v3, v8, v2, v5, v11, v4, v13)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64| -> i32 {
        table_diff_run_0(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_diff_run(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: u64) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), v5)
}
