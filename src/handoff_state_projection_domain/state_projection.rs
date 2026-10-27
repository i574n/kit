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
fn method2() -> Rc<str> {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_1() -> Rc<str> {
    method2()
}
fn starts_with_loop_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
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
fn find_text_loop_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
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
                starts_with_loop_4(v0.clone(), v1.clone(), v2, v11, v8)
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
fn codec_runtime_char_5(mut v0: Rc<str>) -> u8 {
    let v1: u8 = v0.as_bytes()[0i32 as usize];
    v1
}
fn find_char_loop_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
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
fn projection_find_digit_loop_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v1 == v2 ;
        if v3 {
            return v2;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
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
                return v1;
            } else {
                let v26: i32 = v1 + 1i32 ;
                (v0, v1, v2) = (v0.clone(), v26, v2);
                continue;
            }
        }
    }
}
fn codec_u64_decimal_valid_loop_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u64) -> bool {
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
fn codec_u64_decimal_normalize_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: bool = v1 == 0i32 ;
    if v2 {
        codec_empty_1()
    } else {
        let v4: i32 = 0i32;
        let v5: u64 = 0u64;
        let v6: bool = codec_u64_decimal_valid_loop_9(v0.clone(), v4, v1, v5);
        if v6 {
            v0
        } else {
            codec_empty_1()
        }
    }
}
fn handoff_ratings_projection_0(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl active_codebase_readiness"); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = 0i32;
    let v4: i32 = 29i32;
    let v5: i32 = find_text_loop_3(v0.clone(), v2.clone(), v3, v1, v4);
    let v6: bool = v5 == v1 ;
    let v16: Rc<str> = if v6 {
        codec_empty_1()
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v9: u8 = codec_runtime_char_5(v8.clone());
        let v10: i32 = find_char_loop_6(v0.clone(), v5, v1, v9);
        let v11: bool = v5 == v10 ;
        if v11 {
            codec_empty_1()
        } else {
            let v13: i32 = v10 - 1i32 ;
            let v14: Rc<str> = string_slice(&v0.clone(), v5 as i64, v13 as i64);
            v14.clone()
        }
    };
    let v17: i32 = (v16.clone().len() as i32);
    let v18: bool = v17 == 0i32 ;
    let v43: Rc<str> = if v18 {
        codec_empty_1()
    } else {
        let v20: i32 = 0i32;
        let v21: u8 = b'=';
        let v22: i32 = find_char_loop_6(v16.clone(), v20, v17, v21);
        let v23: bool = v22 == v17 ;
        if v23 {
            codec_empty_1()
        } else {
            let v25: i32 = v22 + 1i32 ;
            let v26: i32 = projection_find_digit_loop_7(v16.clone(), v25, v17);
            let v27: bool = v26 == v17 ;
            if v27 {
                codec_empty_1()
            } else {
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                let v30: i32 = 3i32;
                let v31: i32 = find_text_loop_3(v16.clone(), v29.clone(), v26, v17, v30);
                let v32: bool = v31 == v17 ;
                if v32 {
                    codec_empty_1()
                } else {
                    let v34: bool = v26 == v31 ;
                    let v38: Rc<str> = if v34 {
                        codec_empty_1()
                    } else {
                        let v36: i32 = v31 - 1i32 ;
                        let v37: Rc<str> = string_slice(&v16.clone(), v26 as i64, v36 as i64);
                        v37.clone()
                    };
                    codec_u64_decimal_normalize_8(v38.clone())
                }
            }
        }
    };
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl subjective_phase_ceiling"); } LIT.with(|lit| lit.clone()) };
    let v45: i32 = 0i32;
    let v46: i32 = 28i32;
    let v47: i32 = find_text_loop_3(v0.clone(), v44.clone(), v45, v1, v46);
    let v48: bool = v47 == v1 ;
    let v58: Rc<str> = if v48 {
        codec_empty_1()
    } else {
        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v51: u8 = codec_runtime_char_5(v50.clone());
        let v52: i32 = find_char_loop_6(v0.clone(), v47, v1, v51);
        let v53: bool = v47 == v52 ;
        if v53 {
            codec_empty_1()
        } else {
            let v55: i32 = v52 - 1i32 ;
            let v56: Rc<str> = string_slice(&v0.clone(), v47 as i64, v55 as i64);
            v56.clone()
        }
    };
    let v59: i32 = (v58.clone().len() as i32);
    let v60: bool = v59 == 0i32 ;
    let v85: Rc<str> = if v60 {
        codec_empty_1()
    } else {
        let v62: i32 = 0i32;
        let v63: u8 = b'=';
        let v64: i32 = find_char_loop_6(v58.clone(), v62, v59, v63);
        let v65: bool = v64 == v59 ;
        if v65 {
            codec_empty_1()
        } else {
            let v67: i32 = v64 + 1i32 ;
            let v68: i32 = projection_find_digit_loop_7(v58.clone(), v67, v59);
            let v69: bool = v68 == v59 ;
            if v69 {
                codec_empty_1()
            } else {
                let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                let v72: i32 = 3i32;
                let v73: i32 = find_text_loop_3(v58.clone(), v71.clone(), v68, v59, v72);
                let v74: bool = v73 == v59 ;
                if v74 {
                    codec_empty_1()
                } else {
                    let v76: bool = v68 == v73 ;
                    let v80: Rc<str> = if v76 {
                        codec_empty_1()
                    } else {
                        let v78: i32 = v73 - 1i32 ;
                        let v79: Rc<str> = string_slice(&v58.clone(), v68 as i64, v78 as i64);
                        v79.clone()
                    };
                    codec_u64_decimal_normalize_8(v80.clone())
                }
            }
        }
    };
    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl active_coverage_attainment"); } LIT.with(|lit| lit.clone()) };
    let v87: i32 = 0i32;
    let v88: i32 = 30i32;
    let v89: i32 = find_text_loop_3(v0.clone(), v86.clone(), v87, v1, v88);
    let v90: bool = v89 == v1 ;
    let v100: Rc<str> = if v90 {
        codec_empty_1()
    } else {
        let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v93: u8 = codec_runtime_char_5(v92.clone());
        let v94: i32 = find_char_loop_6(v0.clone(), v89, v1, v93);
        let v95: bool = v89 == v94 ;
        if v95 {
            codec_empty_1()
        } else {
            let v97: i32 = v94 - 1i32 ;
            let v98: Rc<str> = string_slice(&v0.clone(), v89 as i64, v97 as i64);
            v98.clone()
        }
    };
    let v101: i32 = (v100.clone().len() as i32);
    let v102: bool = v101 == 0i32 ;
    let v127: Rc<str> = if v102 {
        codec_empty_1()
    } else {
        let v104: i32 = 0i32;
        let v105: u8 = b'=';
        let v106: i32 = find_char_loop_6(v100.clone(), v104, v101, v105);
        let v107: bool = v106 == v101 ;
        if v107 {
            codec_empty_1()
        } else {
            let v109: i32 = v106 + 1i32 ;
            let v110: i32 = projection_find_digit_loop_7(v100.clone(), v109, v101);
            let v111: bool = v110 == v101 ;
            if v111 {
                codec_empty_1()
            } else {
                let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                let v114: i32 = 3i32;
                let v115: i32 = find_text_loop_3(v100.clone(), v113.clone(), v110, v101, v114);
                let v116: bool = v115 == v101 ;
                if v116 {
                    codec_empty_1()
                } else {
                    let v118: bool = v110 == v115 ;
                    let v122: Rc<str> = if v118 {
                        codec_empty_1()
                    } else {
                        let v120: i32 = v115 - 1i32 ;
                        let v121: Rc<str> = string_slice(&v100.clone(), v110 as i64, v120 as i64);
                        v121.clone()
                    };
                    codec_u64_decimal_normalize_8(v122.clone())
                }
            }
        }
    };
    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl phase_runtime_drift"); } LIT.with(|lit| lit.clone()) };
    let v129: i32 = 0i32;
    let v130: i32 = 23i32;
    let v131: i32 = find_text_loop_3(v0.clone(), v128.clone(), v129, v1, v130);
    let v132: bool = v131 == v1 ;
    let v142: Rc<str> = if v132 {
        codec_empty_1()
    } else {
        let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v135: u8 = codec_runtime_char_5(v134.clone());
        let v136: i32 = find_char_loop_6(v0.clone(), v131, v1, v135);
        let v137: bool = v131 == v136 ;
        if v137 {
            codec_empty_1()
        } else {
            let v139: i32 = v136 - 1i32 ;
            let v140: Rc<str> = string_slice(&v0.clone(), v131 as i64, v139 as i64);
            v140.clone()
        }
    };
    let v143: i32 = (v142.clone().len() as i32);
    let v144: bool = v143 == 0i32 ;
    let v169: Rc<str> = if v144 {
        codec_empty_1()
    } else {
        let v146: i32 = 0i32;
        let v147: u8 = b'=';
        let v148: i32 = find_char_loop_6(v142.clone(), v146, v143, v147);
        let v149: bool = v148 == v143 ;
        if v149 {
            codec_empty_1()
        } else {
            let v151: i32 = v148 + 1i32 ;
            let v152: i32 = projection_find_digit_loop_7(v142.clone(), v151, v143);
            let v153: bool = v152 == v143 ;
            if v153 {
                codec_empty_1()
            } else {
                let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                let v156: i32 = 3i32;
                let v157: i32 = find_text_loop_3(v142.clone(), v155.clone(), v152, v143, v156);
                let v158: bool = v157 == v143 ;
                if v158 {
                    codec_empty_1()
                } else {
                    let v160: bool = v152 == v157 ;
                    let v164: Rc<str> = if v160 {
                        codec_empty_1()
                    } else {
                        let v162: i32 = v157 - 1i32 ;
                        let v163: Rc<str> = string_slice(&v142.clone(), v152 as i64, v162 as i64);
                        v163.clone()
                    };
                    codec_u64_decimal_normalize_8(v164.clone())
                }
            }
        }
    };
    let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl phase_parallel_readiness"); } LIT.with(|lit| lit.clone()) };
    let v171: i32 = 0i32;
    let v172: i32 = 28i32;
    let v173: i32 = find_text_loop_3(v0.clone(), v170.clone(), v171, v1, v172);
    let v174: bool = v173 == v1 ;
    let v184: Rc<str> = if v174 {
        codec_empty_1()
    } else {
        let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v177: u8 = codec_runtime_char_5(v176.clone());
        let v178: i32 = find_char_loop_6(v0.clone(), v173, v1, v177);
        let v179: bool = v173 == v178 ;
        if v179 {
            codec_empty_1()
        } else {
            let v181: i32 = v178 - 1i32 ;
            let v182: Rc<str> = string_slice(&v0.clone(), v173 as i64, v181 as i64);
            v182.clone()
        }
    };
    let v185: i32 = (v184.clone().len() as i32);
    let v186: bool = v185 == 0i32 ;
    let v211: Rc<str> = if v186 {
        codec_empty_1()
    } else {
        let v188: i32 = 0i32;
        let v189: u8 = b'=';
        let v190: i32 = find_char_loop_6(v184.clone(), v188, v185, v189);
        let v191: bool = v190 == v185 ;
        if v191 {
            codec_empty_1()
        } else {
            let v193: i32 = v190 + 1i32 ;
            let v194: i32 = projection_find_digit_loop_7(v184.clone(), v193, v185);
            let v195: bool = v194 == v185 ;
            if v195 {
                codec_empty_1()
            } else {
                let v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                let v198: i32 = 3i32;
                let v199: i32 = find_text_loop_3(v184.clone(), v197.clone(), v194, v185, v198);
                let v200: bool = v199 == v185 ;
                if v200 {
                    codec_empty_1()
                } else {
                    let v202: bool = v194 == v199 ;
                    let v206: Rc<str> = if v202 {
                        codec_empty_1()
                    } else {
                        let v204: i32 = v199 - 1i32 ;
                        let v205: Rc<str> = string_slice(&v184.clone(), v194 as i64, v204 as i64);
                        v205.clone()
                    };
                    codec_u64_decimal_normalize_8(v206.clone())
                }
            }
        }
    };
    let v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl phase_plan_ir"); } LIT.with(|lit| lit.clone()) };
    let v213: i32 = 0i32;
    let v214: i32 = 17i32;
    let v215: i32 = find_text_loop_3(v0.clone(), v212.clone(), v213, v1, v214);
    let v216: bool = v215 == v1 ;
    let v226: Rc<str> = if v216 {
        codec_empty_1()
    } else {
        let v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v219: u8 = codec_runtime_char_5(v218.clone());
        let v220: i32 = find_char_loop_6(v0.clone(), v215, v1, v219);
        let v221: bool = v215 == v220 ;
        if v221 {
            codec_empty_1()
        } else {
            let v223: i32 = v220 - 1i32 ;
            let v224: Rc<str> = string_slice(&v0.clone(), v215 as i64, v223 as i64);
            v224.clone()
        }
    };
    let v227: i32 = (v226.clone().len() as i32);
    let v228: bool = v227 == 0i32 ;
    let v253: Rc<str> = if v228 {
        codec_empty_1()
    } else {
        let v230: i32 = 0i32;
        let v231: u8 = b'=';
        let v232: i32 = find_char_loop_6(v226.clone(), v230, v227, v231);
        let v233: bool = v232 == v227 ;
        if v233 {
            codec_empty_1()
        } else {
            let v235: i32 = v232 + 1i32 ;
            let v236: i32 = projection_find_digit_loop_7(v226.clone(), v235, v227);
            let v237: bool = v236 == v227 ;
            if v237 {
                codec_empty_1()
            } else {
                let v239: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                let v240: i32 = 3i32;
                let v241: i32 = find_text_loop_3(v226.clone(), v239.clone(), v236, v227, v240);
                let v242: bool = v241 == v227 ;
                if v242 {
                    codec_empty_1()
                } else {
                    let v244: bool = v236 == v241 ;
                    let v248: Rc<str> = if v244 {
                        codec_empty_1()
                    } else {
                        let v246: i32 = v241 - 1i32 ;
                        let v247: Rc<str> = string_slice(&v226.clone(), v236 as i64, v246 as i64);
                        v247.clone()
                    };
                    codec_u64_decimal_normalize_8(v248.clone())
                }
            }
        }
    };
    let v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl migration ()"); } LIT.with(|lit| lit.clone()) };
    let v255: i32 = 0i32;
    let v256: i32 = 16i32;
    let v257: i32 = find_text_loop_3(v0.clone(), v254.clone(), v255, v1, v256);
    let v258: bool = v257 == v1 ;
    let v268: Rc<str> = if v258 {
        codec_empty_1()
    } else {
        let v260: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v261: u8 = codec_runtime_char_5(v260.clone());
        let v262: i32 = find_char_loop_6(v0.clone(), v257, v1, v261);
        let v263: bool = v257 == v262 ;
        if v263 {
            codec_empty_1()
        } else {
            let v265: i32 = v262 - 1i32 ;
            let v266: Rc<str> = string_slice(&v0.clone(), v257 as i64, v265 as i64);
            v266.clone()
        }
    };
    let v269: i32 = (v268.clone().len() as i32);
    let v270: bool = v269 == 0i32 ;
    let v295: Rc<str> = if v270 {
        codec_empty_1()
    } else {
        let v272: i32 = 0i32;
        let v273: u8 = b'=';
        let v274: i32 = find_char_loop_6(v268.clone(), v272, v269, v273);
        let v275: bool = v274 == v269 ;
        if v275 {
            codec_empty_1()
        } else {
            let v277: i32 = v274 + 1i32 ;
            let v278: i32 = projection_find_digit_loop_7(v268.clone(), v277, v269);
            let v279: bool = v278 == v269 ;
            if v279 {
                codec_empty_1()
            } else {
                let v281: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                let v282: i32 = 3i32;
                let v283: i32 = find_text_loop_3(v268.clone(), v281.clone(), v278, v269, v282);
                let v284: bool = v283 == v269 ;
                if v284 {
                    codec_empty_1()
                } else {
                    let v286: bool = v278 == v283 ;
                    let v290: Rc<str> = if v286 {
                        codec_empty_1()
                    } else {
                        let v288: i32 = v283 - 1i32 ;
                        let v289: Rc<str> = string_slice(&v268.clone(), v278 as i64, v288 as i64);
                        v289.clone()
                    };
                    codec_u64_decimal_normalize_8(v290.clone())
                }
            }
        }
    };
    let v296: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v297: bool = v43 == v296 ;
    if v297 {
        codec_empty_1()
    } else {
        let v299: bool = v85 == v296 ;
        if v299 {
            codec_empty_1()
        } else {
            let v301: bool = v127 == v296 ;
            if v301 {
                codec_empty_1()
            } else {
                let v303: bool = v169 == v296 ;
                if v303 {
                    codec_empty_1()
                } else {
                    let v305: bool = v211 == v296 ;
                    if v305 {
                        codec_empty_1()
                    } else {
                        let v307: bool = v253 == v296 ;
                        if v307 {
                            codec_empty_1()
                        } else {
                            let v309: bool = v295 == v296 ;
                            if v309 {
                                codec_empty_1()
                            } else {
                                let v311: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active_readiness="); } LIT.with(|lit| lit.clone()) }, v43.clone()));
                                let v312: Rc<str> = Rc::<str>::from(format!("{}{}", v311.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/1000 subjective_ceiling="); } LIT.with(|lit| lit.clone()) }));
                                let v313: Rc<str> = Rc::<str>::from(format!("{}{}", v312.clone(), v85.clone()));
                                let v314: Rc<str> = Rc::<str>::from(format!("{}{}", v313.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/1000 coverage="); } LIT.with(|lit| lit.clone()) }));
                                let v315: Rc<str> = Rc::<str>::from(format!("{}{}", v314.clone(), v127.clone()));
                                let v316: Rc<str> = Rc::<str>::from(format!("{}{}", v315.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/1000 runtime_drift="); } LIT.with(|lit| lit.clone()) }));
                                let v317: Rc<str> = Rc::<str>::from(format!("{}{}", v316.clone(), v169.clone()));
                                let v318: Rc<str> = Rc::<str>::from(format!("{}{}", v317.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/1000 parallel_readiness="); } LIT.with(|lit| lit.clone()) }));
                                let v319: Rc<str> = Rc::<str>::from(format!("{}{}", v318.clone(), v211.clone()));
                                let v320: Rc<str> = Rc::<str>::from(format!("{}{}", v319.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/1000 canonical_plan_ir="); } LIT.with(|lit| lit.clone()) }));
                                let v321: Rc<str> = Rc::<str>::from(format!("{}{}", v320.clone(), v253.clone()));
                                let v322: Rc<str> = Rc::<str>::from(format!("{}{}", v321.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/1000 migration_historical="); } LIT.with(|lit| lit.clone()) }));
                                let v323: Rc<str> = Rc::<str>::from(format!("{}{}", v322.clone(), v295.clone()));
                                let v324: Rc<str> = Rc::<str>::from(format!("{}{}", v323.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/1000"); } LIT.with(|lit| lit.clone()) }));
                                v324.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}
fn projection_census_loop_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: i32 = projection_find_digit_loop_7(v0.clone(), v1, v2);
        let v6: bool = v5 == v2 ;
        if v6 {
            let v7: bool = v3 == 10i32 ;
            if v7 {
                return v4.clone();
            } else {
                return codec_empty_1();
            }
        } else {
            let v10: bool = v3 >= 10i32;
            if v10 {
                return codec_empty_1();
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                let v13: i32 = (v0.clone().len() as i32);
                let v14: i32 = 3i32;
                let v15: i32 = find_text_loop_3(v0.clone(), v12.clone(), v5, v13, v14);
                let v16: bool = v15 == v2 ;
                if v16 {
                    return codec_empty_1();
                } else {
                    let v18: bool = v5 == v15 ;
                    let v22: Rc<str> = if v18 {
                        codec_empty_1()
                    } else {
                        let v20: i32 = v15 - 1i32 ;
                        let v21: Rc<str> = string_slice(&v0.clone(), v5 as i64, v20 as i64);
                        v21.clone()
                    };
                    let v23: Rc<str> = codec_u64_decimal_normalize_8(v22.clone());
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v25: bool = v23 == v24 ;
                    if v25 {
                        return codec_empty_1();
                    } else {
                        let v27: bool = v3 == 0i32 ;
                        let v54: Rc<str> = if v27 {
                            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral_files="); } LIT.with(|lit| lit.clone()) };
                            v28.clone()
                        } else {
                            let v29: bool = v3 == 1i32 ;
                            if v29 {
                                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" generated_rust_files="); } LIT.with(|lit| lit.clone()) };
                                v30.clone()
                            } else {
                                let v31: bool = v3 == 2i32 ;
                                if v31 {
                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rust_global_sites="); } LIT.with(|lit| lit.clone()) };
                                    v32.clone()
                                } else {
                                    let v33: bool = v3 == 3i32 ;
                                    if v33 {
                                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" spiral_loc="); } LIT.with(|lit| lit.clone()) };
                                        v34.clone()
                                    } else {
                                        let v35: bool = v3 == 4i32 ;
                                        if v35 {
                                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" generated_rust_loc="); } LIT.with(|lit| lit.clone()) };
                                            v36.clone()
                                        } else {
                                            let v37: bool = v3 == 5i32 ;
                                            if v37 {
                                                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cargo_crates="); } LIT.with(|lit| lit.clone()) };
                                                v38.clone()
                                            } else {
                                                let v39: bool = v3 == 6i32 ;
                                                if v39 {
                                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" spiral_packages="); } LIT.with(|lit| lit.clone()) };
                                                    v40.clone()
                                                } else {
                                                    let v41: bool = v3 == 7i32 ;
                                                    if v41 {
                                                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" package_only="); } LIT.with(|lit| lit.clone()) };
                                                        v42.clone()
                                                    } else {
                                                        let v43: bool = v3 == 8i32 ;
                                                        if v43 {
                                                            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_crate_loc="); } LIT.with(|lit| lit.clone()) };
                                                            v44.clone()
                                                        } else {
                                                            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_rust_file_loc="); } LIT.with(|lit| lit.clone()) };
                                                            v45.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54.clone(), v23.clone()));
                        let v56: i32 = v15 + 3i32 ;
                        let v57: i32 = v3 + 1i32 ;
                        let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v4.clone(), v55.clone()));
                        (v0, v1, v2, v3, v4) = (v0.clone(), v56, v2, v57, v58.clone());
                        continue;
                    }
                }
            }
        }
    }
}
fn handoff_census_projection_10(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("AuthorityCensus ("); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = 0i32;
    let v4: i32 = 17i32;
    let v5: i32 = find_text_loop_3(v0.clone(), v2.clone(), v3, v1, v4);
    let v6: bool = v5 == v1 ;
    let v16: Rc<str> = if v6 {
        codec_empty_1()
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v9: u8 = codec_runtime_char_5(v8.clone());
        let v10: i32 = find_char_loop_6(v0.clone(), v5, v1, v9);
        let v11: bool = v5 == v10 ;
        if v11 {
            codec_empty_1()
        } else {
            let v13: i32 = v10 - 1i32 ;
            let v14: Rc<str> = string_slice(&v0.clone(), v5 as i64, v13 as i64);
            v14.clone()
        }
    };
    let v17: i32 = (v16.clone().len() as i32);
    let v18: bool = v17 == 0i32 ;
    if v18 {
        codec_empty_1()
    } else {
        let v20: i32 = 0i32;
        let v21: i32 = 0i32;
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        projection_census_loop_11(v16.clone(), v20, v17, v21, v22.clone())
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        handoff_ratings_projection_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        handoff_census_projection_10(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_handoff_ratings_projection(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_handoff_census_projection(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
