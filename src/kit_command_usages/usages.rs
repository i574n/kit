#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn command_more_usages_0() -> Rc<str> {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)\n"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repo: state <root> [<root>...] | affected <src> <package>... (Cargo crates whose package.spiproj graph reaches a changed package) | accept-review <before> <after> (passes when every changed line differs in numeric literals only)"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)\nrepo: state <root> [<root>...] | affected <src> <package>... (Cargo crates whose package.spiproj graph reaches a changed package) | accept-review <before> <after> (passes when every changed line differs in numeric literals only)"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = &*v4 == "";
    let (mut v10, mut v11): (Rc<str>, Rc<str>) = if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table: query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet] | test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] [--prediction x] [--confidence 0.95] [--where ...]... | sequences <file|dir>... --time <regex (?<ms>-?\\d+)> (--interval name=A [--between B] [--modulo ms] | --window name=A --from M --ms lo..hi | --order name=A,B | --latency name=A --to B | [--section-on <regex>] [--carry <regex>] (--first|--last) name=<regex (?<id>)(?<value>)>... | [--section-on <regex>] (--count name=<regex [(?<value>)]>... [--count-in name=<regex (?<rest>)>] [--in-set name=<file>[:<regex (?<value>)>]])) [--out file.tsv]"); } LIT.with(|lit| lit.clone()) };
        (v6.clone(), v6.clone())
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)\nrepo: state <root> [<root>...] | affected <src> <package>... (Cargo crates whose package.spiproj graph reaches a changed package) | accept-review <before> <after> (passes when every changed line differs in numeric literals only)\n"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table: query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet] | test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] [--prediction x] [--confidence 0.95] [--where ...]... | sequences <file|dir>... --time <regex (?<ms>-?\\d+)> (--interval name=A [--between B] [--modulo ms] | --window name=A --from M --ms lo..hi | --order name=A,B | --latency name=A --to B | [--section-on <regex>] [--carry <regex>] (--first|--last) name=<regex (?<id>)(?<value>)>... | [--section-on <regex>] (--count name=<regex [(?<value>)]>... [--count-in name=<regex (?<rest>)>] [--in-set name=<file>[:<regex (?<value>)>]])) [--out file.tsv]"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)\nrepo: state <root> [<root>...] | affected <src> <package>... (Cargo crates whose package.spiproj graph reaches a changed package) | accept-review <before> <after> (passes when every changed line differs in numeric literals only)\ntable: query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet] | test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] [--prediction x] [--confidence 0.95] [--where ...]... | sequences <file|dir>... --time <regex (?<ms>-?\\d+)> (--interval name=A [--between B] [--modulo ms] | --window name=A --from M --ms lo..hi | --order name=A,B | --latency name=A --to B | [--section-on <regex>] [--carry <regex>] (--first|--last) name=<regex (?<id>)(?<value>)>... | [--section-on <regex>] (--count name=<regex [(?<value>)]>... [--count-in name=<regex (?<rest>)>] [--in-set name=<file>[:<regex (?<value>)>]])) [--out file.tsv]"); } LIT.with(|lit| lit.clone()) };
        (v9.clone(), v8.clone())
    };
    let v12: bool = "read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]" == &*v11;
    let (mut v20, mut v21): (Rc<str>, Rc<str>) = if v12 {
        (v10.clone(), v11.clone())
    } else {
        let v13: bool = &*v10 == "";
        let (mut v18, mut v19): (Rc<str>, Rc<str>) = if v13 {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            (v14.clone(), v14.clone())
        } else {
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v1));
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            (v17.clone(), v16.clone())
        };
        (v18.clone(), v19.clone())
    };
    let v22: bool = "read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]" == &*v21;
    let (mut v30, mut v31): (Rc<str>, Rc<str>) = if v22 {
        (v20.clone(), v21.clone())
    } else {
        let v23: bool = &*v20 == "";
        let (mut v28, mut v29): (Rc<str>, Rc<str>) = if v23 {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            (v24.clone(), v24.clone())
        } else {
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v1));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            (v27.clone(), v26.clone())
        };
        (v28.clone(), v29.clone())
    };
    let v32: bool = "read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]" == &*v31;
    let (mut v40, mut v41): (Rc<str>, Rc<str>) = if v32 {
        (v30.clone(), v31.clone())
    } else {
        let v33: bool = &*v30 == "";
        let (mut v38, mut v39): (Rc<str>, Rc<str>) = if v33 {
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            (v34.clone(), v34.clone())
        } else {
            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v1));
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
            (v37.clone(), v36.clone())
        };
        (v38.clone(), v39.clone())
    };
    let v42: bool = "do <root> [--dry-run] '<program>' | do <root> [--dry-run] - (stdin); edit <path|glob> [dry-run] [expect files n replacements n] / replace [pattern] once|all|<n> times \"old\" with \"new\" / patch <<<hunks>>> / show <path>; search [literal] \"re\" in <glob> [include a,b] [exclude glob] [context n[,m]] [first n per file] [paths] / read <path> around \"text\" [context n[,m]] / files <glob> / head <glob> n / tail <glob> n / wait <file> ~ \"re\" [timeout 5s]; hash <a,b,...> / env <A,B> / exists <a,b> / nearest \"re\" before lines n,m in <file> / json <file> get a.b,c.length / lines <file> count by \"(re)\" / files <glob> count by extension / table <file.tsv> [where col = \"v\"|col <> \"v\"] count by a,b / dirs <dir/*> having a and b + tag <name> = <file> ~ \"re\" [and|or|not ...] + count by <name> / diff <a.tsv> <b.tsv> by k1,k2 [on c1,c2]; run <cmd> [timeout 10s] [until output ~ \"re\" then stop] [background as name] / tail output n / search \"re\" in output / errors in output by \"(re)\" / processes where command ~ \"re\" / stop <name>; append <file> \"line {clock:HH:mm}\" / write <file> [overwrite] <<< text >>> / copy <glob> to <dir> [except *.tmp,...] / move <path> to <path> (no overwrite; absolute paths reach other roots), then read statements; git init|commit <repo> all \"msg\" [at <iso>] / git history <repo> \"text\" in <file> / git status a, b group by state / git log <repo> [since <date>] count by day / search \"re\" in <glob> with git status; job start <cmd> as <name> timeout 30s [notify] / job wait <name> [timeout 5m] / job stop <name>; lab audit <dir> against <backlog>" == &*v41;
    let (mut v50, mut v51): (Rc<str>, Rc<str>) = if v42 {
        (v40.clone(), v41.clone())
    } else {
        let v43: bool = &*v40 == "";
        let (mut v48, mut v49): (Rc<str>, Rc<str>) = if v43 {
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("do <root> [--dry-run] '<program>' | do <root> [--dry-run] - (stdin); edit <path|glob> [dry-run] [expect files n replacements n] / replace [pattern] once|all|<n> times \"old\" with \"new\" / patch <<<hunks>>> / show <path>; search [literal] \"re\" in <glob> [include a,b] [exclude glob] [context n[,m]] [first n per file] [paths] / read <path> around \"text\" [context n[,m]] / files <glob> / head <glob> n / tail <glob> n / wait <file> ~ \"re\" [timeout 5s]; hash <a,b,...> / env <A,B> / exists <a,b> / nearest \"re\" before lines n,m in <file> / json <file> get a.b,c.length / lines <file> count by \"(re)\" / files <glob> count by extension / table <file.tsv> [where col = \"v\"|col <> \"v\"] count by a,b / dirs <dir/*> having a and b + tag <name> = <file> ~ \"re\" [and|or|not ...] + count by <name> / diff <a.tsv> <b.tsv> by k1,k2 [on c1,c2]; run <cmd> [timeout 10s] [until output ~ \"re\" then stop] [background as name] / tail output n / search \"re\" in output / errors in output by \"(re)\" / processes where command ~ \"re\" / stop <name>; append <file> \"line {clock:HH:mm}\" / write <file> [overwrite] <<< text >>> / copy <glob> to <dir> [except *.tmp,...] / move <path> to <path> (no overwrite; absolute paths reach other roots), then read statements; git init|commit <repo> all \"msg\" [at <iso>] / git history <repo> \"text\" in <file> / git status a, b group by state / git log <repo> [since <date>] count by day / search \"re\" in <glob> with git status; job start <cmd> as <name> timeout 30s [notify] / job wait <name> [timeout 5m] / job stop <name>; lab audit <dir> against <backlog>"); } LIT.with(|lit| lit.clone()) };
            (v44.clone(), v44.clone())
        } else {
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v1));
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("do <root> [--dry-run] '<program>' | do <root> [--dry-run] - (stdin); edit <path|glob> [dry-run] [expect files n replacements n] / replace [pattern] once|all|<n> times \"old\" with \"new\" / patch <<<hunks>>> / show <path>; search [literal] \"re\" in <glob> [include a,b] [exclude glob] [context n[,m]] [first n per file] [paths] / read <path> around \"text\" [context n[,m]] / files <glob> / head <glob> n / tail <glob> n / wait <file> ~ \"re\" [timeout 5s]; hash <a,b,...> / env <A,B> / exists <a,b> / nearest \"re\" before lines n,m in <file> / json <file> get a.b,c.length / lines <file> count by \"(re)\" / files <glob> count by extension / table <file.tsv> [where col = \"v\"|col <> \"v\"] count by a,b / dirs <dir/*> having a and b + tag <name> = <file> ~ \"re\" [and|or|not ...] + count by <name> / diff <a.tsv> <b.tsv> by k1,k2 [on c1,c2]; run <cmd> [timeout 10s] [until output ~ \"re\" then stop] [background as name] / tail output n / search \"re\" in output / errors in output by \"(re)\" / processes where command ~ \"re\" / stop <name>; append <file> \"line {clock:HH:mm}\" / write <file> [overwrite] <<< text >>> / copy <glob> to <dir> [except *.tmp,...] / move <path> to <path> (no overwrite; absolute paths reach other roots), then read statements; git init|commit <repo> all \"msg\" [at <iso>] / git history <repo> \"text\" in <file> / git status a, b group by state / git log <repo> [since <date>] count by day / search \"re\" in <glob> with git status; job start <cmd> as <name> timeout 30s [notify] / job wait <name> [timeout 5m] / job stop <name>; lab audit <dir> against <backlog>"); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
            (v47.clone(), v46.clone())
        };
        (v48.clone(), v49.clone())
    };
    let v52: bool = "ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)" == &*v51;
    let (mut v60, mut v61): (Rc<str>, Rc<str>) = if v52 {
        (v50.clone(), v51.clone())
    } else {
        let v53: bool = &*v50 == "";
        let (mut v58, mut v59): (Rc<str>, Rc<str>) = if v53 {
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)"); } LIT.with(|lit| lit.clone()) };
            (v54.clone(), v54.clone())
        } else {
            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v1));
            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)"); } LIT.with(|lit| lit.clone()) };
            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
            (v57.clone(), v56.clone())
        };
        (v58.clone(), v59.clone())
    };
    let v62: bool = "ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)" == &*v61;
    let (mut v70, mut v71): (Rc<str>, Rc<str>) = if v62 {
        (v60.clone(), v61.clone())
    } else {
        let v63: bool = &*v60 == "";
        let (mut v68, mut v69): (Rc<str>, Rc<str>) = if v63 {
            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)"); } LIT.with(|lit| lit.clone()) };
            (v64.clone(), v64.clone())
        } else {
            let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v1));
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)"); } LIT.with(|lit| lit.clone()) };
            let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v66));
            (v67.clone(), v66.clone())
        };
        (v68.clone(), v69.clone())
    };
    let v72: bool = "standards: <package-dir>... [--json] (rules from the package's standards.spi rows, else no-comment-lines, line-width 120, lf-only, test-targets; exit 1 on findings) | --census <dir>... | --baseline <file.spi> <dir>... | --ratchet <file.spi> <dir>... (typefeatureset and drift counts per package; the ratchet fails when a package gets worse than its baseline or a new package misses the floor)" == &*v71;
    let (mut v80, mut v81): (Rc<str>, Rc<str>) = if v72 {
        (v70.clone(), v71.clone())
    } else {
        let v73: bool = &*v70 == "";
        let (mut v78, mut v79): (Rc<str>, Rc<str>) = if v73 {
            let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standards: <package-dir>... [--json] (rules from the package's standards.spi rows, else no-comment-lines, line-width 120, lf-only, test-targets; exit 1 on findings) | --census <dir>... | --baseline <file.spi> <dir>... | --ratchet <file.spi> <dir>... (typefeatureset and drift counts per package; the ratchet fails when a package gets worse than its baseline or a new package misses the floor)"); } LIT.with(|lit| lit.clone()) };
            (v74.clone(), v74.clone())
        } else {
            let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v1));
            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standards: <package-dir>... [--json] (rules from the package's standards.spi rows, else no-comment-lines, line-width 120, lf-only, test-targets; exit 1 on findings) | --census <dir>... | --baseline <file.spi> <dir>... | --ratchet <file.spi> <dir>... (typefeatureset and drift counts per package; the ratchet fails when a package gets worse than its baseline or a new package misses the floor)"); } LIT.with(|lit| lit.clone()) };
            let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v76));
            (v77.clone(), v76.clone())
        };
        (v78.clone(), v79.clone())
    };
    let v82: bool = "window: request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | release <id> | list | check <class> | claim <path> | wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] (machine windows shared by every session; live-gui and compiler-install exclude all other classes; exit 1 when blocked)" == &*v81;
    let (mut v90, mut v91): (Rc<str>, Rc<str>) = if v82 {
        (v80.clone(), v81.clone())
    } else {
        let v83: bool = &*v80 == "";
        let (mut v88, mut v89): (Rc<str>, Rc<str>) = if v83 {
            let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window: request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | release <id> | list | check <class> | claim <path> | wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] (machine windows shared by every session; live-gui and compiler-install exclude all other classes; exit 1 when blocked)"); } LIT.with(|lit| lit.clone()) };
            (v84.clone(), v84.clone())
        } else {
            let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v1));
            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window: request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | release <id> | list | check <class> | claim <path> | wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] (machine windows shared by every session; live-gui and compiler-install exclude all other classes; exit 1 when blocked)"); } LIT.with(|lit| lit.clone()) };
            let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v86));
            (v87.clone(), v86.clone())
        };
        (v88.clone(), v89.clone())
    };
    let v92: bool = "experiment: register <ledger> <ID> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <col> --runs <n> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | analyze <ledger> <ID> <table.tsv> [--where col=value]... [--interim <reason>] | amend <ledger> <ID> --field <name> --value <v> --reason <text> | addendum <ledger> <ID> <text> | show <ledger> <ID> (pre-registered; one analysis at the planned count)" == &*v91;
    let (mut v100, mut v101): (Rc<str>, Rc<str>) = if v92 {
        (v90.clone(), v91.clone())
    } else {
        let v93: bool = &*v90 == "";
        let (mut v98, mut v99): (Rc<str>, Rc<str>) = if v93 {
            let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("experiment: register <ledger> <ID> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <col> --runs <n> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | analyze <ledger> <ID> <table.tsv> [--where col=value]... [--interim <reason>] | amend <ledger> <ID> --field <name> --value <v> --reason <text> | addendum <ledger> <ID> <text> | show <ledger> <ID> (pre-registered; one analysis at the planned count)"); } LIT.with(|lit| lit.clone()) };
            (v94.clone(), v94.clone())
        } else {
            let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v1));
            let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("experiment: register <ledger> <ID> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <col> --runs <n> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | analyze <ledger> <ID> <table.tsv> [--where col=value]... [--interim <reason>] | amend <ledger> <ID> --field <name> --value <v> --reason <text> | addendum <ledger> <ID> <text> | show <ledger> <ID> (pre-registered; one analysis at the planned count)"); } LIT.with(|lit| lit.clone()) };
            let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v96));
            (v97.clone(), v96.clone())
        };
        (v98.clone(), v99.clone())
    };
    let v102: bool = "guide <spec/one-call.md> [--out <file.md>]" == &*v101;
    let (mut v110, mut v111): (Rc<str>, Rc<str>) = if v102 {
        (v100.clone(), v101.clone())
    } else {
        let v103: bool = &*v100 == "";
        let (mut v108, mut v109): (Rc<str>, Rc<str>) = if v103 {
            let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("guide <spec/one-call.md> [--out <file.md>]"); } LIT.with(|lit| lit.clone()) };
            (v104.clone(), v104.clone())
        } else {
            let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v1));
            let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("guide <spec/one-call.md> [--out <file.md>]"); } LIT.with(|lit| lit.clone()) };
            let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v106));
            (v107.clone(), v106.clone())
        };
        (v108.clone(), v109.clone())
    };
    let v112: bool = "machine watch [--sample-seconds n]" == &*v111;
    let (mut v120, mut v121): (Rc<str>, Rc<str>) = if v112 {
        (v110.clone(), v111.clone())
    } else {
        let v113: bool = &*v110 == "";
        let (mut v118, mut v119): (Rc<str>, Rc<str>) = if v113 {
            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine watch [--sample-seconds n]"); } LIT.with(|lit| lit.clone()) };
            (v114.clone(), v114.clone())
        } else {
            let v115: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v1));
            let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine watch [--sample-seconds n]"); } LIT.with(|lit| lit.clone()) };
            let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v115, v116));
            (v117.clone(), v116.clone())
        };
        (v118.clone(), v119.clone())
    };
    let v122: bool = "adoption [--days n] [--out <file.tsv>] [--habits]" == &*v121;
    let (mut v130, mut v131): (Rc<str>, Rc<str>) = if v122 {
        (v120.clone(), v121.clone())
    } else {
        let v123: bool = &*v120 == "";
        let (mut v128, mut v129): (Rc<str>, Rc<str>) = if v123 {
            let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("adoption [--days n] [--out <file.tsv>] [--habits]"); } LIT.with(|lit| lit.clone()) };
            (v124.clone(), v124.clone())
        } else {
            let v125: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v1));
            let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("adoption [--days n] [--out <file.tsv>] [--habits]"); } LIT.with(|lit| lit.clone()) };
            let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v126));
            (v127.clone(), v126.clone())
        };
        (v128.clone(), v129.clone())
    };
    let v132: bool = "memory import <memory-dir> <data.spi> | render <data.spi> <out-dir> | check <data.spi> | list <data.spi> [--kind user|feedback|project|reference] | recall <data.spi> <words> [--limit n] | save <memory-dir> <kind> <slug> <description> <body>|- [--title <text>] | forget <memory-dir> <slug> | edit <memory-dir> <slug> description|body <text>|-" == &*v131;
    let (mut v140, mut v141): (Rc<str>, Rc<str>) = if v132 {
        (v130.clone(), v131.clone())
    } else {
        let v133: bool = &*v130 == "";
        let (mut v138, mut v139): (Rc<str>, Rc<str>) = if v133 {
            let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory import <memory-dir> <data.spi> | render <data.spi> <out-dir> | check <data.spi> | list <data.spi> [--kind user|feedback|project|reference] | recall <data.spi> <words> [--limit n] | save <memory-dir> <kind> <slug> <description> <body>|- [--title <text>] | forget <memory-dir> <slug> | edit <memory-dir> <slug> description|body <text>|-"); } LIT.with(|lit| lit.clone()) };
            (v134.clone(), v134.clone())
        } else {
            let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v130, v1));
            let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory import <memory-dir> <data.spi> | render <data.spi> <out-dir> | check <data.spi> | list <data.spi> [--kind user|feedback|project|reference] | recall <data.spi> <words> [--limit n] | save <memory-dir> <kind> <slug> <description> <body>|- [--title <text>] | forget <memory-dir> <slug> | edit <memory-dir> <slug> description|body <text>|-"); } LIT.with(|lit| lit.clone()) };
            let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v136));
            (v137.clone(), v136.clone())
        };
        (v138.clone(), v139.clone())
    };
    let v142: bool = "resume <backlog-root> [--team <team>] [--as <author>]" == &*v141;
    let (mut v150, mut v151): (Rc<str>, Rc<str>) = if v142 {
        (v140.clone(), v141.clone())
    } else {
        let v143: bool = &*v140 == "";
        let (mut v148, mut v149): (Rc<str>, Rc<str>) = if v143 {
            let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume <backlog-root> [--team <team>] [--as <author>]"); } LIT.with(|lit| lit.clone()) };
            (v144.clone(), v144.clone())
        } else {
            let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v1));
            let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume <backlog-root> [--team <team>] [--as <author>]"); } LIT.with(|lit| lit.clone()) };
            let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v145, v146));
            (v147.clone(), v146.clone())
        };
        (v148.clone(), v149.clone())
    };
    let v152: bool = "ci parity <repo> (setup-rust/setup-zig/setup-beam pins in <repo>/.github/workflows against the local toolchain; exit 1 on a difference)" == &*v151;
    let (mut v160, mut v161): (Rc<str>, Rc<str>) = if v152 {
        (v150.clone(), v151.clone())
    } else {
        let v153: bool = &*v150 == "";
        let (mut v158, mut v159): (Rc<str>, Rc<str>) = if v153 {
            let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ci parity <repo> (setup-rust/setup-zig/setup-beam pins in <repo>/.github/workflows against the local toolchain; exit 1 on a difference)"); } LIT.with(|lit| lit.clone()) };
            (v154.clone(), v154.clone())
        } else {
            let v155: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v1));
            let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ci parity <repo> (setup-rust/setup-zig/setup-beam pins in <repo>/.github/workflows against the local toolchain; exit 1 on a difference)"); } LIT.with(|lit| lit.clone()) };
            let v157: Rc<str> = Rc::<str>::from(format!("{}{}", v155, v156));
            (v157.clone(), v156.clone())
        };
        (v158.clone(), v159.clone())
    };
    let v162: bool = "process start-under-shell <program> [args...] (detached, the user shell as its live parent, no inherited handles, no caller job; prints pid and parent)" == &*v161;
    let (mut v170, mut v171): (Rc<str>, Rc<str>) = if v162 {
        (v160.clone(), v161.clone())
    } else {
        let v163: bool = &*v160 == "";
        let (mut v168, mut v169): (Rc<str>, Rc<str>) = if v163 {
            let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process start-under-shell <program> [args...] (detached, the user shell as its live parent, no inherited handles, no caller job; prints pid and parent)"); } LIT.with(|lit| lit.clone()) };
            (v164.clone(), v164.clone())
        } else {
            let v165: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v1));
            let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process start-under-shell <program> [args...] (detached, the user shell as its live parent, no inherited handles, no caller job; prints pid and parent)"); } LIT.with(|lit| lit.clone()) };
            let v167: Rc<str> = Rc::<str>::from(format!("{}{}", v165, v166));
            (v167.clone(), v166.clone())
        };
        (v168.clone(), v169.clone())
    };
    let v172: bool = "crate gate <root> [--enforce] [--limit n] (generated output lines of every [package.metadata.spiral] crate under root; --enforce fails on a crate over the limit or on finding none)" == &*v171;
    let (mut v180, mut v181): (Rc<str>, Rc<str>) = if v172 {
        (v170.clone(), v171.clone())
    } else {
        let v173: bool = &*v170 == "";
        let (mut v178, mut v179): (Rc<str>, Rc<str>) = if v173 {
            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("crate gate <root> [--enforce] [--limit n] (generated output lines of every [package.metadata.spiral] crate under root; --enforce fails on a crate over the limit or on finding none)"); } LIT.with(|lit| lit.clone()) };
            (v174.clone(), v174.clone())
        } else {
            let v175: Rc<str> = Rc::<str>::from(format!("{}{}", v170, v1));
            let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("crate gate <root> [--enforce] [--limit n] (generated output lines of every [package.metadata.spiral] crate under root; --enforce fails on a crate over the limit or on finding none)"); } LIT.with(|lit| lit.clone()) };
            let v177: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v176));
            (v177.clone(), v176.clone())
        };
        (v178.clone(), v179.clone())
    };
    let v182: bool = "env check <tool>=<pinned-path>... (each tool is looked up on PATH in order; the first copy found must be the pinned one)" == &*v181;
    let (mut v190, mut v191): (Rc<str>, Rc<str>) = if v182 {
        (v180.clone(), v181.clone())
    } else {
        let v183: bool = &*v180 == "";
        let (mut v188, mut v189): (Rc<str>, Rc<str>) = if v183 {
            let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env check <tool>=<pinned-path>... (each tool is looked up on PATH in order; the first copy found must be the pinned one)"); } LIT.with(|lit| lit.clone()) };
            (v184.clone(), v184.clone())
        } else {
            let v185: Rc<str> = Rc::<str>::from(format!("{}{}", v180, v1));
            let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env check <tool>=<pinned-path>... (each tool is looked up on PATH in order; the first copy found must be the pinned one)"); } LIT.with(|lit| lit.clone()) };
            let v187: Rc<str> = Rc::<str>::from(format!("{}{}", v185, v186));
            (v187.clone(), v186.clone())
        };
        (v188.clone(), v189.clone())
    };
    let v192: bool = "mirror check <left-root> <right-root> <path>... (paths relative to both roots; a directory stands for every file under it)" == &*v191;
    let (mut v200, mut v201): (Rc<str>, Rc<str>) = if v192 {
        (v190.clone(), v191.clone())
    } else {
        let v193: bool = &*v190 == "";
        let (mut v198, mut v199): (Rc<str>, Rc<str>) = if v193 {
            let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mirror check <left-root> <right-root> <path>... (paths relative to both roots; a directory stands for every file under it)"); } LIT.with(|lit| lit.clone()) };
            (v194.clone(), v194.clone())
        } else {
            let v195: Rc<str> = Rc::<str>::from(format!("{}{}", v190, v1));
            let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mirror check <left-root> <right-root> <path>... (paths relative to both roots; a directory stands for every file under it)"); } LIT.with(|lit| lit.clone()) };
            let v197: Rc<str> = Rc::<str>::from(format!("{}{}", v195, v196));
            (v197.clone(), v196.clone())
        };
        (v198.clone(), v199.clone())
    };
    let v202: bool = "backlog: list|next|plan|views|view|view-add|roots|check|todos|links <root> ... (the team backlog; same grammar as agile <verb>)" == &*v201;
    let (mut v210, mut v211): (Rc<str>, Rc<str>) = if v202 {
        (v200.clone(), v201.clone())
    } else {
        let v203: bool = &*v200 == "";
        let (mut v208, mut v209): (Rc<str>, Rc<str>) = if v203 {
            let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog: list|next|plan|views|view|view-add|roots|check|todos|links <root> ... (the team backlog; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            (v204.clone(), v204.clone())
        } else {
            let v205: Rc<str> = Rc::<str>::from(format!("{}{}", v200, v1));
            let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog: list|next|plan|views|view|view-add|roots|check|todos|links <root> ... (the team backlog; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            let v207: Rc<str> = Rc::<str>::from(format!("{}{}", v205, v206));
            (v207.clone(), v206.clone())
        };
        (v208.clone(), v209.clone())
    };
    let v212: bool = "story: add|edit|show|set|rename|migrate-tags|note|note-edit|notes <root> ... (one backlog item; same grammar as agile <verb>)" == &*v211;
    let (mut v220, mut v221): (Rc<str>, Rc<str>) = if v212 {
        (v210.clone(), v211.clone())
    } else {
        let v213: bool = &*v210 == "";
        let (mut v218, mut v219): (Rc<str>, Rc<str>) = if v213 {
            let v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("story: add|edit|show|set|rename|migrate-tags|note|note-edit|notes <root> ... (one backlog item; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            (v214.clone(), v214.clone())
        } else {
            let v215: Rc<str> = Rc::<str>::from(format!("{}{}", v210, v1));
            let v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("story: add|edit|show|set|rename|migrate-tags|note|note-edit|notes <root> ... (one backlog item; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            let v217: Rc<str> = Rc::<str>::from(format!("{}{}", v215, v216));
            (v217.clone(), v216.clone())
        };
        (v218.clone(), v219.clone())
    };
    let v222: bool = "standup: status|handoff <root> (where the work stands; same grammar as agile <verb>)" == &*v221;
    let (mut v230, mut v231): (Rc<str>, Rc<str>) = if v222 {
        (v220.clone(), v221.clone())
    } else {
        let v223: bool = &*v220 == "";
        let (mut v228, mut v229): (Rc<str>, Rc<str>) = if v223 {
            let v224: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standup: status|handoff <root> (where the work stands; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            (v224.clone(), v224.clone())
        } else {
            let v225: Rc<str> = Rc::<str>::from(format!("{}{}", v220, v1));
            let v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standup: status|handoff <root> (where the work stands; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            let v227: Rc<str> = Rc::<str>::from(format!("{}{}", v225, v226));
            (v227.clone(), v226.clone())
        };
        (v228.clone(), v229.clone())
    };
    let v232: bool = "evidence refresh <root> <relative path> [--as <new relative path>] [--dry-run] (rewrites that file's EvidenceRef row in state/evidence.spi from the file on disk: sha256 and byte length)" == &*v231;
    let (mut v240, mut v241): (Rc<str>, Rc<str>) = if v232 {
        (v230.clone(), v231.clone())
    } else {
        let v233: bool = &*v230 == "";
        let (mut v238, mut v239): (Rc<str>, Rc<str>) = if v233 {
            let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence refresh <root> <relative path> [--as <new relative path>] [--dry-run] (rewrites that file's EvidenceRef row in state/evidence.spi from the file on disk: sha256 and byte length)"); } LIT.with(|lit| lit.clone()) };
            (v234.clone(), v234.clone())
        } else {
            let v235: Rc<str> = Rc::<str>::from(format!("{}{}", v230, v1));
            let v236: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence refresh <root> <relative path> [--as <new relative path>] [--dry-run] (rewrites that file's EvidenceRef row in state/evidence.spi from the file on disk: sha256 and byte length)"); } LIT.with(|lit| lit.clone()) };
            let v237: Rc<str> = Rc::<str>::from(format!("{}{}", v235, v236));
            (v237.clone(), v236.clone())
        };
        (v238.clone(), v239.clone())
    };
    let v242: bool = "lint macros <root> [<path>...] [--baseline <rows>] [--tsv] (host macros by class: long, block, flow, cast, arith); lint shadows <root> [<path>...] [--baseline <rows>] (a type declared twice in one module, or a top-level term redefined before any use)" == &*v241;
    let (mut v250, mut v251): (Rc<str>, Rc<str>) = if v242 {
        (v240.clone(), v241.clone())
    } else {
        let v243: bool = &*v240 == "";
        let (mut v248, mut v249): (Rc<str>, Rc<str>) = if v243 {
            let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lint macros <root> [<path>...] [--baseline <rows>] [--tsv] (host macros by class: long, block, flow, cast, arith); lint shadows <root> [<path>...] [--baseline <rows>] (a type declared twice in one module, or a top-level term redefined before any use)"); } LIT.with(|lit| lit.clone()) };
            (v244.clone(), v244.clone())
        } else {
            let v245: Rc<str> = Rc::<str>::from(format!("{}{}", v240, v1));
            let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lint macros <root> [<path>...] [--baseline <rows>] [--tsv] (host macros by class: long, block, flow, cast, arith); lint shadows <root> [<path>...] [--baseline <rows>] (a type declared twice in one module, or a top-level term redefined before any use)"); } LIT.with(|lit| lit.clone()) };
            let v247: Rc<str> = Rc::<str>::from(format!("{}{}", v245, v246));
            (v247.clone(), v246.clone())
        };
        (v248.clone(), v249.clone())
    };
    let v252: bool = "schedule due <name> <minutes> (exit 0 when due, 1 when not) | ran <name> | list (stamps under KIT_SCHEDULES_HOME, else LOCALAPPDATA\\kit\\schedules)" == &*v251;
    let (mut v260, mut v261): (Rc<str>, Rc<str>) = if v252 {
        (v250.clone(), v251.clone())
    } else {
        let v253: bool = &*v250 == "";
        let (mut v258, mut v259): (Rc<str>, Rc<str>) = if v253 {
            let v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("schedule due <name> <minutes> (exit 0 when due, 1 when not) | ran <name> | list (stamps under KIT_SCHEDULES_HOME, else LOCALAPPDATA\\kit\\schedules)"); } LIT.with(|lit| lit.clone()) };
            (v254.clone(), v254.clone())
        } else {
            let v255: Rc<str> = Rc::<str>::from(format!("{}{}", v250, v1));
            let v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("schedule due <name> <minutes> (exit 0 when due, 1 when not) | ran <name> | list (stamps under KIT_SCHEDULES_HOME, else LOCALAPPDATA\\kit\\schedules)"); } LIT.with(|lit| lit.clone()) };
            let v257: Rc<str> = Rc::<str>::from(format!("{}{}", v255, v256));
            (v257.clone(), v256.clone())
        };
        (v258.clone(), v259.clone())
    };
    v260.clone()
}
fn closure0() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        command_more_usages_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_command_more_usages() -> Rc<str> {
    closure0()()
}
