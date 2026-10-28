#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use kit_command_help::{kit_agile_subcommand_known, kit_command_more_usages}; use command_spec_domain::{kit_command_code, kit_command_count, kit_command_descriptor, kit_command_descriptor_named, kit_command_descriptor_json, kit_command_schema_witness, kit_command_proxy_usage, kit_proxy_route_code, kit_proxy_route_witness, kit_help_flag_code, kit_help_flag_witness};
use canonical_plan_ir_domain::{kit_plan_ir_count, kit_plan_ir_decode_code, kit_plan_ir_descriptor, kit_plan_ir_index_valid, kit_plan_ir_projection_witness, kit_plan_ir_schema, kit_plan_ir_stage_witness};
use kit_agile_lease::lease_authority_code;

#[must_use]
fn KitProcessVerbCode() -> i32 {
    if !command_spec_domain::kit_help_request_text().is_empty() { return kit_command_code("help") as i32; }
    if lease_authority_code() != 0 { return 7; }
    let verb = std::env::args().nth(1).unwrap_or_else(|| "status".to_owned());
    let code = kit_command_code(&verb);
    let unknown = kit_command_count(); if code < unknown as u64 { code as i32 } else { unknown }
}

fn kit_authority_summary_for_mask(mask: i32) -> Result<(i32, i32, i32, String), String> {
    let state = authority_state_domain::kit_authority_state(mask, 0);
    if mask < 0 || state < 0 { return Err("typed authority state rejected".to_owned()); }
    let labels = authority_state_domain::kit_authority_blocker_labels("v1");
    let labels = [labels.0, labels.1, labels.2, labels.3, labels.4];
    let mut reasons = Vec::new();
    for (index, label) in labels.into_iter().enumerate() {
        let bit = authority_state_domain::kit_authority_blocker_bit(index as i32, 0);
        if bit <= 0 { return Err("typed authority blocker catalog rejected".to_owned()); }
        if mask & bit != 0 { reasons.push(label.to_string()); }
    }
    Ok((mask, state, 1, if reasons.is_empty() { "none".to_owned() } else { reasons.join(",") }))
}

fn kit_authority_summary() -> Result<(i32, i32, i32, String), String> {
    kit_authority_summary_for_mask(authority_state_domain::kit_authority_declared_mask(0, 0))
}

#[must_use]
fn KitStatusOk(migration: i32) -> i32 {
    let (mask, state, operational_minus_fallback, reasons) = match kit_authority_summary() {
        Ok(value) => value,
        Err(error) => { eprintln!("kit error: authority state: {error}"); return 2; }
    };
    let authority = if state == 1 { "exclusive" } else { "blocked" };
    let operational = i32::from(operational_minus_fallback == 1);
    println!(
        "kit status ok migration={migration}/1000 public_verbs=5/5 spiral_state=true spiral_first=true authority_formal={authority} authority_blocker_mask={mask} authority_blockers={reasons} operational_replacement={operational} legacy_fallback_execution=false"
    );
    0
}

#[must_use]
fn KitStatusLive(migration: i32) -> i32 {
    let root_text = std::env::args().nth(2).unwrap_or_default();
    let root = std::path::Path::new(&root_text);
    let evidence = match evidence_currentness_domain::evidence_status(root) {
        Ok(value) => value,
        Err(error) => { eprintln!("kit error: live status evidence: {error}"); return 2; }
    };
    let semantic_current = match semantic_evidence_domain::verify_closeout_registry(root, migration) {
        Ok(_) => 1i32,
        Err(error) => { eprintln!("kit error: live status semantic closeout: {error}"); return 2; }
    };
    let cold_current = match cold_proof_domain::verify_formal_authority(root) {
        Ok(_) => 30i32,
        Err(error) => { eprintln!("kit error: live status ColdProofV4, differential catalog or family contracts: {error}"); return 2; }
    };
    let proof_mask = authority_state_domain::kit_authority_live_mask(semantic_current + cold_current, 0); let declared_mask = authority_state_domain::kit_authority_declared_mask(0, 0); let live_mask = if proof_mask < 0 || declared_mask < 0 { -1 } else { proof_mask | declared_mask };
    let (authority_mask, authority_state, operational_minus_fallback, authority_reasons) = match kit_authority_summary_for_mask(live_mask) {
        Ok(value) => value,
        Err(error) => { eprintln!("kit error: live status authority: {error}"); return 2; }
    };
    let authority_mode = if authority_state == 1 { "exclusive" } else { "blocked" };
    let operational_replacement = i32::from(operational_minus_fallback == 1);
    let toolchain_text = match evidence_currentness_domain::toolchain_status(root) {
        Ok(Some(value)) => format!("toolchain=current toolchain_declared={} toolchain_observed={} toolchain_verified={}", value.declared, value.observed, value.verified),
        Ok(None) => "toolchain=absent".to_owned(),
        Err(error) => { eprintln!("kit error: live status Rust toolchain: {error}"); return 2; }
    };
    match kit_compiler_discovery::resolve_spiral_compiler(None) {
        Ok(Some(compiler)) => println!("kit status live migration={migration}/1000 spiral_state=true spiral_first=true evidence_declared={} evidence_observed={} evidence_verified={} evidence_current=true {} compiler=available compiler_source={} compiler_sha256={} compiler_entry_sha256={} compiler_facade_sha256={} authority_formal={} authority_blocker_mask={} authority_blockers={} operational_replacement={} legacy_fallback_execution=false", evidence.declared, evidence.observed, evidence.verified, toolchain_text, compiler.source, compiler.sha256, compiler.entry_sha256, compiler.facade_sha256.as_deref().unwrap_or("absent"), authority_mode, authority_mask, authority_reasons, operational_replacement),
        Ok(None) => println!("kit status live migration={migration}/1000 spiral_state=true spiral_first=true evidence_declared={} evidence_observed={} evidence_verified={} evidence_current=true {} compiler=absent authority_formal={} authority_blocker_mask={} authority_blockers={} operational_replacement={} legacy_fallback_execution=false", evidence.declared, evidence.observed, evidence.verified, toolchain_text, authority_mode, authority_mask, authority_reasons, operational_replacement),
        Err(error) => { eprintln!("kit error: live status compiler discovery: {error}"); return 2; }
    }
    0
}

fn kit_public_valid_values() -> String { (0..kit_command_count()).map(|index| kit_command_descriptor(&index.to_string()).0.to_string()).collect::<Vec<_>>().join("|") }
fn kit_actionable_error(reason: &str, intent: &str, usage: &str, valid: &str, corrected: &str) -> i32 { eprintln!("kit error: {reason}\nintent: {intent}\nusage: kit {usage}\nvalid: {valid}\ncorrected: {corrected}"); 2 }

#[must_use]
fn KitStatusArityError() -> i32 { let descriptor = kit_command_descriptor_named("status"); kit_actionable_error("status accepts zero arguments or one bundle root", "status", descriptor.2.as_ref(), "zero arguments|one bundle root", "kit status") }

#[must_use]
fn KitStatusAuthorityError() -> i32 {
    eprintln!("kit error: workspace-state authority rejected");
    2
}

#[must_use]
fn kit_reenter_with_compiler(verb: &str, forwarded: &[&str], compiler: &str) -> i32 {
    let compiler = std::path::Path::new(compiler);
    if !compiler.is_absolute() || !compiler.is_file() {
        eprintln!("kit error: Spiral compiler must be an absolute regular file: {}", compiler.display());
        return 2;
    }
    let executable = match std::env::current_exe() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("kit error: locate current executable: {error}");
            return 2;
        }
    };
    let fallback_timeout_ms = std::env::var("KIT_REENTER_TIMEOUT_MS").ok().and_then(|value| value.parse::<u64>().ok()).filter(|value| *value > 0).unwrap_or(300_000);
    let timeout_ms = forwarded.get(1).and_then(|root| {
        let root = std::path::Path::new(root);
        let (_, _, wrap_at_unix_ms, _, should_wrap, _) = kit_agile_lease::agile_prompt_lease_clock(root).ok()?;
        if should_wrap { return Some(1); }
        let now_unix_ms = kit_agile_lease::agile_now_unix_ms().ok()?;
        Some(wrap_at_unix_ms.saturating_sub(now_unix_ms).max(1))
    }).unwrap_or(fallback_timeout_ms);
    let mut command = std::process::Command::new(executable);
    command.arg(verb)
        .args(forwarded)
        .env("KIT_SPIRAL_COMPILE", compiler)
        .env("KIT_PATCH_COMPILE_REQUIRED", "1");
    match kit_process_observation::run_bounded_streaming_receipted_observed(&mut command, timeout_ms, "cli-reenter-with-compiler", None) {
        Ok(observation) => observation.status.code().unwrap_or(2),
        Err(error) => {
            eprintln!("kit error: reenter {verb}: {error}");
            2
        }
    }
}

#[must_use]
fn KitPatchAdapter() -> i32 {
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    let subcommand = args.first().map(String::as_str); if !matches!(subcommand, Some("apply" | "check" | "rehearse" | "gated")) { let descriptor = kit_command_descriptor_named("patch"); let reason = subcommand.map(|value| format!("unknown patch subcommand: {value}")).unwrap_or_else(|| "patch subcommand required".to_owned()); return kit_actionable_error(&reason, "patch", descriptor.2.as_ref(), descriptor.2.as_ref(), "kit help patch"); }
    if args.len() == 5 && args[0] == "apply" && args[3] == "--compiler" {
        return kit_reenter_with_compiler(
            "patch",
            &[args[0].as_str(), args[1].as_str(), args[2].as_str()],
            args[4].as_str(),
        );
    }
    if args.first().map(String::as_str) == Some("rehearse") {
        return kit_patch_rehearsal::kit_patch_rehearse_run();
    }
    if args.first().map(String::as_str) == Some("gated") {
        return kit_patch_rehearsal::kit_patch_gated_run();
    }
    kit_patch_mutation::kit_patch_run()
}

#[must_use]
fn KitAgileAdapter() -> i32 {
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    let subcommand = args.first().map(String::as_str); if subcommand.map(|value| kit_agile_subcommand_known(value) == 0).unwrap_or(true) { let descriptor = kit_command_descriptor_named("agile"); let reason = subcommand.map(|value| format!("unknown agile subcommand: {value}")).unwrap_or_else(|| "agile subcommand required".to_owned()); return kit_actionable_error(&reason, "agile", descriptor.2.as_ref(), descriptor.2.as_ref(), "kit help agile"); }
    if args.len() == 4 && args[0] == "check" && args[2] == "--compiler" {
        return kit_agile_state::agile_check_with_compiler_owned(args[1].as_str(), args[3].as_str());
    }
    kit_agile_mutation::kit_agile_run()
}

#[must_use]
fn KitBundleAdapter() -> i32 {
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    let subcommand = args.first().map(String::as_str); if !matches!(subcommand, Some("growth-baseline" | "growth-receipt" | "check" | "create" | "create-flat" | "verify" | "verify-flat" | "rehydrate" | "retention-plan" | "retention-apply")) { let descriptor = kit_command_descriptor_named("bundle"); let reason = subcommand.map(|value| format!("unknown bundle subcommand: {value}")).unwrap_or_else(|| "bundle subcommand required".to_owned()); return kit_actionable_error(&reason, "bundle", descriptor.2.as_ref(), descriptor.2.as_ref(), "kit help bundle"); }
    match subcommand {
        Some("retention-plan") => kit_proxy_result(kit_bundle_retention::retention_plan_command(&args)),
        Some("retention-apply") => kit_proxy_result(kit_bundle_retention::retention_apply_command(&args)),
        _ => kit_legacy_operations::kit_bundle_run(),
    }
}

fn kit_proxy_result(result: Result<(), String>) -> i32 {
    if let Err(error) = result {
        eprintln!("kit error: {error}");
        2
    } else {
        0
    }
}

fn kit_plan_ir_inspect(args: &[String]) -> Result<(), String> {
    if kit_plan_ir_stage_witness() != 1 { return Err("typed Plan IR stage witness rejected".to_owned()); }
    if kit_plan_ir_count() <= 0 { return Err("typed Plan IR provider is empty".to_owned()); }
    if args.len() != 2 { return Err("plan-ir-inspect expects one index".to_owned()); }
    let index = &args[1];
    if kit_plan_ir_index_valid(index) != 1 { return Err(format!("invalid Plan IR index: {index}")); }
    let numeric_index = index.parse::<u64>().map_err(|_| "Plan IR index must be integer".to_owned())?;
    let descriptor = kit_plan_ir_descriptor(index);
    if kit_plan_ir_decode_code(descriptor.0.as_ref(), descriptor.1.as_ref(), descriptor.2.as_ref(), descriptor.3.as_ref(), descriptor.4.as_ref()) != numeric_index { return Err(format!("canonical Plan IR descriptor rejected: {index}")); }
    println!("schema={} index={} op={} target={} expected={} payload={} effect={}", kit_plan_ir_schema("schema"), index, descriptor.0, descriptor.1, descriptor.2, descriptor.3, descriptor.4);
    Ok(())
}

fn kit_plan_ir_check(args: &[String]) -> Result<(), String> {
    if args.len() != 1 { return Err("plan-ir-check accepts no extra arguments".to_owned()); }
    if kit_plan_ir_stage_witness() != 1 { return Err("typed Plan IR stage witness rejected".to_owned()); }
    if kit_plan_ir_projection_witness() != 1 { return Err("typed Plan IR manifest projection witness rejected".to_owned()); }
    let count = kit_plan_ir_count();
    if count <= 0 { return Err("typed Plan IR provider is empty".to_owned()); }
    for index in 0..count {
        let key = index.to_string();
        let descriptor = kit_plan_ir_descriptor(&key);
        if kit_plan_ir_decode_code(descriptor.0.as_ref(), descriptor.1.as_ref(), descriptor.2.as_ref(), descriptor.3.as_ref(), descriptor.4.as_ref()) != index as u64 { return Err(format!("canonical Plan IR descriptor rejected: {key}")); }
    }
    println!("kit plan-ir check ok schema={} count={} stage_witness=1 projection_witness=1", kit_plan_ir_schema("schema"), count);
    Ok(())
}

#[must_use]
fn KitProxyAdapter() -> i32 {
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    if args.is_empty() { let descriptor = kit_command_descriptor_named("proxy"); return kit_actionable_error("proxy capability required", "proxy capability", descriptor.2.as_ref(), descriptor.2.as_ref(), "kit help proxy"); }
    if kit_proxy_route_witness() != 1 { return kit_actionable_error("typed proxy route witness rejected", "proxy capability", "proxy <capability>", "typed proxy route catalog", "kit help proxy"); }
    let route = kit_proxy_route_code(args[0].as_str());
    match route {
        0 => kit_proxy_result(kit_plan_ir_inspect(&args)),
        1 => kit_proxy_result(kit_plan_ir_check(&args)),
        2 => kit_proxy_result(kit_release_hygiene::prune_compiler_sidecars(&args)),
        3 => kit_proxy_result(kit_release_hygiene::incident_recovery_run(&args)),
        4 => kit_proxy_result(bundle_retention_domain::external_payload_run(&args)),
        5 => kit_proxy_result(kit_coverage_prune::coverage_export(&args)),
        6 => kit_proxy_result(kit_coverage_prune::coverage_union(&args)),
        7 => kit_proxy_result(legacy_capability_domain::legacy_surface_run(&args)),
        8 => kit_proxy_result(legacy_capability_domain::legacy_install_self_run(&args)),
        9 => kit_proxy_result(legacy_capability_domain::legacy_self_upgrade_check_run(&args)),
        10 => kit_proxy_result(kit_legacy_operations::legacy_restart_baton_run(&args)),
        11 => kit_proxy_result(kit_patch_driver_domain::source_authoring_run(&args)),
        12 => kit_proxy_result(kit_batch_write::filesystem_batch_write_run(&args)),
        13 => kit_proxy_result(kit_fs_actions::filesystem_actions_run(&args)),
        14 => kit_proxy_result(kit_handoff::binary_install_run(&args)),
        15 => kit_proxy_result(kit_handoff::release_closeout_run(&args)),
        16 => kit_proxy_result(kit_legacy_operations::dogfood_shim_proxy(&args)),
        17 => kit_proxy_result(legacy_capability_domain::legacy_chmod_run(&args)),
        18 => kit_proxy_result(legacy_capability_domain::legacy_symlink_run(&args)),
        19 => kit_proxy_result(legacy_capability_domain::legacy_copy_tree_run(&args)),
        20 => kit_proxy_result(legacy_capability_domain::legacy_command_capture_run(&args)),
        21 => kit_proxy_result(process_batch_plan_domain::command_capture_env_run(&args)),
        22 => kit_proxy_result(process_batch_plan_domain::command_capture_matrix_run(&args)),
        23 => kit_proxy_result(process_batch_plan_domain::command_capture_matrix_gate_run(&args)),
        24 => kit_proxy_result(process_batch_plan_domain::batch_plan_run(&args)),
        25 => kit_proxy_result(process_batch_plan_domain::batch_plan_resume_run(&args)),
        26 => kit_proxy_result(kit_toolchain_proxy::portable_toolchain_run(&args)),
        27 => kit_proxy_result(kit_product_diff::product_diff_run(&args)),
        28 => kit_proxy_result(kit_product_diff::source_recovery_run(&args)),
        29 => kit_proxy_result(kit_proxy_search::proxy_search(&args)),
        30 => kit_proxy_result(kit_bundle_manifest::archive_extract_generic_run(&args)),
        31 => kit_proxy_result(archive_deflate_domain::archive_extract_run(&args)),
        _ => kit_legacy_operations::kit_proxy_run(),
    }
}

#[must_use]
fn KitHelpRejected() -> i32 {
    let request = command_spec_domain::kit_help_request_text();
    if !request.is_empty() { println!("{request}"); return 0; }
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    let count = kit_command_count();
    if count <= 0 { eprintln!("kit error: typed command catalog is empty"); return 2; }
    if args.len() == 2 && kit_command_code(&args[0]) == 4 {
        let usage = kit_command_proxy_usage(&args[1]);
        if usage.is_empty() { let valid = kit_command_descriptor_named("proxy").2; return kit_actionable_error(&format!("unknown proxy help target: {}", args[1]), "proxy help", "help proxy <capability>", valid.as_ref(), "kit help"); }
        println!("usage: kit {}", usage);
        return 0;
    }
    if args.len() > 1 { let descriptor = kit_command_descriptor_named("help"); let valid = format!("{}|proxy <capability>|--completion|--json|--schema", kit_public_valid_values()); return kit_actionable_error("help accepts zero arguments, one verb, proxy <capability>, --completion, --json, or --schema", "help", descriptor.2.as_ref(), &valid, "kit help"); }
    if let Some(target) = args.first() {
        if kit_help_flag_witness() != 1 { eprintln!("kit error: typed help flag witness rejected"); return 2; }
        let help_flag = kit_help_flag_code(target);
        if help_flag == 0 {
            for index in 0..count { let descriptor = kit_command_descriptor(&index.to_string()); println!("{}", descriptor.0); }
            return 0;
        }
        if help_flag == 1 {
            if kit_command_schema_witness() != 1 { eprintln!("kit error: typed command schema witness rejected"); return 2; }
            print!("[");
            for index in 0..count { if index > 0 { print!(","); } print!("{}", kit_command_descriptor_json(&index.to_string())); }
            println!("]");
            return 0;
        }
        if help_flag == 2 {
            if kit_command_schema_witness() != 1 { eprintln!("kit error: typed command schema witness rejected"); return 2; }
            println!("command-spec");
            return 0;
        }
        let code = kit_command_code(target);
        if code >= count as u64 { let proxy_usage = kit_command_proxy_usage(target); if !proxy_usage.is_empty() { let corrected = format!("kit help proxy {target}"); return kit_actionable_error(&format!("help target is a proxy capability: {target}"), "proxy help", "help proxy <capability>", proxy_usage.as_ref(), &corrected); } let valid = kit_public_valid_values(); return kit_actionable_error(&format!("unknown help target: {target}"), "help", "help [verb|--completion|--json|--schema]", &valid, "kit help"); }
        let descriptor = kit_command_descriptor(&code.to_string());
        println!("name={} effect={} handler={}", descriptor.0, descriptor.1, descriptor.4);
        println!("summary={}", descriptor.3);
        println!("usage: kit {}", descriptor.2);
        return 0;
    }
    println!("usage: kit <status|patch|agile|bundle|proxy|help> ...");
    for index in 0..count { let descriptor = kit_command_descriptor(&index.to_string()); println!("{}", descriptor.2); }
    println!("{}", kit_command_more_usages());
    println!("deployment: runtime operations are compilerless; source-author and prune-uncovered are build-authoring operations requiring KIT_SPIRAL_COMPILE");
    0
}

#[must_use]
fn KitUnknownRejected() -> i32 { let verb = std::env::args().nth(1).unwrap_or_default(); let proxy_usage = kit_command_proxy_usage(&verb); if !proxy_usage.is_empty() { let corrected = format!("kit {proxy_usage}"); return kit_actionable_error(&format!("proxy capability used as public verb: {verb}"), "proxy capability", proxy_usage.as_ref(), "proxy <capability>", &corrected); } let valid = kit_public_valid_values(); kit_actionable_error(&format!("unknown verb: {verb}"), "public command", "<status|patch|agile|bundle|proxy|help> ...", &valid, "kit help") }

#[derive(Clone)]
enum US0 {
    US0_DirectArguments,
    US0_FileArguments(Rc<str>),
    US0_StdinArguments,
}
#[derive(Clone)]
enum US2 {
    US2_VerbJob,
    US2_VerbRepo,
    US2_VerbTable,
    US2_VerbRead,
    US2_VerbLines,
    US2_VerbGrep,
    US2_VerbDo,
    US2_VerbPs,
    US2_VerbKill,
    US2_VerbStandards,
    US2_VerbWindow,
    US2_VerbExperiment,
    US2_VerbGuide,
    US2_VerbMachine,
    US2_VerbAdoption,
    US2_VerbMemory,
    US2_VerbResume,
    US2_VerbCi,
    US2_VerbProcess,
    US2_VerbCrate,
    US2_VerbEnv,
    US2_VerbMirror,
    US2_VerbBacklog,
    US2_VerbStory,
    US2_VerbStandup,
    US2_VerbEvidence,
    US2_VerbLint,
}
#[derive(Clone)]
enum US1 {
    US1_DirectKnown(US2),
    US1_DirectUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_StatusHandler,
    US3_PatchHandler,
    US3_AgileHandler,
    US3_BundleHandler,
    US3_ProxyHandler,
    US3_HelpHandler,
    US3_UnknownHandler,
    US3_LeaseDeniedHandler,
}
#[derive(Clone)]
enum US4 {
    US4_StatusSpec,
    US4_PatchSpec,
    US4_AgileSpec,
    US4_BundleSpec,
    US4_ProxySpec,
    US4_HelpSpec,
    US4_UnknownSpec,
}
fn argument_source_0() -> US0 {
    let v0: i32 = 1i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: bool = &*v1 == "--args-from";
    if v2 {
        let v3: i32 = 2i32;
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v3 as usize).unwrap_or_default());
        let v5: bool = &*v4 == "-";
        if v5 {
            US0::US0_StdinArguments
        } else {
            US0::US0_FileArguments(v4)
        }
    } else {
        US0::US0_DirectArguments
    }
}
fn direct_help_1(mut v0: Rc<str>) -> i32 {
    println!("usage: kit {}", &*v0);
    0i32
}
fn lowercase_named_3(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = (v1).to_ascii_lowercase() == &*v0;
    v2
}
fn direct_of_2(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Job"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = lowercase_named_3(v0.clone(), v1.clone());
    if v2 {
        let v3: US2 = US2::US2_VerbJob;
        US1::US1_DirectKnown(v3.clone())
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Repo"); } LIT.with(|lit| lit.clone()) };
        let v6: bool = lowercase_named_3(v0.clone(), v5.clone());
        if v6 {
            let v7: US2 = US2::US2_VerbRepo;
            US1::US1_DirectKnown(v7.clone())
        } else {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Table"); } LIT.with(|lit| lit.clone()) };
            let v10: bool = lowercase_named_3(v0.clone(), v9.clone());
            if v10 {
                let mut v11: US2 = US2::US2_VerbTable;
                US1::US1_DirectKnown(v11.clone())
            } else {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Read"); } LIT.with(|lit| lit.clone()) };
                let v14: bool = lowercase_named_3(v0.clone(), v13.clone());
                if v14 {
                    let v15: US2 = US2::US2_VerbRead;
                    US1::US1_DirectKnown(v15.clone())
                } else {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Lines"); } LIT.with(|lit| lit.clone()) };
                    let v18: bool = lowercase_named_3(v0.clone(), v17.clone());
                    if v18 {
                        let v19: US2 = US2::US2_VerbLines;
                        US1::US1_DirectKnown(v19.clone())
                    } else {
                        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Grep"); } LIT.with(|lit| lit.clone()) };
                        let v22: bool = lowercase_named_3(v0.clone(), v21.clone());
                        if v22 {
                            let v23: US2 = US2::US2_VerbGrep;
                            US1::US1_DirectKnown(v23.clone())
                        } else {
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Do"); } LIT.with(|lit| lit.clone()) };
                            let v26: bool = lowercase_named_3(v0.clone(), v25.clone());
                            if v26 {
                                let v27: US2 = US2::US2_VerbDo;
                                US1::US1_DirectKnown(v27.clone())
                            } else {
                                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Ps"); } LIT.with(|lit| lit.clone()) };
                                let v30: bool = lowercase_named_3(v0.clone(), v29.clone());
                                if v30 {
                                    let v31: US2 = US2::US2_VerbPs;
                                    US1::US1_DirectKnown(v31.clone())
                                } else {
                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Kill"); } LIT.with(|lit| lit.clone()) };
                                    let v34: bool = lowercase_named_3(v0.clone(), v33.clone());
                                    if v34 {
                                        let v35: US2 = US2::US2_VerbKill;
                                        US1::US1_DirectKnown(v35.clone())
                                    } else {
                                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Standards"); } LIT.with(|lit| lit.clone()) };
                                        let v38: bool = lowercase_named_3(v0.clone(), v37.clone());
                                        if v38 {
                                            let v39: US2 = US2::US2_VerbStandards;
                                            US1::US1_DirectKnown(v39.clone())
                                        } else {
                                            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Window"); } LIT.with(|lit| lit.clone()) };
                                            let v42: bool = lowercase_named_3(v0.clone(), v41.clone());
                                            if v42 {
                                                let v43: US2 = US2::US2_VerbWindow;
                                                US1::US1_DirectKnown(v43.clone())
                                            } else {
                                                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Experiment"); } LIT.with(|lit| lit.clone()) };
                                                let v46: bool = lowercase_named_3(v0.clone(), v45.clone());
                                                if v46 {
                                                    let v47: US2 = US2::US2_VerbExperiment;
                                                    US1::US1_DirectKnown(v47.clone())
                                                } else {
                                                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Guide"); } LIT.with(|lit| lit.clone()) };
                                                    let v50: bool = lowercase_named_3(v0.clone(), v49.clone());
                                                    if v50 {
                                                        let v51: US2 = US2::US2_VerbGuide;
                                                        US1::US1_DirectKnown(v51.clone())
                                                    } else {
                                                        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Machine"); } LIT.with(|lit| lit.clone()) };
                                                        let v54: bool = lowercase_named_3(v0.clone(), v53.clone());
                                                        if v54 {
                                                            let v55: US2 = US2::US2_VerbMachine;
                                                            US1::US1_DirectKnown(v55.clone())
                                                        } else {
                                                            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Adoption"); } LIT.with(|lit| lit.clone()) };
                                                            let v58: bool = lowercase_named_3(v0.clone(), v57.clone());
                                                            if v58 {
                                                                let v59: US2 = US2::US2_VerbAdoption;
                                                                US1::US1_DirectKnown(v59.clone())
                                                            } else {
                                                                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Memory"); } LIT.with(|lit| lit.clone()) };
                                                                let v62: bool = lowercase_named_3(v0.clone(), v61.clone());
                                                                if v62 {
                                                                    let v63: US2 = US2::US2_VerbMemory;
                                                                    US1::US1_DirectKnown(v63.clone())
                                                                } else {
                                                                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Resume"); } LIT.with(|lit| lit.clone()) };
                                                                    let v66: bool = lowercase_named_3(v0.clone(), v65.clone());
                                                                    if v66 {
                                                                        let v67: US2 = US2::US2_VerbResume;
                                                                        US1::US1_DirectKnown(v67.clone())
                                                                    } else {
                                                                        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Ci"); } LIT.with(|lit| lit.clone()) };
                                                                        let v70: bool = lowercase_named_3(v0.clone(), v69.clone());
                                                                        if v70 {
                                                                            let v71: US2 = US2::US2_VerbCi;
                                                                            US1::US1_DirectKnown(v71.clone())
                                                                        } else {
                                                                            let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Process"); } LIT.with(|lit| lit.clone()) };
                                                                            let v74: bool = lowercase_named_3(v0.clone(), v73.clone());
                                                                            if v74 {
                                                                                let v75: US2 = US2::US2_VerbProcess;
                                                                                US1::US1_DirectKnown(v75.clone())
                                                                            } else {
                                                                                let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Crate"); } LIT.with(|lit| lit.clone()) };
                                                                                let v78: bool = lowercase_named_3(v0.clone(), v77.clone());
                                                                                if v78 {
                                                                                    let v79: US2 = US2::US2_VerbCrate;
                                                                                    US1::US1_DirectKnown(v79.clone())
                                                                                } else {
                                                                                    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Env"); } LIT.with(|lit| lit.clone()) };
                                                                                    let v82: bool = lowercase_named_3(v0.clone(), v81.clone());
                                                                                    if v82 {
                                                                                        let v83: US2 = US2::US2_VerbEnv;
                                                                                        US1::US1_DirectKnown(v83.clone())
                                                                                    } else {
                                                                                        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Mirror"); } LIT.with(|lit| lit.clone()) };
                                                                                        let v86: bool = lowercase_named_3(v0.clone(), v85.clone());
                                                                                        if v86 {
                                                                                            let v87: US2 = US2::US2_VerbMirror;
                                                                                            US1::US1_DirectKnown(v87.clone())
                                                                                        } else {
                                                                                            let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Backlog"); } LIT.with(|lit| lit.clone()) };
                                                                                            let v90: bool = lowercase_named_3(v0.clone(), v89.clone());
                                                                                            if v90 {
                                                                                                let v91: US2 = US2::US2_VerbBacklog;
                                                                                                US1::US1_DirectKnown(v91.clone())
                                                                                            } else {
                                                                                                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                                                                                                let v94: bool = lowercase_named_3(v0.clone(), v93.clone());
                                                                                                if v94 {
                                                                                                    let v95: US2 = US2::US2_VerbStory;
                                                                                                    US1::US1_DirectKnown(v95.clone())
                                                                                                } else {
                                                                                                    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Standup"); } LIT.with(|lit| lit.clone()) };
                                                                                                    let v98: bool = lowercase_named_3(v0.clone(), v97.clone());
                                                                                                    if v98 {
                                                                                                        let v99: US2 = US2::US2_VerbStandup;
                                                                                                        US1::US1_DirectKnown(v99.clone())
                                                                                                    } else {
                                                                                                        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Evidence"); } LIT.with(|lit| lit.clone()) };
                                                                                                        let v102: bool = lowercase_named_3(v0.clone(), v101.clone());
                                                                                                        if v102 {
                                                                                                            let v103: US2 = US2::US2_VerbEvidence;
                                                                                                            US1::US1_DirectKnown(v103.clone())
                                                                                                        } else {
                                                                                                            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Lint"); } LIT.with(|lit| lit.clone()) };
                                                                                                            let v106: bool = lowercase_named_3(v0.clone(), v105.clone());
                                                                                                            if v106 {
                                                                                                                let v107: US2 = US2::US2_VerbLint;
                                                                                                                US1::US1_DirectKnown(v107.clone())
                                                                                                            } else {
                                                                                                                US1::US1_DirectUnknown
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
fn ceremony_routed_4(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = 2i32;
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
    let v3: bool = kit_ceremonies::kit_ceremony_admits(&*v0, &*v2);
    if v3 {
        let mut v4: i32 = KitAgileAdapter();
        v4
    } else {
        2i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let v0: i32 = 1i32;
        let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let v2: i32 = 2i32;
        let v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
        let mut v4: bool = &*v1 == "help";
        let v7: Rc<str> = if v4 {
            let v5: Rc<str> = command_spec_domain::kit_direct_verb_usage(&*v3);
            v5.clone()
        } else {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        };
        let v8: bool = &*v7 != "";
        if v8 {
            direct_help_1(v7.clone())
        } else {
            let v10: US1 = direct_of_2(v1.clone());
            match &v10 {
                US1::US1_DirectKnown(v11) => {
                    let mut v11: US2 = v11.clone();
                    match &v11 {
                        US2::US2_VerbAdoption => {
                            let v51: i32 = kit_adoption::kit_adoption_run();
                            v51
                        }
                        US2::US2_VerbBacklog => {
                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog"); } LIT.with(|lit| lit.clone()) };
                            ceremony_routed_4(v59.clone())
                        }
                        US2::US2_VerbCi => {
                            let v54: i32 = kit_ci_parity::kit_ci_parity_run();
                            v54
                        }
                        US2::US2_VerbCrate => {
                            let v56: i32 = kit_crate_gate::kit_crate_gate_run();
                            v56
                        }
                        US2::US2_VerbDo => {
                            let v43: i32 = kit_do::kit_do_run();
                            v43
                        }
                        US2::US2_VerbEnv => {
                            let v57: i32 = kit_env_pins::kit_env_run();
                            v57
                        }
                        US2::US2_VerbEvidence => {
                            let v65: i32 = kit_evidence::kit_evidence_run();
                            v65
                        }
                        US2::US2_VerbExperiment => {
                            let v48: i32 = kit_experiments::kit_experiment_run();
                            v48
                        }
                        US2::US2_VerbGrep => {
                            let v42: i32 = kit_grep_query::kit_grep_run();
                            v42
                        }
                        US2::US2_VerbGuide => {
                            let v49: i32 = kit_guide::kit_guide_run();
                            v49
                        }
                        US2::US2_VerbJob => {
                            let v12: i32 = kit_jobs::kit_job_run();
                            v12
                        }
                        US2::US2_VerbKill => {
                            let v45: i32 = kit_process_table::kit_process_run();
                            v45
                        }
                        US2::US2_VerbLines => {
                            let v41: i32 = kit_read_query::kit_read_run();
                            v41
                        }
                        US2::US2_VerbLint => {
                            let v66: i32 = kit_lint_macros::kit_lint_run();
                            v66
                        }
                        US2::US2_VerbMachine => {
                            let v50: i32 = kit_machine_watch::kit_machine_watch_run();
                            v50
                        }
                        US2::US2_VerbMemory => {
                            let v52: i32 = kit_memory::kit_memory_run();
                            v52
                        }
                        US2::US2_VerbMirror => {
                            let v58: i32 = kit_mirror::kit_mirror_run();
                            v58
                        }
                        US2::US2_VerbProcess => {
                            let v55: i32 = kit_os_launch::kit_process_launch_run();
                            v55
                        }
                        US2::US2_VerbPs => {
                            let v44: i32 = kit_process_table::kit_process_run();
                            v44
                        }
                        US2::US2_VerbRead => {
                            let v40: i32 = kit_read_query::kit_read_run();
                            v40
                        }
                        US2::US2_VerbRepo => {
                            let v13: i32 = 2i32;
                            let v14: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v13 as usize).unwrap_or_default());
                            let v15: bool = &*v14 == "affected";
                            if v15 {
                                let v16: i32 = kit_repo_affected::kit_repo_affected_run();
                                v16
                            } else {
                                let v17: i32 = 2i32;
                                let v18: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v17 as usize).unwrap_or_default());
                                let v19: bool = &*v18 == "accept-review";
                                if v19 {
                                    let v20: i32 = kit_repo_accept_review::kit_repo_accept_review_run();
                                    v20
                                } else {
                                    let v21: i32 = kit_repo_state::kit_repo_run();
                                    v21
                                }
                            }
                        }
                        US2::US2_VerbResume => {
                            let v53: i32 = kit_resume::kit_resume_run();
                            v53
                        }
                        US2::US2_VerbStandards => {
                            let v46: i32 = kit_standards::kit_standards_run();
                            v46
                        }
                        US2::US2_VerbStandup => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standup"); } LIT.with(|lit| lit.clone()) };
                            ceremony_routed_4(v63.clone())
                        }
                        US2::US2_VerbStory => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("story"); } LIT.with(|lit| lit.clone()) };
                            ceremony_routed_4(v61.clone())
                        }
                        US2::US2_VerbTable => {
                            let v24: i32 = 2i32;
                            let v25: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v24 as usize).unwrap_or_default());
                            let v26: bool = &*v25 == "logs";
                            if v26 {
                                let v27: i32 = kit_table_logs::kit_table_logs_run();
                                v27
                            } else {
                                let v28: i32 = 2i32;
                                let v29: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v28 as usize).unwrap_or_default());
                                let v30: bool = &*v29 == "sequences";
                                if v30 {
                                    let v31: i32 = kit_table_sequences::kit_table_sequences_run();
                                    v31
                                } else {
                                    let v32: i32 = 2i32;
                                    let v33: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v32 as usize).unwrap_or_default());
                                    let v34: bool = &*v33 == "test";
                                    if v34 {
                                        let v35: i32 = kit_table_test::kit_table_test_run();
                                        v35
                                    } else {
                                        let v36: i32 = kit_table_query::kit_table_run();
                                        v36
                                    }
                                }
                            }
                        }
                        US2::US2_VerbWindow => {
                            let v47: i32 = kit_machine_windows::kit_window_run();
                            v47
                        }
                    }
                }
                US1::US1_DirectUnknown => {
                    let v94: i32 = kit_legacy_operations::dogfood_shim_argv0_code();
                    let v95: bool = 0i32 == v94;
                    if v95 {
                        let v96: i32 = KitProcessVerbCode();
                        let v97: bool = v96 == 7i32;
                        let v136: US3 = if v97 {
                            US3::US3_LeaseDeniedHandler
                        } else {
                            let v99: i32 = v96.wrapping_add(1i32);
                            let v100: bool = v99 == 1i32;
                            let v121: US4 = if v100 {
                                US4::US4_StatusSpec
                            } else {
                                let v102: bool = v99 == 2i32;
                                if v102 {
                                    US4::US4_PatchSpec
                                } else {
                                    let v104: bool = v99 == 3i32;
                                    if v104 {
                                        US4::US4_AgileSpec
                                    } else {
                                        let v106: bool = v99 == 4i32;
                                        if v106 {
                                            US4::US4_BundleSpec
                                        } else {
                                            let v108: bool = v99 == 5i32;
                                            if v108 {
                                                US4::US4_ProxySpec
                                            } else {
                                                let v110: bool = v99 == 6i32;
                                                if v110 {
                                                    US4::US4_HelpSpec
                                                } else {
                                                    US4::US4_UnknownSpec
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            match &v121 {
                                US4::US4_AgileSpec => {
                                    US3::US3_AgileHandler
                                }
                                US4::US4_BundleSpec => {
                                    US3::US3_BundleHandler
                                }
                                US4::US4_HelpSpec => {
                                    US3::US3_HelpHandler
                                }
                                US4::US4_PatchSpec => {
                                    US3::US3_PatchHandler
                                }
                                US4::US4_ProxySpec => {
                                    US3::US3_ProxyHandler
                                }
                                US4::US4_StatusSpec => {
                                    US3::US3_StatusHandler
                                }
                                US4::US4_UnknownSpec => {
                                    US3::US3_UnknownHandler
                                }
                            }
                        };
                        match &v136 {
                            US3::US3_AgileHandler => {
                                let v168: i32 = KitAgileAdapter();
                                v168
                            }
                            US3::US3_BundleHandler => {
                                let v169: i32 = KitBundleAdapter();
                                v169
                            }
                            US3::US3_HelpHandler => {
                                let v171: i32 = KitHelpRejected();
                                v171
                            }
                            US3::US3_LeaseDeniedHandler => {
                                2i32
                            }
                            US3::US3_PatchHandler => {
                                let v167: i32 = KitPatchAdapter();
                                v167
                            }
                            US3::US3_ProxyHandler => {
                                let v170: i32 = KitProxyAdapter();
                                v170
                            }
                            US3::US3_StatusHandler => {
                                let v137: i32 = std::env::args().skip(2).count() as i32;
                                let v138: bool = 0i32 == v137;
                                if v138 {
                                    let v139: i32 = bootstrap_authority_status::kit_bootstrap_workspace_state_status();
                                    let v140: bool = 1i32 == v139;
                                    if v140 {
                                        let v141: i32 = bootstrap_authority_status::kit_bootstrap_compiler_session_status();
                                        let v142: bool = 1i32 == v141;
                                        if v142 {
                                            let v143: i32 = bootstrap_authority_status::kit_bootstrap_legacy_bundle_writer_status();
                                            let v144: bool = 1i32 == v143;
                                            if v144 {
                                                let v145: i32 = bootstrap_authority_status::kit_bootstrap_ingest_frontier_status();
                                                let v146: bool = 1i32 == v145;
                                                if v146 {
                                                    let v147: i32 = process_batch_plan_domain::kit_batch_matrix_auto_chain_budget(60000, 16000);
                                                    let v148: bool = 1i32 == v147;
                                                    if v148 {
                                                        let v149: i32 = bootstrap_authority_status::kit_bootstrap_settled_migration_status();
                                                        let v150: i32 = KitStatusOk(v149);
                                                        v150
                                                    } else {
                                                        let v151: i32 = KitStatusAuthorityError();
                                                        v151
                                                    }
                                                } else {
                                                    let v153: i32 = KitStatusAuthorityError();
                                                    v153
                                                }
                                            } else {
                                                let v155: i32 = KitStatusAuthorityError();
                                                v155
                                            }
                                        } else {
                                            let v157: i32 = KitStatusAuthorityError();
                                            v157
                                        }
                                    } else {
                                        let v159: i32 = KitStatusAuthorityError();
                                        v159
                                    }
                                } else {
                                    let v161: bool = 1i32 == v137;
                                    if v161 {
                                        let v162: i32 = bootstrap_authority_status::kit_bootstrap_settled_migration_status();
                                        let v163: i32 = KitStatusLive(v162);
                                        v163
                                    } else {
                                        let v164: i32 = KitStatusArityError();
                                        v164
                                    }
                                }
                            }
                            US3::US3_UnknownHandler => {
                                let v172: i32 = KitUnknownRejected();
                                v172
                            }
                        }
                    } else {
                        let v181: i32 = kit_legacy_operations::dogfood_shim_dispatch();
                        v181
                    }
                }
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn run_from_5(mut v0: US0, mut v1: Rc<dyn Fn() -> i32>) -> i32 {
    #[cfg(windows)] { unsafe extern "system" { fn SetHandleInformation(handle: *mut std::ffi::c_void, mask: u32, flags: u32) -> i32; } use std::os::windows::io::AsRawHandle; unsafe { SetHandleInformation(std::io::stdin().as_raw_handle(), 1, 0); SetHandleInformation(std::io::stdout().as_raw_handle(), 1, 0); SetHandleInformation(std::io::stderr().as_raw_handle(), 1, 0); } };
    match &v0 {
        US0::US0_DirectArguments => {
            v1()
        }
        US0::US0_FileArguments(v3) => {
            let v3: Rc<str> = v3.clone();
            let mut v4: Result<std::rc::Rc<str>, std::rc::Rc<str>> = std::fs::read_to_string(&*v3).map(std::rc::Rc::<str>::from).map_err(|error| std::rc::Rc::<str>::from(format!("--args-from {}: {error}", &*v3)));
            let v5: bool = v4.is_ok();
            if v5 {
                let v6: Rc<str> = v4.clone().unwrap_or_else(|error| error);
                let v7: i32 = { let arguments = v6.lines().map(|line| line.trim_end_matches(char::from(13u8)).to_owned()).chain(std::env::args().skip(3)).collect::<Vec<_>>(); match std::env::current_exe().and_then(|exe| std::process::Command::new(exe).args(&arguments).status()) { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("kit error: --args-from relaunch: {error}"); 2 } } };
                v7
            } else {
                let v8: Rc<str> = v4.clone().unwrap_or_else(|error| error);
                let v9: i32 = { eprintln!("kit error: {}", &*v8); 2 };
                v9
            }
        }
        US0::US0_StdinArguments => {
            let mut v11: Result<std::rc::Rc<str>, std::rc::Rc<str>> = { let mut text = String::new(); std::io::Read::read_to_string(&mut std::io::stdin(), &mut text).map(|_| std::rc::Rc::<str>::from(text)).map_err(|error| std::rc::Rc::<str>::from(format!("--args-from -: {error}"))) };
            let v12: bool = v11.is_ok();
            if v12 {
                let v13: Rc<str> = v11.clone().unwrap_or_else(|error| error);
                let v14: i32 = { let arguments = v13.lines().map(|line| line.trim_end_matches(char::from(13u8)).to_owned()).chain(std::env::args().skip(3)).collect::<Vec<_>>(); match std::env::current_exe().and_then(|exe| std::process::Command::new(exe).args(&arguments).status()) { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("kit error: --args-from relaunch: {error}"); 2 } } };
                v14
            } else {
                let v15: Rc<str> = v11.clone().unwrap_or_else(|error| error);
                let v16: i32 = { eprintln!("kit error: {}", &*v15); 2 };
                v16
            }
        }
    }
}
fn spiral_main() -> i32 {
    std::panic::set_hook(Box::new(|info| { if info.to_string().contains("failed printing to stdout") { std::process::exit(0) } eprintln!("{info}") }));
    let v0: US0 = argument_source_0();
    let mut v1: Rc<dyn Fn() -> i32> = closure0();
    let v2: i32 = run_from_5(v0.clone(), v1.clone());
    let v3: i32 = kit_coverage_prune::coverage_finish(v2);
    v3
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
