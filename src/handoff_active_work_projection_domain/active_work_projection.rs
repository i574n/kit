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
fn method3() -> Rc<str> {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_2() -> Rc<str> {
    method3()
}
fn codec_runtime_char_4(mut v0: Rc<str>) -> u8 {
    let v1: u8 = v0.as_bytes()[0i32 as usize];
    v1
}
fn find_char_loop_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
    loop {
        let v4: bool = v1 == v2 ;
        if v4 {
            return v2;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == v3 ;
            if v6 {
                return v1;
            } else {
                let v7: i32 = v1 + 1i32 ;
                (v0, v1, v2, v3) = (v0.clone(), v7, v2, v3);
                continue;
            }
        }
    }
}
fn starts_with_loop_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
    loop {
        let v5: bool = v3 == v4 ;
        if v5 {
            return true;
        } else {
            let v6: u8 = v0.clone().as_bytes()[v2 as usize];
            let v7: u8 = v1.clone().as_bytes()[v3 as usize];
            let v8: bool = v6 == v7 ;
            if v8 {
                let v9: i32 = v2 + 1i32 ;
                let v10: i32 = v3 + 1i32 ;
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v9, v10, v4);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn find_text_loop_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        let v5: i32 = v2 + v4 ;
        let v6: bool = v3 < v5;
        if v6 {
            return v3;
        } else {
            let v7: i32 = (v0.clone().len() as i32);
            let v8: i32 = (v1.clone().len() as i32);
            let v9: i32 = v2 + v8 ;
            let v10: bool = v7 < v9;
            let v13: bool = if v10 {
                false
            } else {
                let v11: i32 = 0i32;
                starts_with_loop_7(v0.clone(), v1.clone(), v2, v11, v8)
            };
            if v13 {
                return v2;
            } else {
                let v14: i32 = v2 + 1i32 ;
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v14, v3, v4);
                continue;
            }
        }
    }
}
fn parse_quoted_loop_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: bool) -> (Rc<str>, i32, bool) {
    loop {
        let v5: bool = v1 == v2 ;
        if v5 {
            return (v3.clone(), v1, false);
        } else {
            let v6: u8 = v0.clone().as_bytes()[v1 as usize];
            if v4 {
                let v7: bool = v6 == b'n' ;
                let v24: Rc<str> = if v7 {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                } else {
                    let v9: bool = v6 == b'r' ;
                    if v9 {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v10.clone()
                    } else {
                        let v11: bool = v6 == b't' ;
                        if v11 {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v12.clone()
                        } else {
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                            let v14: u8 = codec_runtime_char_4(v13.clone());
                            let v15: bool = v6 == v14 ;
                            if v15 {
                                v13.clone()
                            } else {
                                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                let v17: u8 = codec_runtime_char_4(v16.clone());
                                let v18: bool = v6 == v17 ;
                                if v18 {
                                    v16.clone()
                                } else {
                                    codec_empty_2()
                                }
                            }
                        }
                    }
                };
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: bool = v24 == v25 ;
                if v26 {
                    return (v3.clone(), v1, false);
                } else {
                    let v27: i32 = v1 + 1i32 ;
                    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v24.clone()));
                    let v29: bool = false;
                    (v0, v1, v2, v3, v4) = (v0.clone(), v27, v2, v28.clone(), v29);
                    continue;
                }
            } else {
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v37: u8 = codec_runtime_char_4(v36.clone());
                let v38: bool = v6 == v37 ;
                if v38 {
                    let v39: i32 = v1 + 1i32 ;
                    let v40: bool = true;
                    (v0, v1, v2, v3, v4) = (v0.clone(), v39, v2, v3.clone(), v40);
                    continue;
                } else {
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v45: u8 = codec_runtime_char_4(v44.clone());
                    let v46: bool = v6 == v45 ;
                    if v46 {
                        let v47: i32 = v1 + 1i32 ;
                        return (v3.clone(), v47, true);
                    } else {
                        let v48: i32 = v1 + 1i32 ;
                        let v49: Rc<str> = string_slice(&v0.clone(), v1 as i64, v1 as i64);
                        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v49.clone()));
                        let v51: bool = false;
                        (v0, v1, v2, v3, v4) = (v0.clone(), v48, v2, v50.clone(), v51);
                        continue;
                    }
                }
            }
        }
    }
}
fn projection_digit_start_loop_9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 == 0i32 ;
        if v2 {
            return 0i32;
        } else {
            let v3: i32 = v1 - 1i32 ;
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b'0' ;
            let v24: u64 = if v5 {
                0u64
            } else {
                let v6: bool = v4 == b'1' ;
                if v6 {
                    1u64
                } else {
                    let v7: bool = v4 == b'2' ;
                    if v7 {
                        2u64
                    } else {
                        let v8: bool = v4 == b'3' ;
                        if v8 {
                            3u64
                        } else {
                            let v9: bool = v4 == b'4' ;
                            if v9 {
                                4u64
                            } else {
                                let v10: bool = v4 == b'5' ;
                                if v10 {
                                    5u64
                                } else {
                                    let v11: bool = v4 == b'6' ;
                                    if v11 {
                                        6u64
                                    } else {
                                        let v12: bool = v4 == b'7' ;
                                        if v12 {
                                            7u64
                                        } else {
                                            let v13: bool = v4 == b'8' ;
                                            if v13 {
                                                8u64
                                            } else {
                                                let v14: bool = v4 == b'9' ;
                                                if v14 {
                                                    9u64
                                                } else {
                                                    10u64
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            let v25: bool = v24 < 10u64;
            if v25 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn codec_u64_decimal_valid_loop_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u64) -> bool {
    loop {
        let v4: bool = v1 == v2 ;
        if v4 {
            return true;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == b'0' ;
            let v25: u64 = if v6 {
                0u64
            } else {
                let v7: bool = v5 == b'1' ;
                if v7 {
                    1u64
                } else {
                    let v8: bool = v5 == b'2' ;
                    if v8 {
                        2u64
                    } else {
                        let v9: bool = v5 == b'3' ;
                        if v9 {
                            3u64
                        } else {
                            let v10: bool = v5 == b'4' ;
                            if v10 {
                                4u64
                            } else {
                                let v11: bool = v5 == b'5' ;
                                if v11 {
                                    5u64
                                } else {
                                    let v12: bool = v5 == b'6' ;
                                    if v12 {
                                        6u64
                                    } else {
                                        let v13: bool = v5 == b'7' ;
                                        if v13 {
                                            7u64
                                        } else {
                                            let v14: bool = v5 == b'8' ;
                                            if v14 {
                                                8u64
                                            } else {
                                                let v15: bool = v5 == b'9' ;
                                                if v15 {
                                                    9u64
                                                } else {
                                                    10u64
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            let v26: bool = v25 == 10u64 ;
            if v26 {
                return false;
            } else {
                let v27: bool = v3 > 1844674407370955161u64;
                if v27 {
                    return false;
                } else {
                    let v28: bool = v3 == 1844674407370955161u64 ;
                    if v28 {
                        let v29: bool = v25 > 5u64;
                        if v29 {
                            return false;
                        } else {
                            let v30: i32 = v1 + 1i32 ;
                            let v31: u64 = v3 * 10u64 ;
                            let v32: u64 = v31 + v25 ;
                            (v0, v1, v2, v3) = (v0.clone(), v30, v2, v32);
                            continue;
                        }
                    } else {
                        let v35: i32 = v1 + 1i32 ;
                        let v36: u64 = v3 * 10u64 ;
                        let v37: u64 = v36 + v25 ;
                        (v0, v1, v2, v3) = (v0.clone(), v35, v2, v37);
                        continue;
                    }
                }
            }
        }
    }
}
fn codec_u64_decimal_normalize_10(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: bool = v1 == 0i32 ;
    if v2 {
        codec_empty_2()
    } else {
        let v4: i32 = 0i32;
        let v5: u64 = 0u64;
        let v6: bool = codec_u64_decimal_valid_loop_11(v0.clone(), v4, v1, v5);
        if v6 {
            v0
        } else {
            codec_empty_2()
        }
    }
}
fn handoff_active_work_projection_loop_1(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: bool = v1 == v2 ;
        if v4 {
            return v3.clone();
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v6: u8 = codec_runtime_char_4(v5.clone());
            let v7: i32 = (v0.clone().len() as i32);
            let v8: i32 = find_char_loop_5(v0.clone(), v1, v7, v6);
            let v9: bool = v1 == v8 ;
            let v13: Rc<str> = if v9 {
                codec_empty_2()
            } else {
                let v11: i32 = v8 - 1i32 ;
                let v12: Rc<str> = string_slice(&v0.clone(), v1 as i64, v11 as i64);
                v12.clone()
            };
            let v14: i32 = (v13.clone().len() as i32);
            let v15: bool = v8 == v2 ;
            let v17: i32 = if v15 {
                v2
            } else {
                let v16: i32 = v8 + 1i32 ;
                v16
            };
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
            let v19: i32 = 0i32;
            let v20: i32 = 6i32;
            let v21: i32 = find_text_loop_6(v13.clone(), v18.clone(), v19, v14, v20);
            let v22: bool = v21 < v14;
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", Active,"); } LIT.with(|lit| lit.clone()) };
            let v24: i32 = 0i32;
            let v25: i32 = 9i32;
            let v26: i32 = find_text_loop_6(v13.clone(), v23.clone(), v24, v14, v25);
            let v27: bool = v26 < v14;
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", Blocked,"); } LIT.with(|lit| lit.clone()) };
            let v29: i32 = 0i32;
            let v30: i32 = 10i32;
            let v31: i32 = find_text_loop_6(v13.clone(), v28.clone(), v29, v14, v30);
            let v32: bool = v31 < v14;
            let v34: bool = if v22 {
                let v33: bool = v27 || v32;
                v33
            } else {
                false
            };
            if v34 {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let v36: u8 = codec_runtime_char_4(v35.clone());
                let v37: i32 = 0i32;
                let v38: i32 = find_char_loop_5(v13.clone(), v37, v14, v36);
                let v39: bool = v38 == v14 ;
                let v145: Rc<str> = if v39 {
                    codec_empty_2()
                } else {
                    let v41: bool = v38 == v14 ;
                    let (mut v56, mut v57, mut v58): (Rc<str>, i32, bool) = if v41 {
                        let v42: Rc<str> = codec_empty_2();
                        (v42.clone(), v38, false)
                    } else {
                        let v43: u8 = v13.clone().as_bytes()[v38 as usize];
                        let v44: u8 = codec_runtime_char_4(v35.clone());
                        let v45: bool = v43 == v44 ;
                        if v45 {
                            let v46: i32 = v38 + 1i32 ;
                            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v48: bool = false;
                            parse_quoted_loop_8(v13.clone(), v46, v14, v47.clone(), v48)
                        } else {
                            let v52: Rc<str> = codec_empty_2();
                            (v52.clone(), v38, false)
                        }
                    };
                    if v58 {
                        let v59: u8 = codec_runtime_char_4(v35.clone());
                        let v60: i32 = find_char_loop_5(v13.clone(), v57, v14, v59);
                        let v61: bool = v60 == v14 ;
                        if v61 {
                            codec_empty_2()
                        } else {
                            let v63: bool = v60 == v14 ;
                            let (mut v78, mut v79, mut v80): (Rc<str>, i32, bool) = if v63 {
                                let v64: Rc<str> = codec_empty_2();
                                (v64.clone(), v60, false)
                            } else {
                                let v65: u8 = v13.clone().as_bytes()[v60 as usize];
                                let v66: u8 = codec_runtime_char_4(v35.clone());
                                let v67: bool = v65 == v66 ;
                                if v67 {
                                    let v68: i32 = v60 + 1i32 ;
                                    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v70: bool = false;
                                    parse_quoted_loop_8(v13.clone(), v68, v14, v69.clone(), v70)
                                } else {
                                    let v74: Rc<str> = codec_empty_2();
                                    (v74.clone(), v60, false)
                                }
                            };
                            let v82: bool = if v80 {
                                let v81: bool = v79 <= v14;
                                v81
                            } else {
                                false
                            };
                            if v82 {
                                let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                                let v84: i32 = 3i32;
                                let v85: i32 = find_text_loop_6(v13.clone(), v83.clone(), v79, v14, v84);
                                let v86: bool = v85 == v14 ;
                                let v98: Rc<str> = if v86 {
                                    codec_empty_2()
                                } else {
                                    let v88: i32 = projection_digit_start_loop_9(v13.clone(), v85);
                                    let v89: bool = v88 == v85 ;
                                    if v89 {
                                        codec_empty_2()
                                    } else {
                                        let v91: bool = v88 == v85 ;
                                        let v95: Rc<str> = if v91 {
                                            codec_empty_2()
                                        } else {
                                            let v93: i32 = v85 - 1i32 ;
                                            let v94: Rc<str> = string_slice(&v13.clone(), v88 as i64, v93 as i64);
                                            v94.clone()
                                        };
                                        codec_u64_decimal_normalize_10(v95.clone())
                                    }
                                };
                                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v100: bool = v98 == v99 ;
                                if v100 {
                                    codec_empty_2()
                                } else {
                                    let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", P0,"); } LIT.with(|lit| lit.clone()) };
                                    let v103: i32 = 0i32;
                                    let v104: i32 = 5i32;
                                    let v105: i32 = find_text_loop_6(v13.clone(), v102.clone(), v103, v14, v104);
                                    let v106: bool = v105 < v14;
                                    let v123: Rc<str> = if v106 {
                                        let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                                        v107.clone()
                                    } else {
                                        let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", P1,"); } LIT.with(|lit| lit.clone()) };
                                        let v109: i32 = 0i32;
                                        let v110: i32 = 5i32;
                                        let v111: i32 = find_text_loop_6(v13.clone(), v108.clone(), v109, v14, v110);
                                        let v112: bool = v111 < v14;
                                        if v112 {
                                            let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                                            v113.clone()
                                        } else {
                                            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", P2,"); } LIT.with(|lit| lit.clone()) };
                                            let v115: i32 = 0i32;
                                            let v116: i32 = 5i32;
                                            let v117: i32 = find_text_loop_6(v13.clone(), v114.clone(), v115, v14, v116);
                                            let v118: bool = v117 < v14;
                                            if v118 {
                                                let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                                                v119.clone()
                                            } else {
                                                let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3+"); } LIT.with(|lit| lit.clone()) };
                                                v120.clone()
                                            }
                                        }
                                    };
                                    let v124: i32 = 0i32;
                                    let v125: i32 = 10i32;
                                    let v126: i32 = find_text_loop_6(v13.clone(), v28.clone(), v124, v14, v125);
                                    let v127: bool = v126 < v14;
                                    let v130: Rc<str> = if v127 {
                                        let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
                                        v128.clone()
                                    } else {
                                        let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
                                        v129.clone()
                                    };
                                    let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v123.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                                    let v132: Rc<str> = Rc::<str>::from(format!("{}{}", v131.clone(), v130.clone()));
                                    let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v132.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                                    let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v133.clone(), v98.clone()));
                                    let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v134.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("% "); } LIT.with(|lit| lit.clone()) }));
                                    let v136: Rc<str> = Rc::<str>::from(format!("{}{}", v135.clone(), v56.clone()));
                                    let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v136.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" — "); } LIT.with(|lit| lit.clone()) }));
                                    let v138: Rc<str> = Rc::<str>::from(format!("{}{}", v137.clone(), v78.clone()));
                                    v138.clone()
                                }
                            } else {
                                codec_empty_2()
                            }
                        }
                    } else {
                        codec_empty_2()
                    }
                };
                let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v147: bool = v145 == v146 ;
                if v147 {
                    return codec_empty_2();
                } else {
                    let v149: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                    let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v149.clone(), v145.clone()));
                    (v0, v1, v2, v3) = (v0.clone(), v17, v2, v150.clone());
                    continue;
                }
            } else {
                (v0, v1, v2, v3) = (v0.clone(), v17, v2, v3.clone());
                continue;
            }
        }
    }
}
fn handoff_active_work_projection_0(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = 0i32;
    let v2: i32 = (v0.clone().len() as i32);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
    handoff_active_work_projection_loop_1(v0.clone(), v1, v2, v3.clone())
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        handoff_active_work_projection_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_handoff_active_work_projection(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
