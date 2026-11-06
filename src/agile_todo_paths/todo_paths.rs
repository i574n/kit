#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_CFamily,
    US1_HashFamily,
    US1_DashFamily,
    US1_HaskellFamily,
    US1_LeanFamily,
    US1_LispFamily,
    US1_ErlangFamily,
    US1_MarkupFamily,
    US1_MlFamily,
    US1_PascalFamily,
    US1_BatchFamily,
    US1_FsharpFamily,
}
#[derive(Clone)]
enum US2 {
    US2_SkipNestedRepository,
    US2_SkipVendorDirectory,
    US2_SkipUnknownSyntax,
    US2_SkipGeneratedOutput,
    US2_SkipLinkedEntry,
}
#[derive(Clone)]
enum US0 {
    US0_ScanPath(US1),
    US0_SkipPath(US2),
}
#[derive(Clone)]
enum US3 {
    US3_KnownSyntax(US1),
    US3_UnknownSyntax,
}
fn vendor_segment_from_2(mut v0: Rc<str>, mut v1: i32) -> bool {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v3: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return false;
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules|vendor|third_party|target|dist|_build|.zig-cache|zig-out|.lake|__pycache__|.venv|venv|.gradle|.next|bin|obj|deps|.git|.cache|elm-stuff|bower_components|.dart_tool|.mypy_cache|.pytest_cache|.tox"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v3 as usize).unwrap_or(""));
            let mut v7: bool = v5.split("|").any(|item| item == &*v6);
            if v7 {
                return true;
            } else {
                let v8: i32 = v3.wrapping_add(1i32);
                v1 = v8;
                continue;
            }
        }
    }
}
fn last_offset_of_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v4 <= v2;
        if v5 {
            return v3;
        } else {
            let v6: i32 = v0.as_bytes().get(v2 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v7: bool = v6 == v1;
            if v7 {
                let v8: i32 = v2.wrapping_add(1i32);
                (v2, v3) = (v8, v2);
                continue;
            } else {
                let v10: i32 = v2.wrapping_add(1i32);
                v2 = v10;
                continue;
            }
        }
    }
}
fn syntax_of_path_3(mut v0: Rc<str>) -> US3 {
    let mut v1: i32 = 47i32;
    let mut v2: i32 = 0i32;
    let v3: i32 = -1i32;
    let v4: i32 = last_offset_of_4(v0.clone(), v1, v2, v3);
    let v5: i32 = v4.wrapping_add(1i32);
    let v6: i32 = (v0.clone().len() as i32);
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v6 as usize).unwrap_or(""));
    let v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
    let v9: i32 = 46i32;
    let v10: i32 = 0i32;
    let v11: i32 = -1i32;
    let v12: i32 = last_offset_of_4(v8.clone(), v9, v10, v11);
    let v13: bool = v12 < 0i32;
    let v18: Rc<str> = if v13 {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v14
    } else {
        let v15: i32 = v12.wrapping_add(1i32);
        let v16: i32 = (v8.clone().len() as i32);
        let v17: Rc<str> = std::rc::Rc::<str>::from(v8.get(v15 as usize..v16 as usize).unwrap_or(""));
        v17
    };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spi|spir|rs|c|h|cc|cpp|cxx|hpp|hh|cs|js|mjs|cjs|ts|tsx|jsx|go|java|kt|kts|swift|zig|gleam|scala|dart|wgsl|glsl|hlsl|proto|jsonc|css|scss|less|php|sv|v"); } LIT.with(|lit| lit.clone()) };
    let v20: bool = &*v18 == "";
    let mut v21: bool = v20 == false;
    let v22: bool = v19.split("|").any(|item| item == &*v18);
    let v23: bool = v21 && v22;
    if v23 {
        let v24: US1 = US1::US1_CFamily;
        US3::US3_KnownSyntax(v24)
    } else {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps1|psm1|psd1|py|pyi|sh|bash|zsh|fish|rb|toml|yml|yaml|ex|exs|r|pl|pm|cmake|nix|conf|cfg|mk|tf|jl|nu|ini|gitignore|gitattributes|editorconfig|dockerignore"); } LIT.with(|lit| lit.clone()) };
        let v27: bool = v26.split("|").any(|item| item == &*v18);
        let v28: bool = v21 && v27;
        if v28 {
            let v29: US1 = US1::US1_HashFamily;
            US3::US3_KnownSyntax(v29)
        } else {
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("makefile|dockerfile|justfile|gemfile|rakefile|vagrantfile"); } LIT.with(|lit| lit.clone()) };
            let v32: bool = v31.split("|").any(|item| item == &*v8);
            if v32 {
                let v33: US1 = US1::US1_HashFamily;
                US3::US3_KnownSyntax(v33)
            } else {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lua|sql|ada|adb|ads|vhd|vhdl"); } LIT.with(|lit| lit.clone()) };
                let v36: bool = v35.split("|").any(|item| item == &*v18);
                let v37: bool = v21 && v36;
                if v37 {
                    let v38: US1 = US1::US1_DashFamily;
                    US3::US3_KnownSyntax(v38)
                } else {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hs|lhs|elm|purs|agda|idr"); } LIT.with(|lit| lit.clone()) };
                    let v41: bool = v40.split("|").any(|item| item == &*v18);
                    let v42: bool = v21 && v41;
                    if v42 {
                        let v43: US1 = US1::US1_HaskellFamily;
                        US3::US3_KnownSyntax(v43)
                    } else {
                        let v45: bool = &*v18 == "lean";
                        if v45 {
                            let v46: US1 = US1::US1_LeanFamily;
                            US3::US3_KnownSyntax(v46)
                        } else {
                            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("clj|cljs|cljc|edn|lisp|lsp|el|scm|ss|rkt|asm|nasm"); } LIT.with(|lit| lit.clone()) };
                            let v49: bool = v48.split("|").any(|item| item == &*v18);
                            let v50: bool = v21 && v49;
                            if v50 {
                                let v51: US1 = US1::US1_LispFamily;
                                US3::US3_KnownSyntax(v51)
                            } else {
                                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("erl|hrl|tex|sty|bib"); } LIT.with(|lit| lit.clone()) };
                                let v54: bool = v53.split("|").any(|item| item == &*v18);
                                let v55: bool = v21 && v54;
                                if v55 {
                                    let v56: US1 = US1::US1_ErlangFamily;
                                    US3::US3_KnownSyntax(v56)
                                } else {
                                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("md|markdown|livemd|html|htm|xhtml|xml|svg|xaml|vue|svelte|csproj|fsproj|props|targets|plist"); } LIT.with(|lit| lit.clone()) };
                                    let v59: bool = v58.split("|").any(|item| item == &*v18);
                                    let v60: bool = v21 && v59;
                                    if v60 {
                                        let v61: US1 = US1::US1_MarkupFamily;
                                        US3::US3_KnownSyntax(v61)
                                    } else {
                                        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ml|mli|sml"); } LIT.with(|lit| lit.clone()) };
                                        let v64: bool = v63.split("|").any(|item| item == &*v18);
                                        let v65: bool = v21 && v64;
                                        if v65 {
                                            let v66: US1 = US1::US1_MlFamily;
                                            US3::US3_KnownSyntax(v66)
                                        } else {
                                            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pas|pp|dpr|dpk|lpr|inc"); } LIT.with(|lit| lit.clone()) };
                                            let v69: bool = v68.split("|").any(|item| item == &*v18);
                                            let v70: bool = v21 && v69;
                                            if v70 {
                                                let v71: US1 = US1::US1_PascalFamily;
                                                US3::US3_KnownSyntax(v71)
                                            } else {
                                                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bat|cmd"); } LIT.with(|lit| lit.clone()) };
                                                let v74: bool = v73.split("|").any(|item| item == &*v18);
                                                let v75: bool = v21 && v74;
                                                if v75 {
                                                    let v76: US1 = US1::US1_BatchFamily;
                                                    US3::US3_KnownSyntax(v76)
                                                } else {
                                                    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs|fsx|fsi"); } LIT.with(|lit| lit.clone()) };
                                                    let v79: bool = v78.split("|").any(|item| item == &*v18);
                                                    let v80: bool = v21 && v79;
                                                    if v80 {
                                                        let v81: US1 = US1::US1_FsharpFamily;
                                                        US3::US3_KnownSyntax(v81)
                                                    } else {
                                                        US3::US3_UnknownSyntax
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn path_verdict_1(mut v0: Rc<str>, mut v1: bool) -> US0 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v0.ends_with(&*v2);
    if v3 {
        let v4: US2 = US2::US2_SkipNestedRepository;
        US0::US0_SkipPath(v4)
    } else {
        let v6: i32 = 0i32;
        let mut v7: bool = vendor_segment_from_2(v0.clone(), v6);
        if v7 {
            let v8: US2 = US2::US2_SkipVendorDirectory;
            US0::US0_SkipPath(v8)
        } else {
            let v10: US3 = syntax_of_path_3(v0.clone());
            match v10 {
                US3::US3_KnownSyntax(v13) => {
                    let v13: US1 = v13;
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spi|spir"); } LIT.with(|lit| lit.clone()) };
                    let v15: i32 = 47i32;
                    let v16: i32 = 0i32;
                    let v17: i32 = -1i32;
                    let v18: i32 = last_offset_of_4(v0.clone(), v15, v16, v17);
                    let v19: i32 = v18.wrapping_add(1i32);
                    let v20: i32 = (v0.clone().len() as i32);
                    let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v20 as usize).unwrap_or(""));
                    let v22: Rc<str> = std::rc::Rc::<str>::from(v21.to_ascii_lowercase());
                    let v23: i32 = 46i32;
                    let v24: i32 = 0i32;
                    let v25: i32 = -1i32;
                    let v26: i32 = last_offset_of_4(v22.clone(), v23, v24, v25);
                    let v27: bool = v26 < 0i32;
                    let v32: Rc<str> = if v27 {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v28
                    } else {
                        let v29: i32 = v26.wrapping_add(1i32);
                        let v30: i32 = (v22.clone().len() as i32);
                        let v31: Rc<str> = std::rc::Rc::<str>::from(v22.get(v29 as usize..v30 as usize).unwrap_or(""));
                        v31
                    };
                    let v33: bool = v14.split("|").any(|item| item == &*v32);
                    let v34: bool = v33 == false;
                    let v35: bool = v1 && v34;
                    if v35 {
                        let v36: US2 = US2::US2_SkipGeneratedOutput;
                        US0::US0_SkipPath(v36)
                    } else {
                        US0::US0_ScanPath(v13)
                    }
                }
                US3::US3_UnknownSyntax => {
                    let v11: US2 = US2::US2_SkipUnknownSyntax;
                    US0::US0_SkipPath(v11)
                }
            }
        }
    }
}
fn todo_path_verdict_0(mut v0: Rc<str>, mut v1: bool) -> i32 {
    let mut v2: US0 = path_verdict_1(v0, v1);
    match v2 {
        US0::US0_ScanPath(v3) => {
            let v3: US1 = v3;
            match v3 {
                US1::US1_BatchFamily => {
                    11i32
                }
                US1::US1_CFamily => {
                    1i32
                }
                US1::US1_DashFamily => {
                    3i32
                }
                US1::US1_ErlangFamily => {
                    7i32
                }
                US1::US1_FsharpFamily => {
                    12i32
                }
                US1::US1_HashFamily => {
                    2i32
                }
                US1::US1_HaskellFamily => {
                    4i32
                }
                US1::US1_LeanFamily => {
                    5i32
                }
                US1::US1_LispFamily => {
                    6i32
                }
                US1::US1_MarkupFamily => {
                    8i32
                }
                US1::US1_MlFamily => {
                    9i32
                }
                US1::US1_PascalFamily => {
                    10i32
                }
            }
        }
        US0::US0_SkipPath(v5) => {
            let v5: US2 = v5;
            let v6: i32 = match v5 {
                US2::US2_SkipGeneratedOutput => {
                    4i32
                }
                US2::US2_SkipLinkedEntry => {
                    5i32
                }
                US2::US2_SkipNestedRepository => {
                    1i32
                }
                US2::US2_SkipUnknownSyntax => {
                    3i32
                }
                US2::US2_SkipVendorDirectory => {
                    2i32
                }
            };
            let mut v7: i32 = v6.wrapping_neg();
            v7
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, bool) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, bool) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: bool| -> i32 {
        todo_path_verdict_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_todo_path_verdict(v0: &str, v1: bool) -> i32 {
    closure0()(Rc::<str>::from(v0), v1)
}
