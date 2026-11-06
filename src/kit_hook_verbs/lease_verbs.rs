#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_LongVerb(Rc<str>),
    US1_ShortVerb(Rc<str>),
    US1_Redirect(i32),
    US1_GitVerb(Rc<str>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(US1),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(u8),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
pub struct Mut0 { pub l0: Rc<str> }
fn method3(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn char_at_5(mut v0: Rc<str>, mut v1: i32) -> US2 {
    let mut v2: bool = v1 < 0i32;
    let v5: bool = if v2 {
        true
    } else {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 >= v3;
        v4
    };
    if v5 {
        US2::US2_None
    } else {
        let v7: u8 = v0.as_bytes()[v1 as usize];
        US2::US2_Some(v7)
    }
}
fn word_char_7(mut v0: u8) -> bool {
    let mut v1: bool = v0 >= b'a';
    let v3: bool = if v1 {
        let mut v2: bool = v0 <= b'z';
        v2
    } else {
        false
    };
    let v7: bool = if v3 {
        true
    } else {
        let v4: bool = v0 >= b'A';
        if v4 {
            let v5: bool = v0 <= b'Z';
            v5
        } else {
            false
        }
    };
    let mut v11: bool = if v7 {
        true
    } else {
        let v8: bool = v0 >= b'0';
        if v8 {
            let v9: bool = v0 <= b'9';
            v9
        } else {
            false
        }
    };
    if v11 {
        true
    } else {
        let mut v12: bool = v0 == b'_';
        v12
    }
}
fn path_char_6(mut v0: u8) -> bool {
    let mut v1: bool = word_char_7(v0);
    let v3: bool = if v1 {
        true
    } else {
        let mut v2: bool = v0 == b'.';
        v2
    };
    let v5: bool = if v3 {
        true
    } else {
        let v4: bool = v0 == b'/';
        v4
    };
    let v7: bool = if v5 {
        true
    } else {
        let v6: bool = v0 == b'\\';
        v6
    };
    if v7 {
        true
    } else {
        let v8: bool = v0 == b'-';
        v8
    }
}
fn apart_4(mut v0: Rc<str>, mut v1: i32) -> bool {
    let mut v2: US2 = char_at_5(v0, v1);
    match v2 {
        US2::US2_None => {
            true
        }
        US2::US2_Some(v3) => {
            let v3: u8 = v3;
            let v4: bool = path_char_6(v3);
            let v5: bool = v4 == false;
            v5
        }
    }
}
fn short_at_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: i32 = v3.wrapping_sub(v2);
        let v5: usize = ((v2) as usize);
        let v6: i32 = v2.wrapping_add(v4);
        let v7: usize = ((v6) as usize);
        let v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
        let v9: Option<usize> = v8.find(&*v1);
        let v10: Option<usize> = method3(v9);
        let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v12: Option<i32> = v10.map(|x| v11(x));
        let v14: i32 = v12.unwrap_or(-1i32);
        let v15: bool = v14 < 0i32;
        if v15 {
            return false;
        } else {
            let v16: i32 = v2.wrapping_add(v14);
            let v17: i32 = v16.wrapping_sub(1i32);
            let v18: bool = apart_4(v0.clone(), v17);
            let v22: bool = if v18 {
                let v19: i32 = (v1.clone().len() as i32);
                let v20: i32 = v16.wrapping_add(v19);
                apart_4(v0.clone(), v20)
            } else {
                false
            };
            if v22 {
                return true;
            } else {
                let v23: i32 = v16.wrapping_add(1i32);
                v2 = v23;
                continue;
            }
        }
    }
}
fn redirect_at_9(mut v0: Rc<str>, mut v1: i32) -> bool {
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: US2 = char_at_5(v0.clone(), v2);
    let v4: i32 = v1.wrapping_add(1i32);
    let v5: US2 = char_at_5(v0.clone(), v4);
    let v15: bool = match v3 {
        US2::US2_None => {
            true
        }
        US2::US2_Some(v6) => {
            let v6: u8 = v6;
            let v7: bool = v6 != b'<';
            let v9: bool = if v7 {
                let v8: bool = v6 != b'>';
                v8
            } else {
                false
            };
            let mut v11: bool = if v9 {
                let v10: bool = v6 != b'=';
                v10
            } else {
                false
            };
            if v11 {
                let mut v12: bool = v6 != b'-';
                v12
            } else {
                false
            }
        }
    };
    let v16: US2 = char_at_5(v0, v1);
    let v19: bool = match v16 {
        US2::US2_Some(v17) => {
            let v17: u8 = v17;
            let v18: bool = v17 == b'>';
            v18
        }
        _ => {
            false
        }
    };
    let v20: bool = v19 && v15;
    if v20 {
        let v23: bool = match v5 {
            US2::US2_Some(v21) => {
                let v21: u8 = v21;
                let v22: bool = v21 == b'=';
                v22
            }
            _ => {
                false
            }
        };
        let v24: bool = v23 != true;
        v24
    } else {
        false
    }
}
fn redirect_from_8(mut v0: Rc<str>, mut v1: i32) -> US3 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return US3::US3_None;
        } else {
            let v5: bool = redirect_at_9(v0.clone(), v1);
            if v5 {
                return US3::US3_Some(v1);
            } else {
                let v7: i32 = v1.wrapping_add(1i32);
                v1 = v7;
                continue;
            }
        }
    }
}
fn spaces_end_11(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: US2 = char_at_5(v0.clone(), v1);
        match v2 {
            US2::US2_Some(v3) => {
                let v3: u8 = v3;
                let v4: bool = v3 == b' ';
                let v6: bool = if v4 {
                    true
                } else {
                    let v5: bool = v3 == b'\t';
                    v5
                };
                let v8: bool = if v6 {
                    true
                } else {
                    let v7: bool = v3 == b'\n';
                    v7
                };
                let v10: bool = if v8 {
                    true
                } else {
                    let v9: bool = v3 == b'\r';
                    v9
                };
                if v10 {
                    let mut v11: i32 = v1.wrapping_add(1i32);
                    v1 = v11;
                    continue;
                } else {
                    return v1;
                }
            }
            _ => {
                return v1;
            }
        }
    }
}
fn git_from_10(mut v0: Rc<str>, mut v1: i32) -> US4 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let v3: i32 = v2.wrapping_sub(v1);
        let v4: usize = ((v1) as usize);
        let v5: i32 = v1.wrapping_add(v3);
        let v6: usize = ((v5) as usize);
        let v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
        let v9: Option<usize> = v7.find(&*v8);
        let v10: Option<usize> = method3(v9);
        let mut v11: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v12: Option<i32> = v10.map(|x| v11(x));
        let v14: i32 = v12.unwrap_or(-1i32);
        let v15: bool = v14 < 0i32;
        if v15 {
            return US4::US4_None;
        } else {
            let v17: i32 = v1.wrapping_add(v14);
            let v18: i32 = v17.wrapping_add(3i32);
            let v19: i32 = spaces_end_11(v0.clone(), v18);
            let v20: i32 = v2.wrapping_sub(v19);
            let v21: usize = ((v19) as usize);
            let v22: i32 = v19.wrapping_add(v20);
            let v23: usize = ((v22) as usize);
            let v24: Rc<str> = Rc::<str>::from(&(v0)[v21..v23]);
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("clean"); } LIT.with(|lit| lit.clone()) };
            let v26: bool = v24.starts_with(&*v25);
            let v30: Rc<UH0> = if v26 {
                let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                Rc::new(UH0::UH0_Cons(v25, v27))
            } else {
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
            };
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stash"); } LIT.with(|lit| lit.clone()) };
            let v32: bool = v24.starts_with(&*v31);
            let v34: Rc<UH0> = if v32 {
                Rc::new(UH0::UH0_Cons(v31, v30))
            } else {
                v30
            };
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
            let v36: bool = v24.starts_with(&*v35);
            let v38: Rc<UH0> = if v36 {
                Rc::new(UH0::UH0_Cons(v35, v34))
            } else {
                v34
            };
            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reset"); } LIT.with(|lit| lit.clone()) };
            let v40: bool = v24.starts_with(&*v39);
            let v42: Rc<UH0> = if v40 {
                Rc::new(UH0::UH0_Cons(v39, v38))
            } else {
                v38
            };
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("restore"); } LIT.with(|lit| lit.clone()) };
            let v44: bool = v24.starts_with(&*v43);
            let v46: Rc<UH0> = if v44 {
                Rc::new(UH0::UH0_Cons(v43, v42))
            } else {
                v42
            };
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("checkout"); } LIT.with(|lit| lit.clone()) };
            let v48: bool = v24.starts_with(&*v47);
            let v50: Rc<UH0> = if v48 {
                Rc::new(UH0::UH0_Cons(v47, v46))
            } else {
                v46
            };
            match Rc::unwrap_or_clone(v50) {
                UH0::UH0_Cons(v51, v52) => {
                    let v51: Rc<str> = v51;
                    let v52: Rc<UH0> = v52;
                    let v53: bool = v19 > v18;
                    if v53 {
                        return US4::US4_Some(v51);
                    } else {
                        let v55: i32 = v18.wrapping_sub(2i32);
                        v1 = v55;
                        continue;
                    }
                }
                _ => {
                    let v58: i32 = v18.wrapping_sub(2i32);
                    v1 = v58;
                    continue;
                }
            }
        }
    }
}
fn write_sign_of_1(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("xcopy"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.contains(&*v2);
    let v7: Rc<UH0> = if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v2, v4))
    } else {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("robocopy"); } LIT.with(|lit| lit.clone()) };
    let v9: bool = v1.contains(&*v8);
    let mut v11: Rc<UH0> = if v9 {
        Rc::new(UH0::UH0_Cons(v8, v7))
    } else {
        v7
    };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("clear-content"); } LIT.with(|lit| lit.clone()) };
    let v13: bool = v1.contains(&*v12);
    let v15: Rc<UH0> = if v13 {
        Rc::new(UH0::UH0_Cons(v12, v11))
    } else {
        v11
    };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("new-item"); } LIT.with(|lit| lit.clone()) };
    let v17: bool = v1.contains(&*v16);
    let v19: Rc<UH0> = if v17 {
        Rc::new(UH0::UH0_Cons(v16, v15))
    } else {
        v15
    };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename-item"); } LIT.with(|lit| lit.clone()) };
    let v21: bool = v1.contains(&*v20);
    let v23: Rc<UH0> = if v21 {
        Rc::new(UH0::UH0_Cons(v20, v19))
    } else {
        v19
    };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("remove-item"); } LIT.with(|lit| lit.clone()) };
    let v25: bool = v1.contains(&*v24);
    let v27: Rc<UH0> = if v25 {
        Rc::new(UH0::UH0_Cons(v24, v23))
    } else {
        v23
    };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move-item"); } LIT.with(|lit| lit.clone()) };
    let v29: bool = v1.contains(&*v28);
    let v31: Rc<UH0> = if v29 {
        Rc::new(UH0::UH0_Cons(v28, v27))
    } else {
        v27
    };
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy-item"); } LIT.with(|lit| lit.clone()) };
    let v33: bool = v1.contains(&*v32);
    let v35: Rc<UH0> = if v33 {
        Rc::new(UH0::UH0_Cons(v32, v31))
    } else {
        v31
    };
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writeallbytes"); } LIT.with(|lit| lit.clone()) };
    let v37: bool = v1.contains(&*v36);
    let v39: Rc<UH0> = if v37 {
        Rc::new(UH0::UH0_Cons(v36, v35))
    } else {
        v35
    };
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writealllines"); } LIT.with(|lit| lit.clone()) };
    let v41: bool = v1.contains(&*v40);
    let v43: Rc<UH0> = if v41 {
        Rc::new(UH0::UH0_Cons(v40, v39))
    } else {
        v39
    };
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writealltext"); } LIT.with(|lit| lit.clone()) };
    let v45: bool = v1.contains(&*v44);
    let v47: Rc<UH0> = if v45 {
        Rc::new(UH0::UH0_Cons(v44, v43))
    } else {
        v43
    };
    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("out-file"); } LIT.with(|lit| lit.clone()) };
    let v49: bool = v1.contains(&*v48);
    let v51: Rc<UH0> = if v49 {
        Rc::new(UH0::UH0_Cons(v48, v47))
    } else {
        v47
    };
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add-content"); } LIT.with(|lit| lit.clone()) };
    let v53: bool = v1.contains(&*v52);
    let v55: Rc<UH0> = if v53 {
        Rc::new(UH0::UH0_Cons(v52, v51))
    } else {
        v51
    };
    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set-content"); } LIT.with(|lit| lit.clone()) };
    let v57: bool = v1.contains(&*v56);
    let v59: Rc<UH0> = if v57 {
        Rc::new(UH0::UH0_Cons(v56, v55))
    } else {
        v55
    };
    match Rc::unwrap_or_clone(v59) {
        UH0::UH0_Cons(v60, v61) => {
            let v60: Rc<str> = v60;
            let v61: Rc<UH0> = v61;
            let v62: US1 = US1::US1_LongVerb(v60);
            US0::US0_Some(v62)
        }
        UH0::UH0_Nil => {
            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tee"); } LIT.with(|lit| lit.clone()) };
            let v65: i32 = 0i32;
            let v66: bool = short_at_2(v1.clone(), v64.clone(), v65);
            let v70: Rc<UH0> = if v66 {
                let v67: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                Rc::new(UH0::UH0_Cons(v64, v67))
            } else {
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
            };
            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rm"); } LIT.with(|lit| lit.clone()) };
            let v72: i32 = 0i32;
            let v73: bool = short_at_2(v1.clone(), v71.clone(), v72);
            let v75: Rc<UH0> = if v73 {
                Rc::new(UH0::UH0_Cons(v71, v70))
            } else {
                v70
            };
            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mv"); } LIT.with(|lit| lit.clone()) };
            let v77: i32 = 0i32;
            let v78: bool = short_at_2(v1.clone(), v76.clone(), v77);
            let v80: Rc<UH0> = if v78 {
                Rc::new(UH0::UH0_Cons(v76, v75))
            } else {
                v75
            };
            let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cp"); } LIT.with(|lit| lit.clone()) };
            let v82: i32 = 0i32;
            let v83: bool = short_at_2(v1.clone(), v81.clone(), v82);
            let v85: Rc<UH0> = if v83 {
                Rc::new(UH0::UH0_Cons(v81, v80))
            } else {
                v80
            };
            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mkdir"); } LIT.with(|lit| lit.clone()) };
            let v87: i32 = 0i32;
            let v88: bool = short_at_2(v1.clone(), v86.clone(), v87);
            let v90: Rc<UH0> = if v88 {
                Rc::new(UH0::UH0_Cons(v86, v85))
            } else {
                v85
            };
            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rmdir"); } LIT.with(|lit| lit.clone()) };
            let v92: i32 = 0i32;
            let v93: bool = short_at_2(v1.clone(), v91.clone(), v92);
            let v95: Rc<UH0> = if v93 {
                Rc::new(UH0::UH0_Cons(v91, v90))
            } else {
                v90
            };
            let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rd"); } LIT.with(|lit| lit.clone()) };
            let v97: i32 = 0i32;
            let v98: bool = short_at_2(v1.clone(), v96.clone(), v97);
            let v100: Rc<UH0> = if v98 {
                Rc::new(UH0::UH0_Cons(v96, v95))
            } else {
                v95
            };
            let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("del"); } LIT.with(|lit| lit.clone()) };
            let v102: i32 = 0i32;
            let v103: bool = short_at_2(v1.clone(), v101.clone(), v102);
            let v105: Rc<UH0> = if v103 {
                Rc::new(UH0::UH0_Cons(v101, v100))
            } else {
                v100
            };
            let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move"); } LIT.with(|lit| lit.clone()) };
            let v107: i32 = 0i32;
            let v108: bool = short_at_2(v1.clone(), v106.clone(), v107);
            let v110: Rc<UH0> = if v108 {
                Rc::new(UH0::UH0_Cons(v106, v105))
            } else {
                v105
            };
            let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy"); } LIT.with(|lit| lit.clone()) };
            let v112: i32 = 0i32;
            let v113: bool = short_at_2(v1.clone(), v111.clone(), v112);
            let v115: Rc<UH0> = if v113 {
                Rc::new(UH0::UH0_Cons(v111, v110))
            } else {
                v110
            };
            match Rc::unwrap_or_clone(v115) {
                UH0::UH0_Cons(v116, v117) => {
                    let v116: Rc<str> = v116;
                    let v117: Rc<UH0> = v117;
                    let v118: US1 = US1::US1_ShortVerb(v116);
                    US0::US0_Some(v118)
                }
                UH0::UH0_Nil => {
                    let v120: i32 = 0i32;
                    let v121: US3 = redirect_from_8(v1.clone(), v120);
                    match v121 {
                        US3::US3_None => {
                            let v125: i32 = 0i32;
                            let v126: US4 = git_from_10(v1, v125);
                            match v126 {
                                US4::US4_None => {
                                    US0::US0_None
                                }
                                US4::US4_Some(v127) => {
                                    let v127: Rc<str> = v127;
                                    let v128: US1 = US1::US1_GitVerb(v127);
                                    US0::US0_Some(v128)
                                }
                            }
                        }
                        US3::US3_Some(v122) => {
                            let v122: i32 = v122;
                            let v123: US1 = US1::US1_Redirect(v122);
                            US0::US0_Some(v123)
                        }
                    }
                }
            }
        }
    }
}
fn method13(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_12(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method13(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn sign_line_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = write_sign_of_1(v0);
    match v1 {
        US0::US0_None => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writes=no"); } LIT.with(|lit| lit.clone()) };
            v16
        }
        US0::US0_Some(v2) => {
            let mut v2: US1 = v2;
            match v2 {
                US1::US1_GitVerb(v10) => {
                    let v10: Rc<str> = v10;
                    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writes=yes sign=git:"); } LIT.with(|lit| lit.clone()) }, v10));
                    v11
                }
                US1::US1_LongVerb(v3) => {
                    let v3: Rc<str> = v3;
                    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writes=yes sign=verb:"); } LIT.with(|lit| lit.clone()) }, v3));
                    v4
                }
                US1::US1_Redirect(v7) => {
                    let v7: i32 = v7;
                    let v8: Rc<str> = format_real_12(v7);
                    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writes=yes sign=redirect@"); } LIT.with(|lit| lit.clone()) }, v8));
                    v9
                }
                US1::US1_ShortVerb(v5) => {
                    let v5: Rc<str> = v5;
                    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writes=yes sign=short:"); } LIT.with(|lit| lit.clone()) }, v5));
                    v6
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        sign_line_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_hook_sign_line(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
