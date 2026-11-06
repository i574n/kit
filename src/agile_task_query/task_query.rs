#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_OnTeam(Rc<str>),
    US1_WithTag(Rc<str>),
    US1_WithPrefix(Rc<str>),
    US1_WithWords(Rc<str>),
    US1_OnlyOpen,
    US1_OnlyReady,
    US1_OnlyBlocked,
}
#[derive(Clone)]
enum US0 {
    US0_FilterRead(US1, i32),
    US0_FlagRefused(Rc<str>),
    US0_NotQueryFlag,
}
#[derive(Clone)]
enum US2 {
    US2_ValueRead(Rc<str>),
    US2_ValueRefused(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_NoFilters,
    UH0_FilterThen(US1, Rc<UH0>),
}
fn method2(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn flag_read_of_1(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let v2: bool = &*v0 == "--team";
    let v37: US0 = if v2 {
        let mut v3: bool = &*v1 != "";
        let v6: bool = if v3 {
            let mut v4: Rc<str> = Rc::<str>::from(v1.to_lowercase());
            let v5: bool = &*v4 == &*v1;
            v5
        } else {
            false
        };
        let v15: bool = if v6 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v8: Option<usize> = v1.find(&*v7);
            let v9: Option<usize> = method2(v8);
            let mut v10: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v11: Option<i32> = v9.map(|x| v10(x));
            let v13: i32 = v11.unwrap_or(-1i32);
            let v14: bool = v13 < 0i32;
            v14
        } else {
            false
        };
        let v24: bool = if v15 {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v17: Option<usize> = v1.find(&*v16);
            let v18: Option<usize> = method2(v17);
            let mut v19: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v20: Option<i32> = v18.map(|x| v19(x));
            let v22: i32 = v20.unwrap_or(-1i32);
            let v23: bool = v22 < 0i32;
            v23
        } else {
            false
        };
        let v28: US2 = if v24 {
            US2::US2_ValueRead(v1.clone())
        } else {
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team expects a lower-case team word, got "); } LIT.with(|lit| lit.clone()) }, v1.clone()));
            US2::US2_ValueRefused(v26)
        };
        match v28 {
            US2::US2_ValueRead(v29) => {
                let v29: Rc<str> = v29;
                let v30: US1 = US1::US1_OnTeam(v29);
                US0::US0_FilterRead(v30, 2i32)
            }
            US2::US2_ValueRefused(v32) => {
                let v32: Rc<str> = v32;
                US0::US0_FlagRefused(v32)
            }
        }
    } else {
        US0::US0_NotQueryFlag
    };
    let v52: US0 = match &v37 {
        US0::US0_NotQueryFlag => {
            let v38: bool = &*v0 == "--tag";
            if v38 {
                let v39: bool = &*v1 == "";
                let v43: US2 = if v39 {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tag expects a tag"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ValueRefused(v40)
                } else {
                    US2::US2_ValueRead(v1.clone())
                };
                match v43 {
                    US2::US2_ValueRead(v44) => {
                        let v44: Rc<str> = v44;
                        let v45: US1 = US1::US1_WithTag(v44);
                        US0::US0_FilterRead(v45, 2i32)
                    }
                    US2::US2_ValueRefused(v47) => {
                        let v47: Rc<str> = v47;
                        US0::US0_FlagRefused(v47)
                    }
                }
            } else {
                v37
            }
        }
        _ => {
            v37
        }
    };
    let v67: US0 = match &v52 {
        US0::US0_NotQueryFlag => {
            let v53: bool = &*v0 == "--prefix";
            if v53 {
                let v54: bool = &*v1 == "";
                let v58: US2 = if v54 {
                    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prefix expects an ID prefix"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ValueRefused(v55)
                } else {
                    US2::US2_ValueRead(v1.clone())
                };
                match v58 {
                    US2::US2_ValueRead(v59) => {
                        let v59: Rc<str> = v59;
                        let v60: US1 = US1::US1_WithPrefix(v59);
                        US0::US0_FilterRead(v60, 2i32)
                    }
                    US2::US2_ValueRefused(v62) => {
                        let v62: Rc<str> = v62;
                        US0::US0_FlagRefused(v62)
                    }
                }
            } else {
                v52
            }
        }
        _ => {
            v52
        }
    };
    let v82: US0 = match &v67 {
        US0::US0_NotQueryFlag => {
            let v68: bool = &*v0 == "--words";
            if v68 {
                let v69: bool = &*v1 == "";
                let v73: US2 = if v69 {
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--words expects comma-separated words"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ValueRefused(v70)
                } else {
                    US2::US2_ValueRead(v1)
                };
                match v73 {
                    US2::US2_ValueRead(v74) => {
                        let v74: Rc<str> = v74;
                        let v75: US1 = US1::US1_WithWords(v74);
                        US0::US0_FilterRead(v75, 2i32)
                    }
                    US2::US2_ValueRefused(v77) => {
                        let v77: Rc<str> = v77;
                        US0::US0_FlagRefused(v77)
                    }
                }
            } else {
                v67
            }
        }
        _ => {
            v67
        }
    };
    let v87: US0 = match &v82 {
        US0::US0_NotQueryFlag => {
            let v83: bool = &*v0 == "--open";
            if v83 {
                let v84: US1 = US1::US1_OnlyOpen;
                US0::US0_FilterRead(v84, 1i32)
            } else {
                v82
            }
        }
        _ => {
            v82
        }
    };
    let v92: US0 = match &v87 {
        US0::US0_NotQueryFlag => {
            let v88: bool = &*v0 == "--ready";
            if v88 {
                let v89: US1 = US1::US1_OnlyReady;
                US0::US0_FilterRead(v89, 1i32)
            } else {
                v87
            }
        }
        _ => {
            v87
        }
    };
    match &v92 {
        US0::US0_NotQueryFlag => {
            let v93: bool = &*v0 == "--blocked";
            if v93 {
                let v94: US1 = US1::US1_OnlyBlocked;
                US0::US0_FilterRead(v94, 1i32)
            } else {
                v92
            }
        }
        _ => {
            v92
        }
    }
}
fn flag_width_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: US0 = flag_read_of_1(v0, v1);
    match v2 {
        US0::US0_FilterRead(v3, v4) => {
            let mut v3: US1 = v3;
            let mut v4: i32 = v4;
            v4
        }
        US0::US0_FlagRefused(v5) => {
            let v5: Rc<str> = v5;
            -1i32
        }
        US0::US0_NotQueryFlag => {
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        flag_width_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn flag_refusal_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: US0 = flag_read_of_1(v0, v1);
    match v2 {
        US0::US0_FlagRefused(v3) => {
            let mut v3: Rc<str> = v3;
            v3
        }
        _ => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        flag_refusal_3(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tag_listed_8(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }, v1));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }, v6));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let v9: Option<usize> = v8.find(&*v3);
    let mut v10: Option<usize> = method2(v9);
    let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v12: Option<i32> = v10.map(|x| v11(x));
    let v14: i32 = v12.unwrap_or(-1i32);
    let v15: bool = v14 >= 0i32;
    v15
}
fn on_team_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = tag_listed_8(v2.clone(), v0.clone());
    let v6: bool = if v3 {
        true
    } else {
        let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) }, v0.clone()));
        tag_listed_8(v2.clone(), v4)
    };
    let v9: bool = if v6 {
        true
    } else {
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-team"); } LIT.with(|lit| lit.clone()) }));
        tag_listed_8(v2, v7)
    };
    if v9 {
        true
    } else {
        let mut v10: Rc<str> = Rc::<str>::from(v0.to_uppercase());
        let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
        let mut v12: bool = v1.starts_with(&*v11);
        v12
    }
}
fn loop_12(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v3 >= v2;
        if v4 {
            return false;
        } else {
            let v5: Rc<str> = v1.borrow()[v3 as usize].clone();
            let v6: bool = &*v5 == &*v0;
            if v6 {
                return true;
            } else {
                let v7: i32 = v3.wrapping_add(1i32);
                v3 = v7;
                continue;
            }
        }
    }
}
fn has_word_11(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v4));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v4));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v4));
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v4));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v13.replace(&*v14, &*v4));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v4));
    let v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v17.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v19: i32 = (v18.clone().borrow().len() as i32);
    let mut v20: i32 = 0i32;
    loop_12(v1, v18, v19, v20)
}
fn loop_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32, mut v4: i32) -> bool {
    loop {
        let v5: bool = v4 >= v3;
        if v5 {
            return false;
        } else {
            let v6: Rc<str> = v2.borrow()[v4 as usize].clone();
            let v7: bool = &*v6 != "";
            let mut v10: bool = if v7 {
                let v8: bool = has_word_11(v0.clone(), v6.clone());
                let v9: bool = v8 == false;
                v9
            } else {
                false
            };
            let v13: bool = if v10 {
                let mut v11: bool = has_word_11(v1.clone(), v6);
                let mut v12: bool = v11 == false;
                v12
            } else {
                false
            };
            if v13 {
                return true;
            } else {
                let v14: i32 = v4.wrapping_add(1i32);
                v4 = v14;
                continue;
            }
        }
    }
}
fn words_hit_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v6: i32 = (v5.clone().borrow().len() as i32);
    let v7: i32 = 0i32;
    let v8: bool = loop_10(v1, v2, &v5, v6, v7);
    let v9: bool = v8 == false;
    v9
}
fn filter_passes_6(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool) -> bool {
    match v0 {
        US1::US1_OnTeam(v6) => {
            let v6: Rc<str> = v6;
            on_team_7(v6, v1, v3)
        }
        US1::US1_OnlyBlocked => {
            let v21: bool = &*v4 == "Blocked";
            if v21 {
                true
            } else {
                let v22: bool = &*v4 == "Planned";
                let v24: bool = if v22 {
                    true
                } else {
                    let v23: bool = &*v4 == "Active";
                    v23
                };
                if v24 {
                    let v25: bool = v5 == false;
                    v25
                } else {
                    false
                }
            }
        }
        US1::US1_OnlyOpen => {
            let v14: bool = &*v4 == "Planned";
            if v14 {
                true
            } else {
                let v15: bool = &*v4 == "Active";
                v15
            }
        }
        US1::US1_OnlyReady => {
            let v17: bool = &*v4 == "Planned";
            let mut v19: bool = if v17 {
                true
            } else {
                let v18: bool = &*v4 == "Active";
                v18
            };
            let mut v20: bool = v19 && v5;
            v20
        }
        US1::US1_WithPrefix(v10) => {
            let mut v10: Rc<str> = v10;
            let mut v11: bool = v1.starts_with(&*v10);
            v11
        }
        US1::US1_WithTag(v8) => {
            let v8: Rc<str> = v8;
            tag_listed_8(v3, v8)
        }
        US1::US1_WithWords(v12) => {
            let mut v12: Rc<str> = v12;
            words_hit_9(v12, v2, v3)
        }
    }
}
fn filters_pass_5(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_FilterThen(v6, v7) => {
                let v6: US1 = v6;
                let v7: Rc<UH0> = v7;
                let v8: bool = filter_passes_6(v6, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5);
                if v8 {
                    v0 = v7;
                    continue;
                } else {
                    return false;
                }
            }
            UH0::UH0_NoFilters => {
                return true;
            }
        }
    }
}
fn query_token_14(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).split_whitespace().nth(v1 as usize).unwrap_or(""));
    v2
}
fn filters_of_query_13(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: Rc<str> = query_token_14(v0.clone(), v1);
        let mut v4: bool = &*v3 == "";
        if v4 {
            return v2;
        } else {
            let v5: i32 = v1.wrapping_add(1i32);
            let v6: Rc<str> = query_token_14(v0.clone(), v5);
            let v7: US0 = flag_read_of_1(v3, v6);
            match v7 {
                US0::US0_FilterRead(v8, v9) => {
                    let v8: US1 = v8;
                    let v9: i32 = v9;
                    let mut v10: i32 = v1.wrapping_add(v9);
                    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_FilterThen(v8, v2));
                    (v1, v2) = (v10, v11);
                    continue;
                }
                _ => {
                    v1 = v5;
                    continue;
                }
            }
        }
    }
}
fn query_passes_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool) -> bool {
    let v6: i32 = 0i32;
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = filters_of_query_13(v0, v6, v7);
    filters_pass_5(v8, v1, v2, v3, v4, v5)
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool| -> bool {
        query_passes_4(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn filters_read_deps_16(mut v0: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_FilterThen(v1, v2) => {
                let mut v1: US1 = v1;
                let v2: Rc<UH0> = v2;
                let mut v4: bool = match v1 {
                    US1::US1_OnlyBlocked => {
                        true
                    }
                    US1::US1_OnlyReady => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v4 {
                    return true;
                } else {
                    v0 = v2;
                    continue;
                }
            }
            UH0::UH0_NoFilters => {
                return false;
            }
        }
    }
}
fn query_reads_deps_15(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = 0i32;
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = filters_of_query_13(v0, v1, v2);
    filters_read_deps_16(v3)
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        query_reads_deps_15(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn filters_text_18(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_FilterThen(v2, v3) => {
                let v2: US1 = v2;
                let mut v3: Rc<UH0> = v3;
                let v21: Rc<str> = match v2 {
                    US1::US1_OnTeam(v4) => {
                        let mut v4: Rc<str> = v4;
                        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team="); } LIT.with(|lit| lit.clone()) }, v4));
                        v5
                    }
                    US1::US1_OnlyBlocked => {
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
                        v14
                    }
                    US1::US1_OnlyOpen => {
                        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open"); } LIT.with(|lit| lit.clone()) };
                        v12
                    }
                    US1::US1_OnlyReady => {
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ready"); } LIT.with(|lit| lit.clone()) };
                        v13
                    }
                    US1::US1_WithPrefix(v8) => {
                        let v8: Rc<str> = v8;
                        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prefix="); } LIT.with(|lit| lit.clone()) }, v8));
                        v9
                    }
                    US1::US1_WithTag(v6) => {
                        let v6: Rc<str> = v6;
                        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tag="); } LIT.with(|lit| lit.clone()) }, v6));
                        v7
                    }
                    US1::US1_WithWords(v10) => {
                        let mut v10: Rc<str> = v10;
                        let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("words="); } LIT.with(|lit| lit.clone()) }, v10));
                        v11
                    }
                };
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }, v21));
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v1));
                (v0, v1) = (v3, v23);
                continue;
            }
            UH0::UH0_NoFilters => {
                return v1;
            }
        }
    }
}
fn query_shown_of_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = filters_of_query_13(v0, v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    filters_text_18(v3, v4)
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        query_shown_of_17(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>, --tag <tag>, --prefix <ID prefix>, --words <w1,w2>, --open, --ready, --blocked"); } LIT.with(|lit| lit.clone()) };
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_query_flag_width(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_query_flag_refusal(v0: &str, v1: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_query_passes(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: bool) -> bool {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), v5)
}
pub fn kit_query_reads_deps(v0: &str) -> bool {
    closure4()(Rc::<str>::from(v0))
}
pub fn kit_query_shown(v0: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0))
}
pub fn kit_query_usage() -> Rc<str> {
    closure6()()
}
