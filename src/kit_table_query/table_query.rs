#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_TableQuery,
    US1_TableDiff,
    US1_TableUnknown,
}
#[derive(Clone)]
enum US2 {
    US2_StepTaken(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool),
    US2_StepRejected(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
fn rows_kept_2(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, Rc<str>) {
    let (mut v2, mut v3): (bool, Rc<str>) = kit_table_filter::kit_table_filter_kept(&*v0, &*v1);
    (v2, v3)
}
fn measured_query_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let (mut v9, mut v10): (bool, Rc<str>) = rows_kept_2(v0.clone(), v1);
    let v11: i32 = kit_table_measures::kit_table_measured(&*v0, &*v3, &*v6, v5, v8, v9, &*v10);
    v11
}
fn plain_query_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let v9: i32 = kit_table_plain::kit_table_plain(&*v0, &*v1, &*v2, &*v3, v5, v8);
    v9
}
fn command_line_argument_count_5() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_6(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_4(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_5();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US0 = if v5 {
        let v6: Rc<str> = command_line_argument_6(v1);
        US0::US0_Some(v6)
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10
        }
    }
}
fn argument_count_7() -> i32 {
    let v0: i32 = command_line_argument_count_5();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn no_request_8() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0, false)
}
fn each_prefixed_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).map(|item| format!("{}{}", &*v0, item.trim())).collect::<Vec<_>>().join(",").as_str());
    v2.clone()
}
fn method13(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_12(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method13(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn option_applied_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>) -> US2 {
    let v10: bool = &*v8 == "--where";
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
        let v12: bool = v9.contains(&*v11);
        let v15: bool = if v12 {
            true
        } else {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
            let v14: bool = v9.contains(&*v13);
            v14
        };
        let v18: bool = if v15 {
            true
        } else {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
            let v17: bool = v9.contains(&*v16);
            v17
        };
        if v18 {
            let v19: bool = &*v0 == "";
            let v23: Rc<str> = if v19 {
                v9.clone()
            } else {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v20, &*v9, &*v21, &*v21].concat());
                v22.clone()
            };
            US2::US2_StepTaken(v23.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
        } else {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where expects col=value (or col!=value, col>value, col>=value, col<value, col<=value), got "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v9, &*v26, &*v26, &*v26].concat());
            US2::US2_StepRejected(v27.clone())
        }
    } else {
        let v30: bool = &*v8 == "--cols";
        if v30 {
            US2::US2_StepTaken(v0.clone(), v9.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
        } else {
            let v32: bool = &*v8 == "--group";
            if v32 {
                US2::US2_StepTaken(v0.clone(), v1.clone(), v9.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
            } else {
                let v34: bool = &*v8 == "--key";
                if v34 {
                    US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v9.clone(), v4, v5.clone(), v6.clone(), v7)
                } else {
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mean|--sd|--median|--sum|--min|--max|--count"); } LIT.with(|lit| lit.clone()) };
                    let v37: bool = v36.split("|").any(|item| item == &*v8);
                    if v37 {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<str> = std::rc::Rc::<str>::from((v8).trim_start_matches(&*v38));
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v40, &*v41, &*v41, &*v41].concat());
                        let v43: Rc<str> = each_prefixed_11(v42.clone(), v9.clone());
                        let v44: bool = &*v5 == "";
                        let v49: Rc<str> = if v44 {
                            v43.clone()
                        } else {
                            let v45: bool = &*v43 == "";
                            if v45 {
                                v5.clone()
                            } else {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                let v47: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v46, &*v43, &*v41, &*v41].concat());
                                v47.clone()
                            }
                        };
                        US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v49.clone(), v6.clone(), v7)
                    } else {
                        let v51: bool = &*v8 == "--latest-by";
                        if v51 {
                            US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v9.clone(), v7)
                        } else {
                            let v53: bool = &*v8 == "--limit";
                            if v53 {
                                let v54: u64 = decimal_u64_12(v9.clone());
                                let v55: bool = v54 == 0u64;
                                if v55 {
                                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit expects a positive integer"); } LIT.with(|lit| lit.clone()) };
                                    US2::US2_StepRejected(v56.clone())
                                } else {
                                    US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v54, v5.clone(), v6.clone(), v7)
                                }
                            } else {
                                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v62: Rc<str> = std::rc::Rc::<str>::from([&*v60, &*v8, &*v61, &*v61, &*v61].concat());
                                US2::US2_StepRejected(v62.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn options_from_9(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool) -> US2 {
    loop {
        let v10: bool = v0 < v1;
        if v10 {
            let v11: Rc<str> = argument_4(v0);
            let v12: bool = &*v11 == "--quiet";
            if v12 {
                let v13: i32 = v0.wrapping_add(1i32);
                let v14: bool = true;
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v13, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v14);
                continue;
            } else {
                let v16: i32 = v0.wrapping_add(1i32);
                let v17: bool = v16 < v1;
                if v17 {
                    let v18: Rc<str> = argument_4(v0);
                    let v19: Rc<str> = argument_4(v16);
                    let v20: US2 = option_applied_10(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9, v18.clone(), v19.clone());
                    match &v20 {
                        US2::US2_StepRejected(v31) => {
                            let v31: Rc<str> = v31.clone();
                            return US2::US2_StepRejected(v31.clone());
                        }
                        US2::US2_StepTaken(v21, v22, v23, v24, v25, v26, v27, v28) => {
                            let v21: Rc<str> = v21.clone();
                            let v22: Rc<str> = v22.clone();
                            let v23: Rc<str> = v23.clone();
                            let v24: Rc<str> = v24.clone();
                            let v25: u64 = *v25;
                            let v26: Rc<str> = v26.clone();
                            let v27: Rc<str> = v27.clone();
                            let v28: bool = *v28;
                            let v29: i32 = v0.wrapping_add(2i32);
                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v29, v1, v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25, v26.clone(), v27.clone(), v28);
                            continue;
                        }
                    }
                } else {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = argument_4(v0);
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v36, &*v37, &*v38, &*v38].concat());
                    return US2::US2_StepRejected(v39.clone());
                }
            }
        } else {
            return US2::US2_StepTaken(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9);
        }
    }
}
fn table_run_0() -> i32 {
    let v0: i32 = 2i32;
    let v1: Rc<str> = argument_4(v0);
    let v2: bool = &*v1 == "query";
    let v8: US1 = if v2 {
        US1::US1_TableQuery
    } else {
        let mut v4: bool = &*v1 == "diff";
        if v4 {
            US1::US1_TableDiff
        } else {
            US1::US1_TableUnknown
        }
    };
    match &v8 {
        US1::US1_TableDiff => {
            let v47: i32 = argument_count_7();
            let v48: bool = v47 < 5i32;
            if v48 {
                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", v51);
                2i32
            } else {
                let v52: i32 = 5i32;
                let v53: i32 = argument_count_7();
                let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_8();
                let v62: US2 = options_from_9(v52, v53, v54.clone(), v55.clone(), v56.clone(), v57.clone(), v58, v59.clone(), v60.clone(), v61);
                match &v62 {
                    US2::US2_StepRejected(v76) => {
                        let v76: Rc<str> = v76.clone();
                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                        let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v76));
                        eprintln!("{}", v78);
                        2i32
                    }
                    US2::US2_StepTaken(v63, v64, v65, v66, v67, v68, v69, v70) => {
                        let v63: Rc<str> = v63.clone();
                        let v64: Rc<str> = v64.clone();
                        let v65: Rc<str> = v65.clone();
                        let v66: Rc<str> = v66.clone();
                        let v67: u64 = *v67;
                        let v68: Rc<str> = v68.clone();
                        let v69: Rc<str> = v69.clone();
                        let v70: bool = *v70;
                        let v71: i32 = 3i32;
                        let v72: Rc<str> = argument_4(v71);
                        let v73: i32 = 4i32;
                        let v74: Rc<str> = argument_4(v73);
                        let v75: i32 = kit_table_diff::kit_table_diff_run(&*v72, &*v74, &*v66, &*v64, &*v69, v67);
                        v75
                    }
                }
            }
        }
        US1::US1_TableQuery => {
            let v9: i32 = argument_count_7();
            let v10: bool = v9 < 4i32;
            if v10 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", v13);
                2i32
            } else {
                let v14: i32 = 4i32;
                let v15: i32 = argument_count_7();
                let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_8();
                let v24: US2 = options_from_9(v14, v15, v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20, v21.clone(), v22.clone(), v23);
                match &v24 {
                    US2::US2_StepRejected(v41) => {
                        let v41: Rc<str> = v41.clone();
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v41));
                        eprintln!("{}", v43);
                        2i32
                    }
                    US2::US2_StepTaken(v25, v26, v27, v28, v29, v30, v31, v32) => {
                        let v25: Rc<str> = v25.clone();
                        let v26: Rc<str> = v26.clone();
                        let v27: Rc<str> = v27.clone();
                        let v28: Rc<str> = v28.clone();
                        let v29: u64 = *v29;
                        let v30: Rc<str> = v30.clone();
                        let v31: Rc<str> = v31.clone();
                        let v32: bool = *v32;
                        let v33: bool = &*v30 != "";
                        if v33 {
                            let v34: i32 = 3i32;
                            let v35: Rc<str> = argument_4(v34);
                            measured_query_1(v35.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone(), v29, v30.clone(), v31.clone(), v32)
                        } else {
                            let v37: i32 = 3i32;
                            let v38: Rc<str> = argument_4(v37);
                            plain_query_3(v38.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone(), v29, v30.clone(), v31.clone(), v32)
                        }
                    }
                }
            }
        }
        US1::US1_TableUnknown => {
            let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
            let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
            let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v84);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        table_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (bool, Rc<str>) {
        rows_kept_2(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_run() -> i32 {
    closure0()()
}
pub fn kit_table_rows_kept(v0: &str, v1: &str) -> (bool, Rc<str>) {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
