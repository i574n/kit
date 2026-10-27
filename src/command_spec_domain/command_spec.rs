#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_NoHelpRequested,
    US0_VerbHelp(Rc<str>),
    US0_CapabilityHelp(Rc<str>),
}
fn command_code_0(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--help|-h"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v1.split("|").any(|item| item == &*v0);
    if v2 {
        let v3: u64 = (6i32 - 1) as u64;
        v3
    } else {
        let v4: u64 = u64::MAX;
        let v5: bool = "status" == &*v0;
        if v5 {
            let v6: u64 = (1i32 - 1) as u64;
            v6
        } else {
            let v7: bool = "patch" == &*v0;
            if v7 {
                let v8: u64 = (2i32 - 1) as u64;
                v8
            } else {
                let v9: bool = "agile" == &*v0;
                if v9 {
                    let v10: u64 = (3i32 - 1) as u64;
                    v10
                } else {
                    let v11: bool = "bundle" == &*v0;
                    if v11 {
                        let v12: u64 = (4i32 - 1) as u64;
                        v12
                    } else {
                        let v13: bool = "proxy" == &*v0;
                        if v13 {
                            let v14: u64 = (5i32 - 1) as u64;
                            v14
                        } else {
                            let v15: bool = "help" == &*v0;
                            if v15 {
                                let v16: u64 = (6i32 - 1) as u64;
                                v16
                            } else {
                                v4
                            }
                        }
                    }
                }
            }
        }
    }
}
fn command_effect_code_1(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (1i32 - 1) as u64;
    let v2: bool = "status" == &*v0;
    if v2 {
        let v3: u64 = (1i32 - 1) as u64;
        v3
    } else {
        let v4: bool = "patch" == &*v0;
        if v4 {
            let v5: u64 = (3i32 - 1) as u64;
            v5
        } else {
            let v6: bool = "agile" == &*v0;
            if v6 {
                let v7: u64 = (3i32 - 1) as u64;
                v7
            } else {
                let v8: bool = "bundle" == &*v0;
                if v8 {
                    let v9: u64 = (2i32 - 1) as u64;
                    v9
                } else {
                    let v10: bool = "proxy" == &*v0;
                    if v10 {
                        let v11: u64 = (3i32 - 1) as u64;
                        v11
                    } else {
                        let v12: bool = "help" == &*v0;
                        if v12 {
                            let v13: u64 = (1i32 - 1) as u64;
                            v13
                        } else {
                            v1
                        }
                    }
                }
            }
        }
    }
}
fn command_subcommand_effect_code_2(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch:check|proxy:text-replace:check|proxy:fs-batch-write:preview|proxy:fs-actions:preview|proxy:source-author:preview|agile:list|agile:show|agile:todos|agile:next|agile:migrate-tags:preview|agile:plan|agile:check|agile:lease|agile:status|agile:handoff|proxy:source-topology/1|proxy:plan-ir-inspect|proxy:plan-ir-check|proxy:legacy-surface|proxy:self-upgrade-check|proxy:fs-list|proxy:fs-read|proxy:fs-slice|proxy:fs-context|proxy:fs-search|proxy:parallel-search|proxy:parallel-manifest|proxy:product-diff|proxy:hash|proxy:hash-tree|proxy:source-stats|proxy:ingest-map|proxy:ingest-trim-check|proxy:ingest-extracted-manifest|proxy:ingest-targets|proxy:ingest-targets-derive|proxy:frontier-map|proxy:spi-map|proxy:spi-diff|proxy:source-recovery-diff|proxy:coverage-assess"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v1.split("|").any(|item| item == &*v0);
    if v2 {
        let v3: u64 = (1i32 - 1) as u64;
        v3
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile:record|proxy:release-closeout|proxy:spiral-session-shutdown"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v4.split("|").any(|item| item == &*v0);
        if v5 {
            let v6: u64 = (2i32 - 1) as u64;
            v6
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile:begin|agile:init"); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v7.split("|").any(|item| item == &*v0);
            if v8 {
                let v9: u64 = (4i32 - 1) as u64;
                v9
            } else {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bundle:retention-apply"); } LIT.with(|lit| lit.clone()) };
                let v11: bool = v0 == v10 ;
                if v11 {
                    let v12: u64 = (3i32 - 1) as u64;
                    v12
                } else {
                    4u64
                }
            }
        }
    }
}
fn command_descriptor_by_index_3(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: i32 = (v0).parse::<u8>().ok().filter(|index| index.to_string() == &*v0).map(|index| index as i32).unwrap_or(-1);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = 0i32 == v1;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = std::rc::Rc::<str>::from((v4).split(char::from(10u8)).nth(0i32 as usize).unwrap_or(""));
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect migration evidence toolchain and authority state"); } LIT.with(|lit| lit.clone()) };
        (v6.clone(), v7.clone(), v5.clone(), v8.clone(), v6.clone())
    } else {
        let v9: bool = 1i32 == v1;
        if v9 {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = std::rc::Rc::<str>::from((v10).split(char::from(10u8)).nth(1i32 as usize).unwrap_or(""));
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mutate"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rehearse gate and apply typed patch plans"); } LIT.with(|lit| lit.clone()) };
            (v12.clone(), v13.clone(), v11.clone(), v14.clone(), v12.clone())
        } else {
            let v15: bool = 2i32 == v1;
            if v15 {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = std::rc::Rc::<str>::from((v16).split(char::from(10u8)).nth(2i32 as usize).unwrap_or(""));
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mutate"); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("manage typed backlog leases and turn receipts"); } LIT.with(|lit| lit.clone()) };
                (v18.clone(), v19.clone(), v17.clone(), v20.clone(), v18.clone())
            } else {
                let v21: bool = 3i32 == v1;
                if v21 {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = std::rc::Rc::<str>::from((v22).split(char::from(10u8)).nth(3i32 as usize).unwrap_or(""));
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bundle"); } LIT.with(|lit| lit.clone()) };
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wrap"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check create verify rehydrate and retain bundles"); } LIT.with(|lit| lit.clone()) };
                    (v24.clone(), v25.clone(), v23.clone(), v26.clone(), v24.clone())
                } else {
                    let v27: bool = 4i32 == v1;
                    if v27 {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = std::rc::Rc::<str>::from((v28).split(char::from(10u8)).nth(4i32 as usize).unwrap_or(""));
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proxy"); } LIT.with(|lit| lit.clone()) };
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mutate"); } LIT.with(|lit| lit.clone()) };
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("execute bounded filesystem process archive toolchain and evidence capabilities"); } LIT.with(|lit| lit.clone()) };
                        (v30.clone(), v31.clone(), v29.clone(), v32.clone(), v30.clone())
                    } else {
                        let v33: bool = 5i32 == v1;
                        if v33 {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
                            let v35: Rc<str> = std::rc::Rc::<str>::from((v34).split(char::from(10u8)).nth(5i32 as usize).unwrap_or(""));
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("help"); } LIT.with(|lit| lit.clone()) };
                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect"); } LIT.with(|lit| lit.clone()) };
                            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("render the public command catalog"); } LIT.with(|lit| lit.clone()) };
                            (v36.clone(), v37.clone(), v35.clone(), v38.clone(), v36.clone())
                        } else {
                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect"); } LIT.with(|lit| lit.clone()) };
                            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown public verb"); } LIT.with(|lit| lit.clone()) };
                            (v2.clone(), v39.clone(), v2.clone(), v40.clone(), v2.clone())
                        }
                    }
                }
            }
        }
    }
}
fn command_descriptor_json_by_index_4(mut v0: Rc<str>) -> Rc<str> {
    let (mut v1, mut v2, mut v3, mut v4, mut v5): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = command_descriptor_by_index_3(v0.clone());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"name\":\""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"effect\":\""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v1, &*v7, &*v2, &*v8].concat());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"handler\":\""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"summary\":\""); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v5, &*v11, &*v4, &*v8].concat());
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"usage\":\""); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"}"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v3, &*v14, &*v15, &*v15].concat());
    let v17: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v12, &*v16, &*v15, &*v15].concat());
    v17.clone()
}
fn command_count_5() -> i32 {
    6i32
}
fn command_schema_witness_6() -> i32 {
    1i32
}
fn toolchain_action_code_7(mut v0: Rc<str>) -> u64 {
    let v1: u64 = u64::MAX;
    let v2: bool = "cargo-lock" == &*v0;
    if v2 {
        let v3: u64 = (1i32 - 1) as u64;
        v3
    } else {
        let v4: bool = "cargo-fmt-check" == &*v0;
        if v4 {
            let v5: u64 = (2i32 - 1) as u64;
            v5
        } else {
            let v6: bool = "cargo-check" == &*v0;
            if v6 {
                let v7: u64 = (3i32 - 1) as u64;
                v7
            } else {
                let v8: bool = "cargo-test" == &*v0;
                if v8 {
                    let v9: u64 = (4i32 - 1) as u64;
                    v9
                } else {
                    let v10: bool = "cargo-clippy" == &*v0;
                    if v10 {
                        let v11: u64 = (5i32 - 1) as u64;
                        v11
                    } else {
                        let v12: bool = "cargo-build" == &*v0;
                        if v12 {
                            let v13: u64 = (6i32 - 1) as u64;
                            v13
                        } else {
                            let v14: bool = "cargo-version" == &*v0;
                            if v14 {
                                let v15: u64 = (7i32 - 1) as u64;
                                v15
                            } else {
                                let v16: bool = "spiral-version" == &*v0;
                                if v16 {
                                    let v17: u64 = (8i32 - 1) as u64;
                                    v17
                                } else {
                                    v1
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn proxy_usage_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan-ir-inspect|plan-ir-check|install-self|self-upgrade-check|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|fs-list|fs-read|fs-context|fs-search|parallel-search|hash|hash-tree|fs-write|text-replace|fs-remove|fs-remove-tree|run|archive-extract|zip-extract|toolchain|portable-toolchain|legacy-surface|restart-baton|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-slice|parallel-manifest|product-diff|fs-copy|spiral-session-shutdown|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|source-map|spi-diff|source-diff|source-recovery-diff|cold-rebuild-matrix|cold-rebuild-owner|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|formal-receipts-renew|coverage-register"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proxy plan-ir-inspect <index>\nproxy plan-ir-check\nproxy install-self <source-binary> <installed-binary> <canonical-link>\nproxy self-upgrade-check <current-root> <candidate-root>\nproxy fs-chmod <root> <relative-path> <octal-mode>\nproxy fs-symlink <root> <target-relative> <link-relative>\nproxy fs-copy-tree <source> <destination>\nproxy command-output <root> <timeout-ms> <program> [args...]\nproxy command-capture <root> <receipt-relative> <timeout-ms> <program> [args...]\nproxy command-capture-env <root> <receipt-relative> <timeout-ms> <cwd-relative> <env-cell> <program> [args...]\nproxy fs-list <root> [relative]\nproxy fs-read <root> <relative> [max-bytes]\nproxy fs-context <root> <relative> <needle> <radius-bytes>\nproxy fs-search <root> <relative> <needle>\nproxy parallel-search <root> <relative> <needle> <max-hits>\nproxy hash <root> <relative>\nproxy hash-tree <root> <relative>\nproxy fs-write <root> <relative> [--replace-existing|--create-or-replace] [--expect-hash <sha256>] [--expect-nonempty] [--] <text>\nproxy text-replace <check|apply> <root> <relative> <old> <new> [--all|--expect <count>]\nproxy fs-remove <root> <relative>\nproxy fs-remove-tree <root> <relative> [--missing-ok]\nproxy run --cwd <directory> --program <program> [--timeout-ms <ms>] [--env KEY=VALUE]... [--unset KEY]... [-- <args...>]\nproxy archive-extract <archive> <destination>\nproxy zip-extract <archive> <destination>\nproxy toolchain <root> <action> ...\nproxy portable-toolchain <root> ...\nproxy legacy-surface\nproxy restart-baton <root> <receipt-relative> <bundle-absolute> <expected-migration> <active-workstream> <touched-sources> <next-gate>\nproxy command-capture-matrix <root> <spec-relative> <timeout-ms> <report-relative> [parallel-chains [auto | start-chain chain-count]]\nproxy command-capture-matrix-gate <root> <report-relative>\nproxy batch-plan <root> <spec-relative> <timeout-ms> <report-relative> [parallel-chains [auto | start-chain chain-count]]\nproxy batch-plan-resume <root> <report-relative> <resume-relative>\nproxy fs-slice <root> <relative> <start> <count>\nproxy parallel-manifest <root> <relative> <max-entries> [preview|apply <output-relative>]\nproxy product-diff <left-root> <right-root>\nproxy fs-copy <root> <source> <target>\nproxy spiral-session-shutdown <compiler> [timeout-ms]\nproxy prune-build-cache <root> <relative-target>\nproxy prune-compiler-sidecars <root> [--dry-run]\nproxy incident-recovery snapshot <root> <source-relative> <snapshot-parent-relative> <receipt-relative> | restore <root> <target-relative> <snapshot-relative> <receipt-relative>\nproxy external-payload <register|hydrate|dehydrate> <root> <payload-relative> <cache-relative> <receipt-relative>\nproxy differential-compare <root> <oracle> <direct> <receipt>\nproxy coverage-assess <test.lcov> <smoke.lcov> <test-floor> <smoke-floor> <combined-floor>\nproxy coverage-export <profraw-dir> <binary-path> <grcov> <llvm-bin-dir> <source-root> <output-lcov> <timeout-ms>\nproxy coverage-union <output-lcov> <input-lcov> <input-lcov> [input-lcov ...]\nproxy coverage-run <root> <cargo> <compiler> <target-dir> <profraw-dir> <public|workspace|smoke|owner:package> <timeout-ms>\nproxy source-stats <root> [--crates]\nproxy source-topology <root> [state/authority_census.spi]\nproxy ingest-map <root>\nproxy ingest-trim-check <root>\nproxy ingest-extracted-manifest <root>\nproxy ingest-targets <root>\nproxy ingest-targets-derive <root>\nproxy frontier-map <root>\nproxy spi-map <root>\nproxy source-map <root>\nproxy spi-diff <left-root> <right-root>\nproxy source-diff <left-root> <right-root>\nproxy source-recovery-diff <current-root> <candidate-root> [candidate-root ...]\nproxy cold-rebuild-matrix <root> <matrix-relative> <kit> <compiler> <dotnet> <rustfmt> [compiler-timeout-ms]\nproxy cold-rebuild-owner <root> <snapshot-relative> <input-relative> <output-relative> <compiler> <dotnet> <rustfmt-or-empty> <timeout-ms> <workspace-root-or-empty>\nproxy source-author <preview|apply> <root> <target-relative> <template-relative> <expected-target-sha256|missing> <expected-template-sha256> KEY=VALUE...\nproxy fs-batch-write <preview|apply> <root> <plan.spi>\nproxy fs-actions <preview|apply> <root> <plan.spi>\nproxy binary-install <preview|apply> <root> <source-relative> <destination-relative> <provenance-relative> <expected-source-sha256> <expected-destination-sha256|missing> <expected-provenance-sha256|missing> <mode-octal>\nproxy release-closeout <preview|apply> <root> <receipt-relative> <expected-migration> <expected-receipt-sha256|missing> <name|relative|sha256>... (six gates)\nproxy shim install <root> <relative-dir> <block|warn|pass> | uninstall <root> <relative-dir> | status <root> <relative-dir>\nproxy prune-uncovered <root> <relative> <test.lcov> <smoke.lcov> -- <validation-program> [args...]\nproxy formal-receipts-renew <root>\nproxy coverage-register <root> <current.lcov> <seed.lcov>"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = v1.split("|").zip(v2.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    v3.clone()
}
fn proxy_route_code_9(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan-ir-inspect|plan-ir-check|prune-compiler-sidecars|incident-recovery|external-payload|coverage-export|coverage-union|legacy-surface|install-self|self-upgrade-check|restart-baton|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|fs-chmod|fs-symlink|fs-copy-tree|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|portable-toolchain|product-diff|source-recovery-diff"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v1.split("|").any(|item| item == &*v0);
    if v2 {
        let v3: u64 = v1.split("|").position(|item| item == &*v0).map(|index| index as u64).unwrap_or(u64::MAX);
        v3
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|hash|hash-tree|fs-copy"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v4.split("|").any(|item| item == &*v0);
        if v5 {
            29u64
        } else {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("archive-extract"); } LIT.with(|lit| lit.clone()) };
            let v7: bool = v0 == v6 ;
            if v7 {
                30u64
            } else {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zip-extract"); } LIT.with(|lit| lit.clone()) };
                let v9: bool = v0 == v8 ;
                if v9 {
                    31u64
                } else {
                    32u64
                }
            }
        }
    }
}
fn proxy_route_witness_10() -> i32 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan-ir-inspect"); } LIT.with(|lit| lit.clone()) };
    let v1: u64 = proxy_route_code_9(v0.clone());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("source-recovery-diff"); } LIT.with(|lit| lit.clone()) };
    let v3: u64 = proxy_route_code_9(v2.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash-tree"); } LIT.with(|lit| lit.clone()) };
    let v5: u64 = proxy_route_code_9(v4.clone());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("archive-extract"); } LIT.with(|lit| lit.clone()) };
    let v7: u64 = proxy_route_code_9(v6.clone());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zip-extract"); } LIT.with(|lit| lit.clone()) };
    let v9: u64 = proxy_route_code_9(v8.clone());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-write"); } LIT.with(|lit| lit.clone()) };
    let v11: u64 = proxy_route_code_9(v10.clone());
    let v12: bool = 0u64 == v1;
    if v12 {
        let v13: bool = 28u64 == v3;
        if v13 {
            let v14: bool = 29u64 == v5;
            if v14 {
                let v15: bool = 30u64 == v7;
                if v15 {
                    let v16: bool = 31u64 == v9;
                    if v16 {
                        let v17: bool = 32u64 == v11;
                        if v17 {
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
}
fn help_flag_code_11(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (4i32 - 1) as u64;
    let v2: bool = "--completion" == &*v0;
    if v2 {
        let v3: u64 = (1i32 - 1) as u64;
        v3
    } else {
        let v4: bool = "--json" == &*v0;
        if v4 {
            let v5: u64 = (2i32 - 1) as u64;
            v5
        } else {
            let v6: bool = "--schema" == &*v0;
            if v6 {
                let v7: u64 = (3i32 - 1) as u64;
                v7
            } else {
                let v8: bool = "" == &*v0;
                if v8 {
                    let v9: u64 = (4i32 - 1) as u64;
                    v9
                } else {
                    v1
                }
            }
        }
    }
}
fn help_flag_witness_12() -> i32 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--completion"); } LIT.with(|lit| lit.clone()) };
    let v1: u64 = help_flag_code_11(v0.clone());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
    let v3: u64 = help_flag_code_11(v2.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--schema"); } LIT.with(|lit| lit.clone()) };
    let v5: u64 = help_flag_code_11(v4.clone());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let v7: u64 = help_flag_code_11(v6.clone());
    let v8: bool = 0u64 == v1;
    if v8 {
        let v9: bool = 1u64 == v3;
        if v9 {
            let v10: bool = 2u64 == v5;
            if v10 {
                let v11: bool = 3u64 == v7;
                if v11 {
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
}
fn help_flag_from_14(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let v2: bool = v0 >= v1;
        if v2 {
            return false;
        } else {
            let v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--help|-h"); } LIT.with(|lit| lit.clone()) };
            let v5: bool = v4.split("|").any(|item| item == &*v3);
            if v5 {
                return true;
            } else {
                let v6: i32 = v0 + 1;
                (v0, v1) = (v6, v1);
                continue;
            }
        }
    }
}
fn help_request_text_13() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(1i32 as usize).unwrap_or_default());
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2i32 as usize).unwrap_or_default());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("help"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v0 == v2 ;
    let v32: US0 = if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v1 == v4 ;
        let v9: bool = if v5 {
            false
        } else {
            let v6: Rc<str> = proxy_usage_8(v1.clone());
            let v7: bool = v6 == v4 ;
            let v8: bool = v7 == false;
            v8
        };
        if v9 {
            US0::US0_CapabilityHelp(v1.clone())
        } else {
            US0::US0_NoHelpRequested
        }
    } else {
        let v13: i32 = 2i32;
        let v14: usize = std::env::args().count();
        let v15: i32 = ((v14) as i32);
        let v16: bool = help_flag_from_14(v13, v15);
        if v16 {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proxy"); } LIT.with(|lit| lit.clone()) };
            let v18: bool = v0 == v17 ;
            if v18 {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v20: bool = v1 == v19 ;
                let v24: bool = if v20 {
                    false
                } else {
                    let v21: Rc<str> = proxy_usage_8(v1.clone());
                    let v22: bool = v21 == v19 ;
                    let v23: bool = v22 == false;
                    v23
                };
                if v24 {
                    US0::US0_CapabilityHelp(v1.clone())
                } else {
                    US0::US0_VerbHelp(v0.clone())
                }
            } else {
                US0::US0_VerbHelp(v0.clone())
            }
        } else {
            US0::US0_NoHelpRequested
        }
    };
    match &v32 {
        US0::US0_CapabilityHelp(v51) => {
            let v51: Rc<str> = v51.clone();
            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit "); } LIT.with(|lit| lit.clone()) };
            let v53: Rc<str> = proxy_usage_8(v51.clone());
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v55: Rc<str> = std::rc::Rc::<str>::from([&*v52, &*v53, &*v54, &*v54, &*v54].concat());
            v55.clone()
        }
        US0::US0_NoHelpRequested => {
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v33.clone()
        }
        US0::US0_VerbHelp(v34) => {
            let v34: Rc<str> = v34.clone();
            let v35: u64 = command_code_0(v34.clone());
            let v36: Rc<str> = std::rc::Rc::<str>::from((v35).to_string());
            let (mut v37, mut v38, mut v39, mut v40, mut v41): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = command_descriptor_by_index_3(v36.clone());
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("name="); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" effect="); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" handler="); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v37, &*v43, &*v38, &*v44].concat());
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nsummary="); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nusage: kit "); } LIT.with(|lit| lit.clone()) };
            let v48: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v46, &*v40, &*v47, &*v39].concat());
            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v50: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v48, &*v49, &*v49, &*v49].concat());
            v50.clone()
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        command_code_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        command_effect_code_1(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        command_subcommand_effect_code_2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        command_descriptor_by_index_3(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn command_descriptor_named_15(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = "status" == &*v0;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = std::rc::Rc::<str>::from((v3).split(char::from(10u8)).nth(0i32 as usize).unwrap_or(""));
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect migration evidence toolchain and authority state"); } LIT.with(|lit| lit.clone()) };
        (v5.clone(), v6.clone(), v4.clone(), v7.clone(), v5.clone())
    } else {
        let v8: bool = "patch" == &*v0;
        if v8 {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = std::rc::Rc::<str>::from((v9).split(char::from(10u8)).nth(1i32 as usize).unwrap_or(""));
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mutate"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rehearse gate and apply typed patch plans"); } LIT.with(|lit| lit.clone()) };
            (v11.clone(), v12.clone(), v10.clone(), v13.clone(), v11.clone())
        } else {
            let v14: bool = "agile" == &*v0;
            if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = std::rc::Rc::<str>::from((v15).split(char::from(10u8)).nth(2i32 as usize).unwrap_or(""));
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mutate"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("manage typed backlog leases and turn receipts"); } LIT.with(|lit| lit.clone()) };
                (v17.clone(), v18.clone(), v16.clone(), v19.clone(), v17.clone())
            } else {
                let v20: bool = "bundle" == &*v0;
                if v20 {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = std::rc::Rc::<str>::from((v21).split(char::from(10u8)).nth(3i32 as usize).unwrap_or(""));
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bundle"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wrap"); } LIT.with(|lit| lit.clone()) };
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check create verify rehydrate and retain bundles"); } LIT.with(|lit| lit.clone()) };
                    (v23.clone(), v24.clone(), v22.clone(), v25.clone(), v23.clone())
                } else {
                    let v26: bool = "proxy" == &*v0;
                    if v26 {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
                        let v28: Rc<str> = std::rc::Rc::<str>::from((v27).split(char::from(10u8)).nth(4i32 as usize).unwrap_or(""));
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proxy"); } LIT.with(|lit| lit.clone()) };
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mutate"); } LIT.with(|lit| lit.clone()) };
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("execute bounded filesystem process archive toolchain and evidence capabilities"); } LIT.with(|lit| lit.clone()) };
                        (v29.clone(), v30.clone(), v28.clone(), v31.clone(), v29.clone())
                    } else {
                        let v32: bool = "help" == &*v0;
                        if v32 {
                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|kit] | create <root> <output.zip> [auto|generic|kit] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|kit] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|kit] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
                            let v34: Rc<str> = std::rc::Rc::<str>::from((v33).split(char::from(10u8)).nth(5i32 as usize).unwrap_or(""));
                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("help"); } LIT.with(|lit| lit.clone()) };
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect"); } LIT.with(|lit| lit.clone()) };
                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("render the public command catalog"); } LIT.with(|lit| lit.clone()) };
                            (v35.clone(), v36.clone(), v34.clone(), v37.clone(), v35.clone())
                        } else {
                            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect"); } LIT.with(|lit| lit.clone()) };
                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown public verb"); } LIT.with(|lit| lit.clone()) };
                            (v1.clone(), v38.clone(), v1.clone(), v39.clone(), v1.clone())
                        }
                    }
                }
            }
        }
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        command_descriptor_named_15(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        command_descriptor_json_by_index_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        command_count_5()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        command_schema_witness_6()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        toolchain_action_code_7(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        proxy_usage_8(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure10() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        proxy_route_code_9(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure11() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        proxy_route_witness_10()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure12() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        help_flag_code_11(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure13() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        help_flag_witness_12()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure14() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        help_request_text_13()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn direct_verb_usage_16(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = "job" == &*v0;
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)"); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        let v3: bool = "repo" == &*v0;
        if v3 {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repo: state <root> [<root>...] | affected <src> <package>... (Cargo crates whose package.spiproj graph reaches a changed package) | accept-review <before> <after> (passes when every changed line differs in numeric literals only)"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        } else {
            let v5: bool = "table" == &*v0;
            if v5 {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table: query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet] | test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] [--prediction x] [--confidence 0.95] [--where ...]... | sequences <file|dir>... --time <regex (?<ms>-?\\d+)> (--interval name=A [--between B] [--modulo ms] | --window name=A --from M --ms lo..hi | --order name=A,B | --latency name=A --to B | [--section-on <regex>] [--carry <regex>] (--first|--last) name=<regex (?<id>)(?<value>)>...) [--out file.tsv]"); } LIT.with(|lit| lit.clone()) };
                v6.clone()
            } else {
                let v7: bool = "read" == &*v0;
                if v7 {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                } else {
                    let v9: bool = "lines" == &*v0;
                    if v9 {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
                        v10.clone()
                    } else {
                        let v11: bool = "grep" == &*v0;
                        if v11 {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
                            v12.clone()
                        } else {
                            let v13: bool = "do" == &*v0;
                            if v13 {
                                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("do <root> '<program>' | do <root> - (stdin); edit <path|glob> [dry-run] [expect files n replacements n] / replace [pattern] once|all|<n> times \"old\" with \"new\" / patch <<<hunks>>> / show <path>; search [literal] \"re\" in <glob> [include a,b] [exclude glob] [context n[,m]] [first n per file] [paths] / read <path> around \"text\" [context n[,m]] / files <glob> / head <glob> n / tail <glob> n / wait <file> ~ \"re\" [timeout 5s]; hash <a,b,...> / env <A,B> / exists <a,b> / nearest \"re\" before lines n,m in <file> / json <file> get a.b,c.length / lines <file> count by \"(re)\" / files <glob> count by extension / table <file.tsv> [where col = \"v\"|col <> \"v\"] count by a,b / dirs <dir/*> having a and b + tag <name> = <file> ~ \"re\" [and|or|not ...] + count by <name> / diff <a.tsv> <b.tsv> by k1,k2 [on c1,c2]; run <cmd> [timeout 10s] [until output ~ \"re\" then stop] [background as name] / tail output n / search \"re\" in output / errors in output by \"(re)\" / processes where command ~ \"re\" / stop <name>; append <file> \"line {clock:HH:mm}\" / write <file> [overwrite] <<< text >>> / copy <glob> to <dir> [except *.tmp,...] / move <path> to <path> (no overwrite; absolute paths reach other roots), then read statements; git init|commit <repo> all \"msg\" [at <iso>] / git history <repo> \"text\" in <file> / git status a, b group by state / git log <repo> [since <date>] count by day / search \"re\" in <glob> with git status; job start <cmd> as <name> timeout 30s [notify] / job wait <name> [timeout 5m] / job stop <name>; lab audit <dir> against <backlog>"); } LIT.with(|lit| lit.clone()) };
                                v14.clone()
                            } else {
                                let v15: bool = "ps" == &*v0;
                                if v15 {
                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)"); } LIT.with(|lit| lit.clone()) };
                                    v16.clone()
                                } else {
                                    let v17: bool = "kill" == &*v0;
                                    if v17 {
                                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)"); } LIT.with(|lit| lit.clone()) };
                                        v18.clone()
                                    } else {
                                        let v19: bool = "standards" == &*v0;
                                        if v19 {
                                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standards: <package-dir>... [--json] (rules from the package's standards.spi rows, else no-comment-lines, line-width 120, lf-only, test-targets; exit 1 on findings) | --census <dir>... | --baseline <file.spi> <dir>... | --ratchet <file.spi> <dir>... (typefeatureset and drift counts per package; the ratchet fails when a package gets worse than its baseline or a new package misses the floor)"); } LIT.with(|lit| lit.clone()) };
                                            v20.clone()
                                        } else {
                                            let v21: bool = "window" == &*v0;
                                            if v21 {
                                                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window: request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | release <id> | list | check <class> | claim <path> | wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] (machine windows shared by every session; live-gui and compiler-install exclude all other classes; exit 1 when blocked)"); } LIT.with(|lit| lit.clone()) };
                                                v22.clone()
                                            } else {
                                                let v23: bool = "experiment" == &*v0;
                                                if v23 {
                                                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("experiment: register <ledger> <ID> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <col> --runs <n> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | analyze <ledger> <ID> <table.tsv> [--where col=value]... [--interim <reason>] | amend <ledger> <ID> --field <name> --value <v> --reason <text> | addendum <ledger> <ID> <text> | show <ledger> <ID> (pre-registered; one analysis at the planned count)"); } LIT.with(|lit| lit.clone()) };
                                                    v24.clone()
                                                } else {
                                                    let v25: bool = "guide" == &*v0;
                                                    if v25 {
                                                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("guide <spec/one-call.md> [--out <file.md>]"); } LIT.with(|lit| lit.clone()) };
                                                        v26.clone()
                                                    } else {
                                                        let v27: bool = "machine" == &*v0;
                                                        if v27 {
                                                            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine watch [--sample-seconds n]"); } LIT.with(|lit| lit.clone()) };
                                                            v28.clone()
                                                        } else {
                                                            let v29: bool = "adoption" == &*v0;
                                                            if v29 {
                                                                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("adoption [--days n] [--out <file.tsv>] [--habits]"); } LIT.with(|lit| lit.clone()) };
                                                                v30.clone()
                                                            } else {
                                                                let v31: bool = "memory" == &*v0;
                                                                if v31 {
                                                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory import <memory-dir> <data.spi> | render <data.spi> <out-dir> | check <data.spi> | list <data.spi> [--kind user|feedback|project|reference] | recall <data.spi> <words> [--limit n]"); } LIT.with(|lit| lit.clone()) };
                                                                    v32.clone()
                                                                } else {
                                                                    let v33: bool = "resume" == &*v0;
                                                                    if v33 {
                                                                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume <backlog-root> [--team <team>] [--as <author>]"); } LIT.with(|lit| lit.clone()) };
                                                                        v34.clone()
                                                                    } else {
                                                                        let v35: bool = "ci" == &*v0;
                                                                        if v35 {
                                                                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ci parity <repo> (setup-rust/setup-zig/setup-beam pins in <repo>/.github/workflows against the local toolchain; exit 1 on a difference)"); } LIT.with(|lit| lit.clone()) };
                                                                            v36.clone()
                                                                        } else {
                                                                            let v37: bool = "process" == &*v0;
                                                                            if v37 {
                                                                                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process start-under-shell <program> [args...] (detached, the user shell as its live parent, no inherited handles, no caller job; prints pid and parent)"); } LIT.with(|lit| lit.clone()) };
                                                                                v38.clone()
                                                                            } else {
                                                                                let v39: bool = "crate" == &*v0;
                                                                                if v39 {
                                                                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("crate gate <root> [--enforce] [--limit n] (generated output lines of every [package.metadata.spiral] crate under root; --enforce fails on a crate over the limit or on finding none)"); } LIT.with(|lit| lit.clone()) };
                                                                                    v40.clone()
                                                                                } else {
                                                                                    let v41: bool = "env" == &*v0;
                                                                                    if v41 {
                                                                                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env check <tool>=<pinned-path>... (each tool is looked up on PATH in order; the first copy found must be the pinned one)"); } LIT.with(|lit| lit.clone()) };
                                                                                        v42.clone()
                                                                                    } else {
                                                                                        let v43: bool = "mirror" == &*v0;
                                                                                        if v43 {
                                                                                            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mirror check <left-root> <right-root> <path>... (paths relative to both roots; a directory stands for every file under it)"); } LIT.with(|lit| lit.clone()) };
                                                                                            v44.clone()
                                                                                        } else {
                                                                                            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                                            v45.clone()
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
fn closure15() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        direct_verb_usage_16(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_command_code(v0: &str) -> u64 {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_command_effect_code(v0: &str) -> u64 {
    closure1()(Rc::<str>::from(v0))
}
pub fn kit_command_subcommand_effect_code(v0: &str) -> u64 {
    closure2()(Rc::<str>::from(v0))
}
pub fn kit_command_descriptor(v0: &str) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure3()(Rc::<str>::from(v0))
}
pub fn kit_command_descriptor_named(v0: &str) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure4()(Rc::<str>::from(v0))
}
pub fn kit_command_descriptor_json(v0: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0))
}
pub fn kit_command_count() -> i32 {
    closure6()()
}
pub fn kit_command_schema_witness() -> i32 {
    closure7()()
}
pub fn kit_toolchain_action_name_code(v0: &str) -> u64 {
    closure8()(Rc::<str>::from(v0))
}
pub fn kit_command_proxy_usage(v0: &str) -> Rc<str> {
    closure9()(Rc::<str>::from(v0))
}
pub fn kit_proxy_route_code(v0: &str) -> u64 {
    closure10()(Rc::<str>::from(v0))
}
pub fn kit_proxy_route_witness() -> i32 {
    closure11()()
}
pub fn kit_help_flag_code(v0: &str) -> u64 {
    closure12()(Rc::<str>::from(v0))
}
pub fn kit_help_flag_witness() -> i32 {
    closure13()()
}
pub fn kit_help_request_text() -> Rc<str> {
    closure14()()
}
pub fn kit_direct_verb_usage(v0: &str) -> Rc<str> {
    closure15()(Rc::<str>::from(v0))
}
