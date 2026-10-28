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
fn projection_has_content_loop_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let v3: bool = v1 == v2 ;
        if v3 {
            return false;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v6: u8 = codec_runtime_char_4(v5.clone());
            let v7: bool = v4 == v6 ;
            let v11: bool = if v7 {
                true
            } else {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v9: u8 = codec_runtime_char_4(v8.clone());
                let v10: bool = v4 == v9 ;
                v10
            };
            let v15: bool = if v11 {
                true
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v13: u8 = codec_runtime_char_4(v12.clone());
                let v14: bool = v4 == v13 ;
                v14
            };
            let v19: bool = if v15 {
                true
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v17: u8 = codec_runtime_char_4(v16.clone());
                let v18: bool = v4 == v17 ;
                v18
            };
            if v19 {
                let v20: i32 = v1 + 1i32 ;
                (v0, v1, v2) = (v0.clone(), v20, v2);
                continue;
            } else {
                return true;
            }
        }
    }
}
fn projection_handoff_title_seen_loop_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> bool {
    loop {
        let v5: bool = v1 < v2;
        if v5 {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v7: u8 = codec_runtime_char_4(v6.clone());
            let v8: i32 = (v0.clone().len() as i32);
            let v9: i32 = find_char_loop_5(v0.clone(), v1, v8, v7);
            let v10: bool = v1 == v9 ;
            let v14: Rc<str> = if v10 {
                codec_empty_2()
            } else {
                let v12: i32 = v9 - 1i32 ;
                let v13: Rc<str> = string_slice(&v0.clone(), v1 as i64, v12 as i64);
                v13.clone()
            };
            let v15: i32 = (v14.clone().len() as i32);
            let v16: bool = v9 == v3 ;
            let v18: i32 = if v16 {
                v3
            } else {
                let v17: i32 = v9 + 1i32 ;
                v17
            };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HandoffSection ("); } LIT.with(|lit| lit.clone()) };
            let v20: i32 = 0i32;
            let v21: i32 = 16i32;
            let v22: i32 = find_text_loop_6(v14.clone(), v19.clone(), v20, v15, v21);
            let v23: bool = v22 < v15;
            if v23 {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let v25: u8 = codec_runtime_char_4(v24.clone());
                let v26: i32 = 0i32;
                let v27: i32 = find_char_loop_5(v14.clone(), v26, v15, v25);
                let v28: bool = v27 == v15 ;
                let v56: Rc<str> = if v28 {
                    codec_empty_2()
                } else {
                    let v30: bool = v27 == v15 ;
                    let (mut v45, mut v46, mut v47): (Rc<str>, i32, bool) = if v30 {
                        let v31: Rc<str> = codec_empty_2();
                        (v31.clone(), v27, false)
                    } else {
                        let v32: u8 = v14.clone().as_bytes()[v27 as usize];
                        let v33: u8 = codec_runtime_char_4(v24.clone());
                        let v34: bool = v32 == v33 ;
                        if v34 {
                            let v35: i32 = v27 + 1i32 ;
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v37: bool = false;
                            parse_quoted_loop_8(v14.clone(), v35, v15, v36.clone(), v37)
                        } else {
                            let v41: Rc<str> = codec_empty_2();
                            (v41.clone(), v27, false)
                        }
                    };
                    let v49: bool = if v47 {
                        let v48: bool = v46 <= v15;
                        v48
                    } else {
                        false
                    };
                    let v53: bool = if v49 {
                        let v50: i32 = 0i32;
                        let v51: i32 = (v45.clone().len() as i32);
                        projection_has_content_loop_9(v45.clone(), v50, v51)
                    } else {
                        false
                    };
                    if v53 {
                        v45.clone()
                    } else {
                        codec_empty_2()
                    }
                };
                let v57: bool = v56 == v4 ;
                if v57 {
                    return true;
                } else {
                    (v0, v1, v2, v3, v4) = (v0.clone(), v18, v2, v3, v4.clone());
                    continue;
                }
            } else {
                (v0, v1, v2, v3, v4) = (v0.clone(), v18, v2, v3, v4.clone());
                continue;
            }
        } else {
            return false;
        }
    }
}
fn handoff_sections_projection_loop_1(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v1 == v2 ;
        if v5 {
            let v6: bool = v3 < 4i32;
            if v6 {
                return codec_empty_2();
            } else {
                let v8: bool = 10i32 < v3;
                if v8 {
                    return codec_empty_2();
                } else {
                    return v4.clone();
                }
            }
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v13: u8 = codec_runtime_char_4(v12.clone());
            let v14: i32 = (v0.clone().len() as i32);
            let v15: i32 = find_char_loop_5(v0.clone(), v1, v14, v13);
            let v16: bool = v1 == v15 ;
            let v20: Rc<str> = if v16 {
                codec_empty_2()
            } else {
                let v18: i32 = v15 - 1i32 ;
                let v19: Rc<str> = string_slice(&v0.clone(), v1 as i64, v18 as i64);
                v19.clone()
            };
            let v21: i32 = (v20.clone().len() as i32);
            let v22: bool = v15 == v2 ;
            let v24: i32 = if v22 {
                v2
            } else {
                let v23: i32 = v15 + 1i32 ;
                v23
            };
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HandoffSection ("); } LIT.with(|lit| lit.clone()) };
            let v26: i32 = 0i32;
            let v27: i32 = 16i32;
            let v28: i32 = find_text_loop_6(v20.clone(), v25.clone(), v26, v21, v27);
            let v29: bool = v28 < v21;
            if v29 {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let v31: u8 = codec_runtime_char_4(v30.clone());
                let v32: i32 = 0i32;
                let v33: i32 = find_char_loop_5(v20.clone(), v32, v21, v31);
                let v34: bool = v33 == v21 ;
                let v62: Rc<str> = if v34 {
                    codec_empty_2()
                } else {
                    let v36: bool = v33 == v21 ;
                    let (mut v51, mut v52, mut v53): (Rc<str>, i32, bool) = if v36 {
                        let v37: Rc<str> = codec_empty_2();
                        (v37.clone(), v33, false)
                    } else {
                        let v38: u8 = v20.clone().as_bytes()[v33 as usize];
                        let v39: u8 = codec_runtime_char_4(v30.clone());
                        let v40: bool = v38 == v39 ;
                        if v40 {
                            let v41: i32 = v33 + 1i32 ;
                            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v43: bool = false;
                            parse_quoted_loop_8(v20.clone(), v41, v21, v42.clone(), v43)
                        } else {
                            let v47: Rc<str> = codec_empty_2();
                            (v47.clone(), v33, false)
                        }
                    };
                    let v55: bool = if v53 {
                        let v54: bool = v52 <= v21;
                        v54
                    } else {
                        false
                    };
                    let v59: bool = if v55 {
                        let v56: i32 = 0i32;
                        let v57: i32 = (v51.clone().len() as i32);
                        projection_has_content_loop_9(v51.clone(), v56, v57)
                    } else {
                        false
                    };
                    if v59 {
                        v51.clone()
                    } else {
                        codec_empty_2()
                    }
                };
                let v63: u8 = codec_runtime_char_4(v30.clone());
                let v64: i32 = 0i32;
                let v65: i32 = find_char_loop_5(v20.clone(), v64, v21, v63);
                let v66: bool = v65 == v21 ;
                let v127: Rc<str> = if v66 {
                    codec_empty_2()
                } else {
                    let v68: bool = v65 == v21 ;
                    let (mut v83, mut v84, mut v85): (Rc<str>, i32, bool) = if v68 {
                        let v69: Rc<str> = codec_empty_2();
                        (v69.clone(), v65, false)
                    } else {
                        let v70: u8 = v20.clone().as_bytes()[v65 as usize];
                        let v71: u8 = codec_runtime_char_4(v30.clone());
                        let v72: bool = v70 == v71 ;
                        if v72 {
                            let v73: i32 = v65 + 1i32 ;
                            let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v75: bool = false;
                            parse_quoted_loop_8(v20.clone(), v73, v21, v74.clone(), v75)
                        } else {
                            let v79: Rc<str> = codec_empty_2();
                            (v79.clone(), v65, false)
                        }
                    };
                    let v89: bool = if v85 {
                        let v86: i32 = 0i32;
                        let v87: i32 = (v83.clone().len() as i32);
                        projection_has_content_loop_9(v83.clone(), v86, v87)
                    } else {
                        false
                    };
                    if v89 {
                        let v90: u8 = codec_runtime_char_4(v30.clone());
                        let v91: i32 = find_char_loop_5(v20.clone(), v84, v21, v90);
                        let v92: bool = v91 == v21 ;
                        if v92 {
                            codec_empty_2()
                        } else {
                            let v94: bool = v91 == v21 ;
                            let (mut v109, mut v110, mut v111): (Rc<str>, i32, bool) = if v94 {
                                let v95: Rc<str> = codec_empty_2();
                                (v95.clone(), v91, false)
                            } else {
                                let v96: u8 = v20.clone().as_bytes()[v91 as usize];
                                let v97: u8 = codec_runtime_char_4(v30.clone());
                                let v98: bool = v96 == v97 ;
                                if v98 {
                                    let v99: i32 = v91 + 1i32 ;
                                    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v101: bool = false;
                                    parse_quoted_loop_8(v20.clone(), v99, v21, v100.clone(), v101)
                                } else {
                                    let v105: Rc<str> = codec_empty_2();
                                    (v105.clone(), v91, false)
                                }
                            };
                            let v112: u8 = codec_runtime_char_4(v30.clone());
                            let v113: i32 = find_char_loop_5(v20.clone(), v110, v21, v112);
                            let v115: bool = if v111 {
                                let v114: bool = v110 <= v21;
                                v114
                            } else {
                                false
                            };
                            let v117: bool = if v115 {
                                let v116: bool = v113 == v21 ;
                                v116
                            } else {
                                false
                            };
                            let v121: bool = if v117 {
                                let v118: i32 = 0i32;
                                let v119: i32 = (v109.clone().len() as i32);
                                projection_has_content_loop_9(v109.clone(), v118, v119)
                            } else {
                                false
                            };
                            if v121 {
                                v109.clone()
                            } else {
                                codec_empty_2()
                            }
                        }
                    } else {
                        codec_empty_2()
                    }
                };
                let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v129: bool = v62 == v128 ;
                let v131: bool = if v129 {
                    true
                } else {
                    let v130: bool = v127 == v128 ;
                    v130
                };
                let v134: bool = if v131 {
                    true
                } else {
                    let v132: i32 = 0i32;
                    projection_handoff_title_seen_loop_10(v0.clone(), v132, v1, v14, v62.clone())
                };
                if v134 {
                    return codec_empty_2();
                } else {
                    let v136: bool = v3 == 0i32 ;
                    let v139: Rc<str> = if v136 {
                        let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n## "); } LIT.with(|lit| lit.clone()) };
                        v137.clone()
                    } else {
                        let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n## "); } LIT.with(|lit| lit.clone()) };
                        v138.clone()
                    };
                    let v140: Rc<str> = Rc::<str>::from(format!("{}{}", v4.clone(), v139.clone()));
                    let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v140.clone(), v62.clone()));
                    let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v141.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) }));
                    let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v142.clone(), v127.clone()));
                    let v144: i32 = v3 + 1i32 ;
                    (v0, v1, v2, v3, v4) = (v0.clone(), v24, v2, v144, v143.clone());
                    continue;
                }
            } else {
                (v0, v1, v2, v3, v4) = (v0.clone(), v24, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn handoff_sections_projection_0(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = 0i32;
    let v2: i32 = (v0.clone().len() as i32);
    let v3: i32 = 0i32;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
    handoff_sections_projection_loop_1(v0.clone(), v1, v2, v3, v4.clone())
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        handoff_sections_projection_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_handoff_sections_projection(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
