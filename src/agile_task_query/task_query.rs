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
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn flag_read_of_1(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team"); } LIT.with(|lit| lit.clone()) };
    let mut v38: US0 = if v2 {
        let mut v3: bool = v1.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = if v3 {
            let mut v4: Rc<str> = Rc::<str>::from(v1.to_lowercase());
            let mut v5: bool = v4.clone() == v1.clone();
            v5
        } else {
            false
        };
        let mut v15: bool = if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Option<usize> = v1.find(&*v7);
            let mut v9: Option<usize> = method2(v8.clone());
            let mut v10: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v11: Option<i32> = v9.map(|x| v10(x));
            let mut v13: i32 = v11.unwrap_or(-1i32);
            let mut v14: bool = v13 < 0i32;
            v14
        } else {
            false
        };
        let mut v24: bool = if v15 {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v17: Option<usize> = v1.find(&*v16);
            let mut v18: Option<usize> = method2(v17.clone());
            let mut v19: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v20: Option<i32> = v18.map(|x| v19(x));
            let mut v22: i32 = v20.unwrap_or(-1i32);
            let mut v23: bool = v22 < 0i32;
            v23
        } else {
            false
        };
        let mut v29: US2 = if v24 {
            US2::US2_ValueRead(v1.clone())
        } else {
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team expects a lower-case team word, got "); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v1));
            US2::US2_ValueRefused(v27.clone())
        };
        match &v29 {
            US2::US2_ValueRead(v30) => {
                let mut v30: Rc<str> = v30.clone();
                let mut v31: US1 = US1::US1_OnTeam(v30.clone());
                US0::US0_FilterRead(v31.clone(), 2i32)
            }
            US2::US2_ValueRefused(v33) => {
                let mut v33: Rc<str> = v33.clone();
                US0::US0_FlagRefused(v33.clone())
            }
        }
    } else {
        US0::US0_NotQueryFlag
    };
    let mut v53: US0 = match &v38 {
        US0::US0_NotQueryFlag => {
            let mut v39: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tag"); } LIT.with(|lit| lit.clone()) };
            if v39 {
                let mut v40: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v44: US2 = if v40 {
                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tag expects a tag"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ValueRefused(v41.clone())
                } else {
                    US2::US2_ValueRead(v1.clone())
                };
                match &v44 {
                    US2::US2_ValueRead(v45) => {
                        let mut v45: Rc<str> = v45.clone();
                        let mut v46: US1 = US1::US1_WithTag(v45.clone());
                        US0::US0_FilterRead(v46.clone(), 2i32)
                    }
                    US2::US2_ValueRefused(v48) => {
                        let mut v48: Rc<str> = v48.clone();
                        US0::US0_FlagRefused(v48.clone())
                    }
                }
            } else {
                v38.clone()
            }
        }
        _ => {
            v38.clone()
        }
    };
    let mut v68: US0 = match &v53 {
        US0::US0_NotQueryFlag => {
            let mut v54: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prefix"); } LIT.with(|lit| lit.clone()) };
            if v54 {
                let mut v55: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v59: US2 = if v55 {
                    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prefix expects an ID prefix"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ValueRefused(v56.clone())
                } else {
                    US2::US2_ValueRead(v1.clone())
                };
                match &v59 {
                    US2::US2_ValueRead(v60) => {
                        let mut v60: Rc<str> = v60.clone();
                        let mut v61: US1 = US1::US1_WithPrefix(v60.clone());
                        US0::US0_FilterRead(v61.clone(), 2i32)
                    }
                    US2::US2_ValueRefused(v63) => {
                        let mut v63: Rc<str> = v63.clone();
                        US0::US0_FlagRefused(v63.clone())
                    }
                }
            } else {
                v53.clone()
            }
        }
        _ => {
            v53.clone()
        }
    };
    let mut v83: US0 = match &v68 {
        US0::US0_NotQueryFlag => {
            let mut v69: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--words"); } LIT.with(|lit| lit.clone()) };
            if v69 {
                let mut v70: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v74: US2 = if v70 {
                    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--words expects comma-separated words"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ValueRefused(v71.clone())
                } else {
                    US2::US2_ValueRead(v1.clone())
                };
                match &v74 {
                    US2::US2_ValueRead(v75) => {
                        let mut v75: Rc<str> = v75.clone();
                        let mut v76: US1 = US1::US1_WithWords(v75.clone());
                        US0::US0_FilterRead(v76.clone(), 2i32)
                    }
                    US2::US2_ValueRefused(v78) => {
                        let mut v78: Rc<str> = v78.clone();
                        US0::US0_FlagRefused(v78.clone())
                    }
                }
            } else {
                v68.clone()
            }
        }
        _ => {
            v68.clone()
        }
    };
    let mut v88: US0 = match &v83 {
        US0::US0_NotQueryFlag => {
            let mut v84: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--open"); } LIT.with(|lit| lit.clone()) };
            if v84 {
                let mut v85: US1 = US1::US1_OnlyOpen;
                US0::US0_FilterRead(v85.clone(), 1i32)
            } else {
                v83.clone()
            }
        }
        _ => {
            v83.clone()
        }
    };
    let mut v93: US0 = match &v88 {
        US0::US0_NotQueryFlag => {
            let mut v89: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ready"); } LIT.with(|lit| lit.clone()) };
            if v89 {
                let mut v90: US1 = US1::US1_OnlyReady;
                US0::US0_FilterRead(v90.clone(), 1i32)
            } else {
                v88.clone()
            }
        }
        _ => {
            v88.clone()
        }
    };
    match &v93 {
        US0::US0_NotQueryFlag => {
            let mut v94: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--blocked"); } LIT.with(|lit| lit.clone()) };
            if v94 {
                let mut v95: US1 = US1::US1_OnlyBlocked;
                US0::US0_FilterRead(v95.clone(), 1i32)
            } else {
                v93.clone()
            }
        }
        _ => {
            v93.clone()
        }
    }
}
fn flag_width_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: US0 = flag_read_of_1(v0.clone(), v1.clone());
    match &v2 {
        US0::US0_FilterRead(v3, v4) => {
            let mut v3: US1 = v3.clone();
            let mut v4: i32 = *v4;
            v4
        }
        US0::US0_FlagRefused(v5) => {
            let mut v5: Rc<str> = v5.clone();
            -1i32
        }
        US0::US0_NotQueryFlag => {
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        flag_width_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn flag_refusal_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US0 = flag_read_of_1(v0.clone(), v1.clone());
    match &v2 {
        US0::US0_FlagRefused(v3) => {
            let mut v3: Rc<str> = v3.clone();
            v3.clone()
        }
        _ => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        flag_refusal_3(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tag_listed_8(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(v0.replace(&*v5, &*v6));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v7));
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v2));
    let mut v10: Option<usize> = v9.find(&*v4);
    let mut v11: Option<usize> = method2(v10.clone());
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let mut v15: i32 = v13.unwrap_or(-1i32);
    let mut v16: bool = v15 >= 0i32;
    v16
}
fn on_team_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = tag_listed_8(v2.clone(), v0.clone());
    if v3 {
        true
    } else {
        let mut v4: Rc<str> = Rc::<str>::from(v0.to_uppercase());
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
        let mut v7: bool = v1.starts_with(&*v6);
        v7
    }
}
fn loop_12(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v3 >= v2;
        if v4 {
            return false;
        } else {
            let mut v5: Rc<str> = v1.clone().borrow()[v3 as usize].clone();
            let mut v6: bool = v5.clone() == v0.clone();
            if v6 {
                return true;
            } else {
                let mut v7: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2, v7);
                continue;
            }
        }
    }
}
fn has_word_11(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v4));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v4));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v4));
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v4));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = Rc::<str>::from(v13.replace(&*v14, &*v4));
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v4));
    let mut v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v17.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v19: i32 = (v18.clone().borrow().len() as i32);
    let mut v20: i32 = 0i32;
    loop_12(v1.clone(), v18.clone(), v19, v20)
}
fn loop_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32, mut v4: i32) -> bool {
    loop {
        let mut v5: bool = v4 >= v3;
        if v5 {
            return false;
        } else {
            let mut v6: Rc<str> = v2.clone().borrow()[v4 as usize].clone();
            let mut v7: bool = v6.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v10: bool = if v7 {
                let mut v8: bool = has_word_11(v0.clone(), v6.clone());
                let mut v9: bool = v8 == false;
                v9
            } else {
                false
            };
            let mut v13: bool = if v10 {
                let mut v11: bool = has_word_11(v1.clone(), v6.clone());
                let mut v12: bool = v11 == false;
                v12
            } else {
                false
            };
            if v13 {
                return true;
            } else {
                let mut v14: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v3, v14);
                continue;
            }
        }
    }
}
fn words_hit_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = (v5.clone().borrow().len() as i32);
    let mut v7: i32 = 0i32;
    let mut v8: bool = loop_10(v1.clone(), v2.clone(), v5.clone(), v6, v7);
    let mut v9: bool = v8 == false;
    v9
}
fn filter_passes_6(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool) -> bool {
    match &v0 {
        US1::US1_OnTeam(v6) => {
            let mut v6: Rc<str> = v6.clone();
            on_team_7(v6.clone(), v1.clone(), v3.clone())
        }
        US1::US1_OnlyBlocked => {
            let mut v21: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
            if v21 {
                true
            } else {
                let mut v22: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
                let mut v24: bool = if v22 {
                    true
                } else {
                    let mut v23: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
                    v23
                };
                if v24 {
                    let mut v25: bool = v5 == false;
                    v25
                } else {
                    false
                }
            }
        }
        US1::US1_OnlyOpen => {
            let mut v14: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
            if v14 {
                true
            } else {
                let mut v15: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
                v15
            }
        }
        US1::US1_OnlyReady => {
            let mut v17: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
            let mut v19: bool = if v17 {
                true
            } else {
                let mut v18: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
                v18
            };
            let mut v20: bool = v19 && v5;
            v20
        }
        US1::US1_WithPrefix(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: bool = v1.starts_with(&*v10);
            v11
        }
        US1::US1_WithTag(v8) => {
            let mut v8: Rc<str> = v8.clone();
            tag_listed_8(v3.clone(), v8.clone())
        }
        US1::US1_WithWords(v12) => {
            let mut v12: Rc<str> = v12.clone();
            words_hit_9(v12.clone(), v2.clone(), v3.clone())
        }
    }
}
fn filters_pass_5(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v6, v7) => {
                let mut v6: US1 = v6.clone();
                let mut v7: Rc<UH0> = v7.clone();
                let mut v8: bool = filter_passes_6(v6.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5);
                if v8 {
                    (v0, v1, v2, v3, v4, v5) = (v7.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5);
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
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).split_whitespace().nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn filters_of_query_13(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: Rc<str> = query_token_14(v0.clone(), v1);
        let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            return v2.clone();
        } else {
            let mut v5: i32 = v1.wrapping_add(1i32);
            let mut v6: Rc<str> = query_token_14(v0.clone(), v5);
            let mut v7: US0 = flag_read_of_1(v3.clone(), v6.clone());
            match &v7 {
                US0::US0_FilterRead(v8, v9) => {
                    let mut v8: US1 = v8.clone();
                    let mut v9: i32 = *v9;
                    let mut v10: i32 = v1.wrapping_add(v9);
                    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_FilterThen(v8.clone(), v2.clone()));
                    (v0, v1, v2) = (v0.clone(), v10, v11.clone());
                    continue;
                }
                _ => {
                    (v0, v1, v2) = (v0.clone(), v5, v2.clone());
                    continue;
                }
            }
        }
    }
}
fn query_passes_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool) -> bool {
    let mut v6: i32 = 0i32;
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = filters_of_query_13(v0.clone(), v6, v7.clone());
    filters_pass_5(v8.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5)
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool| -> bool {
        query_passes_4(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn filters_read_deps_16(mut v0: Rc<UH0>) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v1, v2) => {
                let mut v1: US1 = v1.clone();
                let mut v2: Rc<UH0> = v2.clone();
                let mut v4: bool = match &v1 {
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
                    v0 = v2.clone();
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
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = filters_of_query_13(v0.clone(), v1, v2.clone());
    filters_read_deps_16(v3.clone())
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        query_reads_deps_15(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn filters_text_18(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v2, v3) => {
                let mut v2: US1 = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v25: Rc<str> = match &v2 {
                    US1::US1_OnTeam(v4) => {
                        let mut v4: Rc<str> = v4.clone();
                        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team="); } LIT.with(|lit| lit.clone()) };
                        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
                        v6.clone()
                    }
                    US1::US1_OnlyBlocked => {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    }
                    US1::US1_OnlyOpen => {
                        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open"); } LIT.with(|lit| lit.clone()) };
                        v16.clone()
                    }
                    US1::US1_OnlyReady => {
                        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ready"); } LIT.with(|lit| lit.clone()) };
                        v17.clone()
                    }
                    US1::US1_WithPrefix(v10) => {
                        let mut v10: Rc<str> = v10.clone();
                        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prefix="); } LIT.with(|lit| lit.clone()) };
                        let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10));
                        v12.clone()
                    }
                    US1::US1_WithTag(v7) => {
                        let mut v7: Rc<str> = v7.clone();
                        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tag="); } LIT.with(|lit| lit.clone()) };
                        let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
                        v9.clone()
                    }
                    US1::US1_WithWords(v13) => {
                        let mut v13: Rc<str> = v13.clone();
                        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("words="); } LIT.with(|lit| lit.clone()) };
                        let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
                        v15.clone()
                    }
                };
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v25));
                let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v1));
                (v0, v1) = (v3.clone(), v28.clone());
                continue;
            }
            UH0::UH0_NoFilters => {
                return v1.clone();
            }
        }
    }
}
fn query_shown_of_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = filters_of_query_13(v0.clone(), v1, v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    filters_text_18(v3.clone(), v4.clone())
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        query_shown_of_17(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>"); } LIT.with(|lit| lit.clone()) };
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>, "); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tag <tag>"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>, --tag <tag>"); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prefix <ID prefix>"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>, --tag <tag>, "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prefix <ID prefix>"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>, --tag <tag>, --prefix <ID prefix>"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        };
        let mut v11: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--words <w1,w2>"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        } else {
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v1));
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--words <w1,w2>"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            v15.clone()
        };
        let mut v17: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = if v17 {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--open"); } LIT.with(|lit| lit.clone()) };
            v18.clone()
        } else {
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v1));
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--open"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
            v21.clone()
        };
        let mut v23: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = if v23 {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ready"); } LIT.with(|lit| lit.clone()) };
            v24.clone()
        } else {
            let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v1));
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ready"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            v27.clone()
        };
        let mut v29: bool = v28.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v29 {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--blocked"); } LIT.with(|lit| lit.clone()) };
            v30.clone()
        } else {
            let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v1));
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--blocked"); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
            v33.clone()
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_query_flag_width(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_query_flag_refusal(v0: &str, v1: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_query_passes(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: bool) -> bool {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), v5)
}
pub fn eoie_query_reads_deps(v0: &str) -> bool {
    closure4()(Rc::<str>::from(v0))
}
pub fn eoie_query_shown(v0: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0))
}
pub fn eoie_query_usage() -> Rc<str> {
    closure6()()
}
