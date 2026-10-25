#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_GetContent,
    US0_SelectString,
    US0_GetChildItem,
    US0_SetContent,
    US0_AddContent,
    US0_OutFile,
    US0_ForEachObject,
    US0_MeasureObject,
    US0_CopyItem,
    US0_RemoveItem,
    US0_MoveItem,
    US0_TestPath,
    US0_StartSleep,
    US0_Robocopy,
    US0_Findstr,
    US0_Python,
    US0_Python3,
    US0_PyExe,
    US0_IoFile,
}
fn method3(mut v0: Option<char>) -> Option<char> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((char)) -> u32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> u32> = Rc::new(move |mut v0: (char)| -> u32 {
        let mut v1: char = (v0);
        let mut v2: u32 = u32::from(v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn bounded_at_2(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> bool {
    loop {
        let mut v2: i32 = (v0.clone().borrow().len() as i32);
        let mut v3: i32 = v1.wrapping_add(1i32);
        let mut v4: bool = v2 <= v3;
        if v4 {
            return false;
        } else {
            let mut v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: bool = v6 > 0i32;
            let mut v29: bool = if v7 {
                let mut v8: i32 = v6.wrapping_sub(1i32);
                let mut v9: usize = ((v8) as usize);
                let mut v10: Option<char> = (&*v5)[v9..].chars().next();
                let mut v11: Option<char> = method3(v10.clone());
                let mut v12: Rc<dyn Fn((char)) -> u32> = closure1();
                let mut v13: Option<u32> = v11.map(|x| v12(x));
                let mut v15: u32 = v13.unwrap_or(0u32);
                let mut v16: bool = 48u32 <= v15;
                let mut v18: bool = if v16 {
                    let mut v17: bool = v15 <= 57u32;
                    v17
                } else {
                    false
                };
                let mut v22: bool = if v18 {
                    true
                } else {
                    let mut v19: bool = 97u32 <= v15;
                    if v19 {
                        let mut v20: bool = v15 <= 122u32;
                        v20
                    } else {
                        false
                    }
                };
                let mut v26: bool = if v22 {
                    true
                } else {
                    let mut v23: bool = 65u32 <= v15;
                    if v23 {
                        let mut v24: bool = v15 <= 90u32;
                        v24
                    } else {
                        false
                    }
                };
                if v26 {
                    true
                } else {
                    let mut v27: bool = v15 == 95u32;
                    v27
                }
            } else {
                false
            };
            let mut v30: bool = v29 == false;
            let mut v56: bool = if v30 {
                let mut v31: Rc<str> = v0.clone().borrow()[v3 as usize].clone();
                let mut v32: i32 = (v31.clone().len() as i32);
                let mut v33: bool = v32 > 0i32;
                let mut v54: bool = if v33 {
                    let mut v34: usize = ((0i32) as usize);
                    let mut v35: Option<char> = (&*v31)[v34..].chars().next();
                    let mut v36: Option<char> = method3(v35.clone());
                    let mut v37: Rc<dyn Fn((char)) -> u32> = closure1();
                    let mut v38: Option<u32> = v36.map(|x| v37(x));
                    let mut v40: u32 = v38.unwrap_or(0u32);
                    let mut v41: bool = 48u32 <= v40;
                    let mut v43: bool = if v41 {
                        let mut v42: bool = v40 <= 57u32;
                        v42
                    } else {
                        false
                    };
                    let mut v47: bool = if v43 {
                        true
                    } else {
                        let mut v44: bool = 97u32 <= v40;
                        if v44 {
                            let mut v45: bool = v40 <= 122u32;
                            v45
                        } else {
                            false
                        }
                    };
                    let mut v51: bool = if v47 {
                        true
                    } else {
                        let mut v48: bool = 65u32 <= v40;
                        if v48 {
                            let mut v49: bool = v40 <= 90u32;
                            v49
                        } else {
                            false
                        }
                    };
                    if v51 {
                        true
                    } else {
                        let mut v52: bool = v40 == 95u32;
                        v52
                    }
                } else {
                    false
                };
                let mut v55: bool = v54 == false;
                v55
            } else {
                false
            };
            if v56 {
                return true;
            } else {
                (v0, v1) = (v0.clone(), v3);
                continue;
            }
        }
    }
}
fn habit_in_1(mut v0: Rc<str>, mut v1: US0) -> bool {
    let mut v2: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    match &v1 {
        US0::US0_IoFile => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[io.file]"); } LIT.with(|lit| lit.clone()) };
            let mut v4: bool = v2.contains(&*v3);
            v4
        }
        _ => {
            let mut v40: Rc<str> = match &v1 {
                US0::US0_AddContent => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add-content"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                }
                US0::US0_CopyItem => {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy-item"); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                }
                US0::US0_Findstr => {
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("findstr"); } LIT.with(|lit| lit.clone()) };
                    v19.clone()
                }
                US0::US0_ForEachObject => {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("foreach-object"); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                }
                US0::US0_GetChildItem => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("get-childitem"); } LIT.with(|lit| lit.clone()) };
                    v7.clone()
                }
                US0::US0_GetContent => {
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("get-content"); } LIT.with(|lit| lit.clone()) };
                    v5.clone()
                }
                US0::US0_MeasureObject => {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("measure-object"); } LIT.with(|lit| lit.clone()) };
                    v12.clone()
                }
                US0::US0_MoveItem => {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move-item"); } LIT.with(|lit| lit.clone()) };
                    v15.clone()
                }
                US0::US0_OutFile => {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("out-file"); } LIT.with(|lit| lit.clone()) };
                    v10.clone()
                }
                US0::US0_PyExe => {
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("py.exe"); } LIT.with(|lit| lit.clone()) };
                    v22.clone()
                }
                US0::US0_Python => {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python"); } LIT.with(|lit| lit.clone()) };
                    v20.clone()
                }
                US0::US0_Python3 => {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python3"); } LIT.with(|lit| lit.clone()) };
                    v21.clone()
                }
                US0::US0_RemoveItem => {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("remove-item"); } LIT.with(|lit| lit.clone()) };
                    v14.clone()
                }
                US0::US0_Robocopy => {
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("robocopy"); } LIT.with(|lit| lit.clone()) };
                    v18.clone()
                }
                US0::US0_SelectString => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("select-string"); } LIT.with(|lit| lit.clone()) };
                    v6.clone()
                }
                US0::US0_SetContent => {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set-content"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                }
                US0::US0_StartSleep => {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start-sleep"); } LIT.with(|lit| lit.clone()) };
                    v17.clone()
                }
                US0::US0_TestPath => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-path"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                }
                _ => unreachable!(),
            };
            let mut v41: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v40).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let mut v42: i32 = 0i32;
            bounded_at_2(v41.clone(), v42)
        }
    }
}
fn habit_any_0(mut v0: Rc<str>) -> bool {
    let mut v1: US0 = US0::US0_GetContent;
    let mut v2: bool = habit_in_1(v0.clone(), v1.clone());
    let mut v5: bool = if v2 {
        true
    } else {
        let mut v3: US0 = US0::US0_SelectString;
        habit_in_1(v0.clone(), v3.clone())
    };
    let mut v8: bool = if v5 {
        true
    } else {
        let mut v6: US0 = US0::US0_GetChildItem;
        habit_in_1(v0.clone(), v6.clone())
    };
    let mut v11: bool = if v8 {
        true
    } else {
        let mut v9: US0 = US0::US0_SetContent;
        habit_in_1(v0.clone(), v9.clone())
    };
    let mut v14: bool = if v11 {
        true
    } else {
        let mut v12: US0 = US0::US0_AddContent;
        habit_in_1(v0.clone(), v12.clone())
    };
    let mut v17: bool = if v14 {
        true
    } else {
        let mut v15: US0 = US0::US0_OutFile;
        habit_in_1(v0.clone(), v15.clone())
    };
    let mut v20: bool = if v17 {
        true
    } else {
        let mut v18: US0 = US0::US0_ForEachObject;
        habit_in_1(v0.clone(), v18.clone())
    };
    let mut v23: bool = if v20 {
        true
    } else {
        let mut v21: US0 = US0::US0_MeasureObject;
        habit_in_1(v0.clone(), v21.clone())
    };
    let mut v26: bool = if v23 {
        true
    } else {
        let mut v24: US0 = US0::US0_CopyItem;
        habit_in_1(v0.clone(), v24.clone())
    };
    let mut v29: bool = if v26 {
        true
    } else {
        let mut v27: US0 = US0::US0_RemoveItem;
        habit_in_1(v0.clone(), v27.clone())
    };
    let mut v32: bool = if v29 {
        true
    } else {
        let mut v30: US0 = US0::US0_MoveItem;
        habit_in_1(v0.clone(), v30.clone())
    };
    let mut v35: bool = if v32 {
        true
    } else {
        let mut v33: US0 = US0::US0_TestPath;
        habit_in_1(v0.clone(), v33.clone())
    };
    let mut v38: bool = if v35 {
        true
    } else {
        let mut v36: US0 = US0::US0_StartSleep;
        habit_in_1(v0.clone(), v36.clone())
    };
    let mut v41: bool = if v38 {
        true
    } else {
        let mut v39: US0 = US0::US0_Robocopy;
        habit_in_1(v0.clone(), v39.clone())
    };
    let mut v44: bool = if v41 {
        true
    } else {
        let mut v42: US0 = US0::US0_Findstr;
        habit_in_1(v0.clone(), v42.clone())
    };
    let mut v47: bool = if v44 {
        true
    } else {
        let mut v45: US0 = US0::US0_Python;
        habit_in_1(v0.clone(), v45.clone())
    };
    let mut v50: bool = if v47 {
        true
    } else {
        let mut v48: US0 = US0::US0_Python3;
        habit_in_1(v0.clone(), v48.clone())
    };
    let mut v53: bool = if v50 {
        true
    } else {
        let mut v51: US0 = US0::US0_PyExe;
        habit_in_1(v0.clone(), v51.clone())
    };
    if v53 {
        true
    } else {
        let mut v54: US0 = US0::US0_IoFile;
        habit_in_1(v0.clone(), v54.clone())
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        habit_any_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn habit_words_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = US0::US0_GetContent;
    let mut v2: bool = habit_in_1(v0.clone(), v1.clone());
    let mut v9: Rc<str> = if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("get-content"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("get-content"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("get-content\n"); } LIT.with(|lit| lit.clone()) };
        v7.clone()
    } else {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v8.clone()
    };
    let mut v10: US0 = US0::US0_SelectString;
    let mut v11: bool = habit_in_1(v0.clone(), v10.clone());
    let mut v16: Rc<str> = if v11 {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("select-string"); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v12));
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
        v15.clone()
    } else {
        v9.clone()
    };
    let mut v17: US0 = US0::US0_GetChildItem;
    let mut v18: bool = habit_in_1(v0.clone(), v17.clone());
    let mut v23: Rc<str> = if v18 {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("get-childitem"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v19));
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        v22.clone()
    } else {
        v16.clone()
    };
    let mut v24: US0 = US0::US0_SetContent;
    let mut v25: bool = habit_in_1(v0.clone(), v24.clone());
    let mut v30: Rc<str> = if v25 {
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set-content"); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v26));
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
        v29.clone()
    } else {
        v23.clone()
    };
    let mut v31: US0 = US0::US0_AddContent;
    let mut v32: bool = habit_in_1(v0.clone(), v31.clone());
    let mut v37: Rc<str> = if v32 {
        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add-content"); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v33));
        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
        v36.clone()
    } else {
        v30.clone()
    };
    let mut v38: US0 = US0::US0_OutFile;
    let mut v39: bool = habit_in_1(v0.clone(), v38.clone());
    let mut v44: Rc<str> = if v39 {
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("out-file"); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v40));
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
        v43.clone()
    } else {
        v37.clone()
    };
    let mut v45: US0 = US0::US0_ForEachObject;
    let mut v46: bool = habit_in_1(v0.clone(), v45.clone());
    let mut v51: Rc<str> = if v46 {
        let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("foreach-object"); } LIT.with(|lit| lit.clone()) };
        let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v47));
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
        v50.clone()
    } else {
        v44.clone()
    };
    let mut v52: US0 = US0::US0_MeasureObject;
    let mut v53: bool = habit_in_1(v0.clone(), v52.clone());
    let mut v58: Rc<str> = if v53 {
        let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("measure-object"); } LIT.with(|lit| lit.clone()) };
        let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v54));
        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
        v57.clone()
    } else {
        v51.clone()
    };
    let mut v59: US0 = US0::US0_CopyItem;
    let mut v60: bool = habit_in_1(v0.clone(), v59.clone());
    let mut v65: Rc<str> = if v60 {
        let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy-item"); } LIT.with(|lit| lit.clone()) };
        let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v61));
        let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v63));
        v64.clone()
    } else {
        v58.clone()
    };
    let mut v66: US0 = US0::US0_RemoveItem;
    let mut v67: bool = habit_in_1(v0.clone(), v66.clone());
    let mut v72: Rc<str> = if v67 {
        let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("remove-item"); } LIT.with(|lit| lit.clone()) };
        let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v68));
        let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
        v71.clone()
    } else {
        v65.clone()
    };
    let mut v73: US0 = US0::US0_MoveItem;
    let mut v74: bool = habit_in_1(v0.clone(), v73.clone());
    let mut v79: Rc<str> = if v74 {
        let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move-item"); } LIT.with(|lit| lit.clone()) };
        let mut v76: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v75));
        let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
        v78.clone()
    } else {
        v72.clone()
    };
    let mut v80: US0 = US0::US0_TestPath;
    let mut v81: bool = habit_in_1(v0.clone(), v80.clone());
    let mut v86: Rc<str> = if v81 {
        let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-path"); } LIT.with(|lit| lit.clone()) };
        let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v82));
        let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v84));
        v85.clone()
    } else {
        v79.clone()
    };
    let mut v87: US0 = US0::US0_StartSleep;
    let mut v88: bool = habit_in_1(v0.clone(), v87.clone());
    let mut v93: Rc<str> = if v88 {
        let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start-sleep"); } LIT.with(|lit| lit.clone()) };
        let mut v90: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v89));
        let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
        v92.clone()
    } else {
        v86.clone()
    };
    let mut v94: US0 = US0::US0_Robocopy;
    let mut v95: bool = habit_in_1(v0.clone(), v94.clone());
    let mut v100: Rc<str> = if v95 {
        let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("robocopy"); } LIT.with(|lit| lit.clone()) };
        let mut v97: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v96));
        let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v98));
        v99.clone()
    } else {
        v93.clone()
    };
    let mut v101: US0 = US0::US0_Findstr;
    let mut v102: bool = habit_in_1(v0.clone(), v101.clone());
    let mut v107: Rc<str> = if v102 {
        let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("findstr"); } LIT.with(|lit| lit.clone()) };
        let mut v104: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v103));
        let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
        v106.clone()
    } else {
        v100.clone()
    };
    let mut v108: US0 = US0::US0_Python;
    let mut v109: bool = habit_in_1(v0.clone(), v108.clone());
    let mut v114: Rc<str> = if v109 {
        let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python"); } LIT.with(|lit| lit.clone()) };
        let mut v111: Rc<str> = Rc::<str>::from(format!("{}{}", v107, v110));
        let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v113: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v112));
        v113.clone()
    } else {
        v107.clone()
    };
    let mut v115: US0 = US0::US0_Python3;
    let mut v116: bool = habit_in_1(v0.clone(), v115.clone());
    let mut v121: Rc<str> = if v116 {
        let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python3"); } LIT.with(|lit| lit.clone()) };
        let mut v118: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v117));
        let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v120: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v119));
        v120.clone()
    } else {
        v114.clone()
    };
    let mut v122: US0 = US0::US0_PyExe;
    let mut v123: bool = habit_in_1(v0.clone(), v122.clone());
    let mut v128: Rc<str> = if v123 {
        let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("py.exe"); } LIT.with(|lit| lit.clone()) };
        let mut v125: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v124));
        let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v127: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v126));
        v127.clone()
    } else {
        v121.clone()
    };
    let mut v129: US0 = US0::US0_IoFile;
    let mut v130: bool = habit_in_1(v0.clone(), v129.clone());
    if v130 {
        let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[io.file]"); } LIT.with(|lit| lit.clone()) };
        let mut v132: Rc<str> = Rc::<str>::from(format!("{}{}", v128, v131));
        let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v134: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v133));
        v134.clone()
    } else {
        v128.clone()
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        habit_words_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_adoption_habit_any(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_adoption_habit_words(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
