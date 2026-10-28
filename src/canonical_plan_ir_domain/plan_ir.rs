#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalPlanRow { pub op: String, pub target: String, pub expected: String, pub payload: String, pub effect: String }

fn plan_hex_nibble(byte: u8) -> Option<u8> { match byte { b'0'..=b'9' => Some(byte-b'0'), b'a'..=b'f' => Some(byte-b'a'+10), b'A'..=b'F' => Some(byte-b'A'+10), _ => None } }
fn plan_hex_decode(value: &str) -> Result<String,String> { if !value.len().is_multiple_of(2) { return Err("canonical plan IR hex field has odd length".to_owned()); } let bytes=value.as_bytes(); let mut out=Vec::with_capacity(bytes.len()/2); let mut index=0usize; while index<bytes.len() { let hi=plan_hex_nibble(bytes[index]).ok_or_else(|| "canonical plan IR contains non-hex data".to_owned())?; let lo=plan_hex_nibble(bytes[index+1]).ok_or_else(|| "canonical plan IR contains non-hex data".to_owned())?; out.push((hi<<4)|lo); index+=2; } String::from_utf8(out).map_err(|error| format!("canonical plan IR field is not UTF-8: {error}")) }

pub fn parse_plan_ir_manifest(text: &str) -> Result<Vec<CanonicalPlanRow>,String> { let mut lines=text.lines(); let header=lines.next().unwrap_or_default(); if kit_plan_ir_header_projection(header).as_ref()!="1" { return Err("canonical plan IR header mismatch".to_owned()); } let mut rows=Vec::new(); for (index,line) in lines.enumerate() { if line.is_empty() { continue; } let (op_hex,target_hex,expected_hex,payload_hex,effect_hex)=kit_plan_ir_row_hex_tuple(line); if op_hex.is_empty() { return Err(format!("canonical plan IR row {} has invalid shape", index+1)); } let row=CanonicalPlanRow { op:plan_hex_decode(op_hex.as_ref())?, target:plan_hex_decode(target_hex.as_ref())?, expected:plan_hex_decode(expected_hex.as_ref())?, payload:plan_hex_decode(payload_hex.as_ref())?, effect:plan_hex_decode(effect_hex.as_ref())? }; let code=kit_plan_ir_decode_code(&row.op,&row.target,&row.expected,&row.payload,&row.effect); if code>=8 { return Err(format!("canonical plan IR row {} was rejected by Spiral decoder", index+1)); } rows.push(row); } if rows.is_empty() { return Err("canonical plan IR manifest contains no operations".to_owned()); } Ok(rows) }
pub fn plan_ir_manifest_count(value: &str) -> i32 { let header=value.lines().next().unwrap_or_default(); if kit_plan_ir_header_projection(header).as_ref()=="1" { match parse_plan_ir_manifest(value) { Ok(rows) => i32::try_from(rows.len()).unwrap_or(-1), Err(_) => -1 } } else if header.starts_with("KIT-PLAN-IR") { -1 } else { 0 } }

fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
fn plan_schema_tag_0(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = &*v0 == "schema";
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("canonical-plan-ir"); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    }
}
fn plan_descriptor_by_index_1(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: i32 = (v0).parse::<u8>().ok().filter(|index| index.to_string() == &*v0).map(|index| index as i32).unwrap_or(-1);
    let v2: bool = 0i32 == v1;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("create"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/example.spi"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("typed-payload"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
        (v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone())
    } else {
        let v8: bool = 1i32 == v1;
        if v8 {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/example.spi"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacement"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
            (v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone())
        } else {
            let v14: bool = 2i32 == v1;
            if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch-exact"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/example.spi"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("old"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("new"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                (v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone())
            } else {
                let v20: bool = 3i32 == v1;
                if v20 {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mkdir"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/generated"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                    (v21.clone(), v22.clone(), v23.clone(), v23.clone(), v24.clone())
                } else {
                    let v25: bool = 4i32 == v1;
                    if v25 {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cargo-check"); } LIT.with(|lit| lit.clone()) };
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        (v26.clone(), v27.clone(), v28.clone(), v28.clone(), v26.clone())
                    } else {
                        let v29: bool = 5i32 == v1;
                        if v29 {
                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy"); } LIT.with(|lit| lit.clone()) };
                            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/copied.spi"); } LIT.with(|lit| lit.clone()) };
                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected-sha"); } LIT.with(|lit| lit.clone()) };
                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/source.spi"); } LIT.with(|lit| lit.clone()) };
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                            (v30.clone(), v31.clone(), v32.clone(), v33.clone(), v34.clone())
                        } else {
                            let v35: bool = 6i32 == v1;
                            if v35 {
                                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chmod"); } LIT.with(|lit| lit.clone()) };
                                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/tool"); } LIT.with(|lit| lit.clone()) };
                                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0755"); } LIT.with(|lit| lit.clone()) };
                                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0644"); } LIT.with(|lit| lit.clone()) };
                                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                                (v36.clone(), v37.clone(), v38.clone(), v39.clone(), v40.clone())
                            } else {
                                let v41: bool = 7i32 == v1;
                                if v41 {
                                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("symlink"); } LIT.with(|lit| lit.clone()) };
                                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/current"); } LIT.with(|lit| lit.clone()) };
                                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/target"); } LIT.with(|lit| lit.clone()) };
                                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                                    (v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46.clone())
                                } else {
                                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid"); } LIT.with(|lit| lit.clone()) };
                                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    (v47.clone(), v48.clone(), v48.clone(), v48.clone(), v47.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method4() -> Rc<str> {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_3() -> Rc<str> {
    method4()
}
fn plan_manifest_header_projection_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = &*v0 == "KIT-PLAN-IR\t1";
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        codec_empty_3()
    }
}
fn codec_runtime_char_6(mut v0: Rc<str>) -> u8 {
    let v1: u8 = v0.as_bytes()[0i32 as usize];
    v1
}
fn find_char_loop_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
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
fn plan_manifest_row_hex_tuple_5(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v3: u8 = codec_runtime_char_6(v2.clone());
    let v4: i32 = 0i32;
    let v5: i32 = find_char_loop_7(v0.clone(), v4, v1, v3);
    let v6: bool = v5 == v1 ;
    if v6 {
        let v7: Rc<str> = codec_empty_3();
        let v8: Rc<str> = codec_empty_3();
        let v9: Rc<str> = codec_empty_3();
        let v10: Rc<str> = codec_empty_3();
        let v11: Rc<str> = codec_empty_3();
        (v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone())
    } else {
        let v12: i32 = v5 + 1i32 ;
        let v13: i32 = find_char_loop_7(v0.clone(), v12, v1, v3);
        let v14: bool = v13 == v1 ;
        if v14 {
            let v15: Rc<str> = codec_empty_3();
            let v16: Rc<str> = codec_empty_3();
            let v17: Rc<str> = codec_empty_3();
            let v18: Rc<str> = codec_empty_3();
            let v19: Rc<str> = codec_empty_3();
            (v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone())
        } else {
            let v20: i32 = v13 + 1i32 ;
            let v21: i32 = find_char_loop_7(v0.clone(), v20, v1, v3);
            let v22: bool = v21 == v1 ;
            if v22 {
                let v23: Rc<str> = codec_empty_3();
                let v24: Rc<str> = codec_empty_3();
                let v25: Rc<str> = codec_empty_3();
                let v26: Rc<str> = codec_empty_3();
                let v27: Rc<str> = codec_empty_3();
                (v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone())
            } else {
                let v28: i32 = v21 + 1i32 ;
                let v29: i32 = find_char_loop_7(v0.clone(), v28, v1, v3);
                let v30: bool = v29 == v1 ;
                if v30 {
                    let v31: Rc<str> = codec_empty_3();
                    let v32: Rc<str> = codec_empty_3();
                    let v33: Rc<str> = codec_empty_3();
                    let v34: Rc<str> = codec_empty_3();
                    let v35: Rc<str> = codec_empty_3();
                    (v31.clone(), v32.clone(), v33.clone(), v34.clone(), v35.clone())
                } else {
                    let v36: i32 = v29 + 1i32 ;
                    let v37: i32 = find_char_loop_7(v0.clone(), v36, v1, v3);
                    let v38: bool = v37 == v1 ;
                    if v38 {
                        let v39: Rc<str> = codec_empty_3();
                        let v40: Rc<str> = codec_empty_3();
                        let v41: Rc<str> = codec_empty_3();
                        let v42: Rc<str> = codec_empty_3();
                        let v43: Rc<str> = codec_empty_3();
                        (v39.clone(), v40.clone(), v41.clone(), v42.clone(), v43.clone())
                    } else {
                        let v44: i32 = v37 + 1i32 ;
                        let v45: i32 = find_char_loop_7(v0.clone(), v44, v1, v3);
                        let v46: bool = 0i32 == v5 ;
                        let v50: Rc<str> = if v46 {
                            codec_empty_3()
                        } else {
                            let v48: i32 = v5 - 1i32 ;
                            let v49: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v48 as i64);
                            v49.clone()
                        };
                        let v51: bool = v45 < v1;
                        if v51 {
                            let v52: Rc<str> = codec_empty_3();
                            let v53: Rc<str> = codec_empty_3();
                            let v54: Rc<str> = codec_empty_3();
                            let v55: Rc<str> = codec_empty_3();
                            let v56: Rc<str> = codec_empty_3();
                            (v52.clone(), v53.clone(), v54.clone(), v55.clone(), v56.clone())
                        } else {
                            let v57: bool = &*v50 == "op";
                            if v57 {
                                let v58: i32 = v5 + 1i32 ;
                                let v59: bool = v58 == v13 ;
                                let v63: Rc<str> = if v59 {
                                    codec_empty_3()
                                } else {
                                    let v61: i32 = v13 - 1i32 ;
                                    let v62: Rc<str> = string_slice(&v0.clone(), v58 as i64, v61 as i64);
                                    v62.clone()
                                };
                                let v64: i32 = v13 + 1i32 ;
                                let v65: bool = v64 == v21 ;
                                let v69: Rc<str> = if v65 {
                                    codec_empty_3()
                                } else {
                                    let v67: i32 = v21 - 1i32 ;
                                    let v68: Rc<str> = string_slice(&v0.clone(), v64 as i64, v67 as i64);
                                    v68.clone()
                                };
                                let v70: i32 = v21 + 1i32 ;
                                let v71: bool = v70 == v29 ;
                                let v75: Rc<str> = if v71 {
                                    codec_empty_3()
                                } else {
                                    let v73: i32 = v29 - 1i32 ;
                                    let v74: Rc<str> = string_slice(&v0.clone(), v70 as i64, v73 as i64);
                                    v74.clone()
                                };
                                let v76: i32 = v29 + 1i32 ;
                                let v77: bool = v76 == v37 ;
                                let v81: Rc<str> = if v77 {
                                    codec_empty_3()
                                } else {
                                    let v79: i32 = v37 - 1i32 ;
                                    let v80: Rc<str> = string_slice(&v0.clone(), v76 as i64, v79 as i64);
                                    v80.clone()
                                };
                                let v82: i32 = v37 + 1i32 ;
                                let v83: bool = v82 == v1 ;
                                let v87: Rc<str> = if v83 {
                                    codec_empty_3()
                                } else {
                                    let v85: i32 = v1 - 1i32 ;
                                    let v86: Rc<str> = string_slice(&v0.clone(), v82 as i64, v85 as i64);
                                    v86.clone()
                                };
                                (v63.clone(), v69.clone(), v75.clone(), v81.clone(), v87.clone())
                            } else {
                                let v88: Rc<str> = codec_empty_3();
                                let v89: Rc<str> = codec_empty_3();
                                let v90: Rc<str> = codec_empty_3();
                                let v91: Rc<str> = codec_empty_3();
                                let v92: Rc<str> = codec_empty_3();
                                (v88.clone(), v89.clone(), v90.clone(), v91.clone(), v92.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn plan_manifest_projection_witness_8() -> i32 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT-PLAN-IR\t1"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = plan_manifest_header_projection_2(v0.clone());
    let v2: bool = &*v1 == "1";
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT-PLAN-IR\t2"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = plan_manifest_header_projection_2(v3.clone());
        let v5: bool = &*v4 == "";
        if v5 {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("op\t01\t02\t03\t04\t05"); } LIT.with(|lit| lit.clone()) };
            let (mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = plan_manifest_row_hex_tuple_5(v6.clone());
            let v12: bool = &*v7 == "01";
            if v12 {
                let v13: bool = &*v8 == "02";
                if v13 {
                    let v14: bool = &*v9 == "03";
                    if v14 {
                        let v15: bool = &*v10 == "04";
                        if v15 {
                            let v16: bool = &*v11 == "05";
                            if v16 {
                                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("op\t01\t02\t03\t04\t05\t06"); } LIT.with(|lit| lit.clone()) };
                                let (mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = plan_manifest_row_hex_tuple_5(v17.clone());
                                let v23: bool = &*v18 == "";
                                if v23 {
                                    let v24: bool = &*v19 == "";
                                    if v24 {
                                        let v25: bool = &*v20 == "";
                                        if v25 {
                                            let v26: bool = &*v21 == "";
                                            if v26 {
                                                let v27: bool = &*v22 == "";
                                                if v27 {
                                                    1i32
                                                } else {
                                                    0i32
                                                }
                                            } else {
                                                0i32
                                            }
                                        } else {
                                            0i32
                                        }
                                    } else {
                                        0i32
                                    }
                                } else {
                                    0i32
                                }
                            } else {
                                0i32
                            }
                        } else {
                            0i32
                        }
                    } else {
                        0i32
                    }
                } else {
                    0i32
                }
            } else {
                0i32
            }
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn plan_provider_count_9() -> i32 {
    8i32
}
fn plan_stage_witness_10() -> i32 {
    1i32
}
fn plan_index_valid_11(mut v0: Rc<str>) -> u64 {
    let v1: i32 = (v0).parse::<u8>().ok().filter(|index| index.to_string() == &*v0).map(|index| index as i32).unwrap_or(-1);
    let v2: bool = 0i32 <= v1;
    let v4: bool = if v2 {
        let v3: bool = v1 < 8i32;
        v3
    } else {
        false
    };
    if v4 {
        1u64
    } else {
        0u64
    }
}
fn plan_decode_code_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> u64 {
    let v5: bool = "create" == &*v0;
    let v7: bool = if v5 {
        let v6: bool = "write" == &*v4;
        v6
    } else {
        false
    };
    if v7 {
        let v8: bool = &*v1 == "";
        let v10: bool = if v8 {
            true
        } else {
            let v9: bool = &*v1 == "-";
            v9
        };
        let v11: bool = v10 == false;
        let v13: bool = if v11 {
            let v12: bool = &*v2 == "-";
            v12
        } else {
            false
        };
        if v13 {
            0u64
        } else {
            8u64
        }
    } else {
        let v15: bool = "replace" == &*v0;
        let v17: bool = if v15 {
            let v16: bool = "write" == &*v4;
            v16
        } else {
            false
        };
        if v17 {
            let v18: bool = &*v1 == "";
            let v20: bool = if v18 {
                true
            } else {
                let v19: bool = &*v1 == "-";
                v19
            };
            let v21: bool = v20 == false;
            let v26: bool = if v21 {
                let v22: bool = &*v2 == "";
                let v24: bool = if v22 {
                    true
                } else {
                    let v23: bool = &*v2 == "-";
                    v23
                };
                let v25: bool = v24 == false;
                v25
            } else {
                false
            };
            if v26 {
                1u64
            } else {
                8u64
            }
        } else {
            let v28: bool = "patch-exact" == &*v0;
            let v30: bool = if v28 {
                let v29: bool = "write" == &*v4;
                v29
            } else {
                false
            };
            if v30 {
                let v31: bool = &*v1 == "";
                let v33: bool = if v31 {
                    true
                } else {
                    let v32: bool = &*v1 == "-";
                    v32
                };
                let v34: bool = v33 == false;
                if v34 {
                    2u64
                } else {
                    8u64
                }
            } else {
                let v36: bool = "mkdir" == &*v0;
                let v38: bool = if v36 {
                    let v37: bool = "write" == &*v4;
                    v37
                } else {
                    false
                };
                if v38 {
                    let v39: bool = &*v1 == "";
                    let v41: bool = if v39 {
                        true
                    } else {
                        let v40: bool = &*v1 == "-";
                        v40
                    };
                    let v42: bool = v41 == false;
                    let v44: bool = if v42 {
                        let v43: bool = &*v2 == "-";
                        v43
                    } else {
                        false
                    };
                    let v46: bool = if v44 {
                        let v45: bool = &*v3 == "-";
                        v45
                    } else {
                        false
                    };
                    if v46 {
                        3u64
                    } else {
                        8u64
                    }
                } else {
                    let v48: bool = "toolchain" == &*v0;
                    let v50: bool = if v48 {
                        let v49: bool = "toolchain" == &*v4;
                        v49
                    } else {
                        false
                    };
                    if v50 {
                        let v51: bool = &*v1 == "";
                        let v53: bool = if v51 {
                            true
                        } else {
                            let v52: bool = &*v1 == "-";
                            v52
                        };
                        let v54: bool = v53 == false;
                        let v56: bool = if v54 {
                            let v55: bool = &*v2 == "-";
                            v55
                        } else {
                            false
                        };
                        let v58: bool = if v56 {
                            let v57: bool = &*v3 == "-";
                            v57
                        } else {
                            false
                        };
                        if v58 {
                            4u64
                        } else {
                            8u64
                        }
                    } else {
                        let v60: bool = "copy" == &*v0;
                        let v62: bool = if v60 {
                            let v61: bool = "write" == &*v4;
                            v61
                        } else {
                            false
                        };
                        if v62 {
                            let v63: bool = &*v1 == "";
                            let v65: bool = if v63 {
                                true
                            } else {
                                let v64: bool = &*v1 == "-";
                                v64
                            };
                            let v66: bool = v65 == false;
                            let v71: bool = if v66 {
                                let v67: bool = &*v2 == "";
                                let v69: bool = if v67 {
                                    true
                                } else {
                                    let v68: bool = &*v2 == "-";
                                    v68
                                };
                                let v70: bool = v69 == false;
                                v70
                            } else {
                                false
                            };
                            let v76: bool = if v71 {
                                let v72: bool = &*v3 == "";
                                let v74: bool = if v72 {
                                    true
                                } else {
                                    let v73: bool = &*v3 == "-";
                                    v73
                                };
                                let v75: bool = v74 == false;
                                v75
                            } else {
                                false
                            };
                            if v76 {
                                5u64
                            } else {
                                8u64
                            }
                        } else {
                            let v78: bool = "chmod" == &*v0;
                            let v80: bool = if v78 {
                                let v79: bool = "write" == &*v4;
                                v79
                            } else {
                                false
                            };
                            if v80 {
                                let v81: bool = &*v1 == "";
                                let v83: bool = if v81 {
                                    true
                                } else {
                                    let v82: bool = &*v1 == "-";
                                    v82
                                };
                                let v84: bool = v83 == false;
                                let v89: bool = if v84 {
                                    let v85: bool = &*v2 == "";
                                    let v87: bool = if v85 {
                                        true
                                    } else {
                                        let v86: bool = &*v2 == "-";
                                        v86
                                    };
                                    let v88: bool = v87 == false;
                                    v88
                                } else {
                                    false
                                };
                                let v94: bool = if v89 {
                                    let v90: bool = &*v3 == "";
                                    let v92: bool = if v90 {
                                        true
                                    } else {
                                        let v91: bool = &*v3 == "-";
                                        v91
                                    };
                                    let v93: bool = v92 == false;
                                    v93
                                } else {
                                    false
                                };
                                if v94 {
                                    6u64
                                } else {
                                    8u64
                                }
                            } else {
                                let v96: bool = "symlink" == &*v0;
                                let v98: bool = if v96 {
                                    let v97: bool = "write" == &*v4;
                                    v97
                                } else {
                                    false
                                };
                                if v98 {
                                    let v99: bool = &*v1 == "";
                                    let v101: bool = if v99 {
                                        true
                                    } else {
                                        let v100: bool = &*v1 == "-";
                                        v100
                                    };
                                    let v102: bool = v101 == false;
                                    let v104: bool = if v102 {
                                        let v103: bool = &*v2 == "-";
                                        v103
                                    } else {
                                        false
                                    };
                                    let v109: bool = if v104 {
                                        let v105: bool = &*v3 == "";
                                        let v107: bool = if v105 {
                                            true
                                        } else {
                                            let v106: bool = &*v3 == "-";
                                            v106
                                        };
                                        let v108: bool = v107 == false;
                                        v108
                                    } else {
                                        false
                                    };
                                    if v109 {
                                        7u64
                                    } else {
                                        8u64
                                    }
                                } else {
                                    8u64
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        plan_schema_tag_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        plan_descriptor_by_index_1(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        plan_manifest_header_projection_2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        plan_manifest_row_hex_tuple_5(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        plan_manifest_projection_witness_8()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        plan_provider_count_9()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        plan_stage_witness_10()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        plan_index_valid_11(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>| -> u64 {
        plan_decode_code_12(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_plan_ir_schema(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_plan_ir_descriptor(v0: &str) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure1()(Rc::<str>::from(v0))
}
pub fn kit_plan_ir_header_projection(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
pub fn kit_plan_ir_row_hex_tuple(v0: &str) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure3()(Rc::<str>::from(v0))
}
pub fn kit_plan_ir_projection_witness() -> i32 {
    closure4()()
}
pub fn kit_plan_ir_count() -> i32 {
    closure5()()
}
pub fn kit_plan_ir_stage_witness() -> i32 {
    closure6()()
}
pub fn kit_plan_ir_index_valid(v0: &str) -> u64 {
    closure7()(Rc::<str>::from(v0))
}
pub fn kit_plan_ir_decode_code(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str) -> u64 {
    closure8()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4))
}
