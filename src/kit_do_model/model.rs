#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
pub enum replace_count {
    replace_count_Once,
    replace_count_Every,
    replace_count_Exactly(u64),
}
#[derive(Clone)]
pub enum rule_matcher {
    rule_matcher_Literal,
    rule_matcher_Pattern,
}
#[derive(Clone)]
pub enum rule_list {
    rule_list_NoRules,
    rule_list_Rule(Rc<str>, Rc<str>, replace_count, rule_matcher, Rc<rule_list>),
}
#[derive(Clone)]
pub enum expectation {
    expectation_Unchecked,
    expectation_Expected(u64, u64),
}
#[derive(Clone)]
pub enum write_mode {
    write_mode_Apply,
    write_mode_Preview,
}
#[derive(Clone)]
pub enum query_verb {
    query_verb_Search,
    query_verb_Read,
    query_verb_Files,
    query_verb_Head,
    query_verb_Tail,
    query_verb_Wait,
}
#[derive(Clone)]
pub enum program_family {
    program_family_Edits,
    program_family_Reads,
    program_family_Facts,
    program_family_Runs,
    program_family_Writes,
    program_family_Git,
    program_family_Jobs,
    program_family_Labs,
}
#[derive(Clone)]
pub enum step {
    step_EditStep(Rc<str>, write_mode, expectation, Rc<str>, Rc<rule_list>),
    step_ShowStep(Rc<str>),
    step_QueryStep(Rc<str>, Rc<str>),
}
#[derive(Clone)]
pub enum steps {
    steps_Done,
    steps_Step(step, Rc<steps>),
}
impl replace_count {
    pub fn tag(&self) -> i32 {
        match self {
            replace_count::replace_count_Once => 0,
            replace_count::replace_count_Every => 1,
            replace_count::replace_count_Exactly(..) => 2,
        }
    }
}
impl rule_matcher {
    pub fn tag(&self) -> i32 {
        match self {
            rule_matcher::rule_matcher_Literal => 0,
            rule_matcher::rule_matcher_Pattern => 1,
        }
    }
}
impl rule_list {
    pub fn tag(&self) -> i32 {
        match self {
            rule_list::rule_list_NoRules => 0,
            rule_list::rule_list_Rule(..) => 1,
        }
    }
}
impl expectation {
    pub fn tag(&self) -> i32 {
        match self {
            expectation::expectation_Unchecked => 0,
            expectation::expectation_Expected(..) => 1,
        }
    }
}
impl write_mode {
    pub fn tag(&self) -> i32 {
        match self {
            write_mode::write_mode_Apply => 0,
            write_mode::write_mode_Preview => 1,
        }
    }
}
impl query_verb {
    pub fn tag(&self) -> i32 {
        match self {
            query_verb::query_verb_Search => 0,
            query_verb::query_verb_Read => 1,
            query_verb::query_verb_Files => 2,
            query_verb::query_verb_Head => 3,
            query_verb::query_verb_Tail => 4,
            query_verb::query_verb_Wait => 5,
        }
    }
}
impl program_family {
    pub fn tag(&self) -> i32 {
        match self {
            program_family::program_family_Edits => 0,
            program_family::program_family_Reads => 1,
            program_family::program_family_Facts => 2,
            program_family::program_family_Runs => 3,
            program_family::program_family_Writes => 4,
            program_family::program_family_Git => 5,
            program_family::program_family_Jobs => 6,
            program_family::program_family_Labs => 7,
        }
    }
}
impl step {
    pub fn tag(&self) -> i32 {
        match self {
            step::step_EditStep(..) => 0,
            step::step_ShowStep(..) => 1,
            step::step_QueryStep(..) => 2,
        }
    }
}
impl steps {
    pub fn tag(&self) -> i32 {
        match self {
            steps::steps_Done => 0,
            steps::steps_Step(..) => 1,
        }
    }
}
impl Default for replace_count {
    fn default() -> Self {
        replace_count::replace_count_Once
    }
}
impl Default for rule_matcher {
    fn default() -> Self {
        rule_matcher::rule_matcher_Literal
    }
}
impl Default for rule_list {
    fn default() -> Self {
        rule_list::rule_list_NoRules
    }
}
impl Default for expectation {
    fn default() -> Self {
        expectation::expectation_Unchecked
    }
}
impl Default for write_mode {
    fn default() -> Self {
        write_mode::write_mode_Apply
    }
}
impl Default for query_verb {
    fn default() -> Self {
        query_verb::query_verb_Search
    }
}
impl Default for program_family {
    fn default() -> Self {
        program_family::program_family_Edits
    }
}
impl Default for step {
    fn default() -> Self {
        step::step_EditStep({ thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, <write_mode>::default(), <expectation>::default(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, <Rc<rule_list>>::default())
    }
}
impl Default for steps {
    fn default() -> Self {
        steps::steps_Done
    }
}
fn closure0() -> Rc<dyn Fn(replace_count) -> replace_count> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(replace_count) -> replace_count> = Rc::new(move |mut v0: replace_count| -> replace_count {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(rule_matcher) -> rule_matcher> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(rule_matcher) -> rule_matcher> = Rc::new(move |mut v0: rule_matcher| -> rule_matcher {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<rule_list>) -> Rc<rule_list>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<rule_list>) -> Rc<rule_list>> = Rc::new(move |mut v0: Rc<rule_list>| -> Rc<rule_list> {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(expectation) -> expectation> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(expectation) -> expectation> = Rc::new(move |mut v0: expectation| -> expectation {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(write_mode) -> write_mode> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(write_mode) -> write_mode> = Rc::new(move |mut v0: write_mode| -> write_mode {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(query_verb) -> query_verb> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(query_verb) -> query_verb> = Rc::new(move |mut v0: query_verb| -> query_verb {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(program_family) -> program_family> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(program_family) -> program_family> = Rc::new(move |mut v0: program_family| -> program_family {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(step) -> step> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(step) -> step> = Rc::new(move |mut v0: step| -> step {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<steps>) -> Rc<steps>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<steps>) -> Rc<steps>> = Rc::new(move |mut v0: Rc<steps>| -> Rc<steps> {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(query_verb) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(query_verb) -> Rc<str>> = Rc::new(move |mut v0: query_verb| -> Rc<str> {
        match v0 {
            query_verb::query_verb_Files => {
                let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
                v3
            }
            query_verb::query_verb_Head => {
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("head"); } LIT.with(|lit| lit.clone()) };
                v4
            }
            query_verb::query_verb_Read => {
                let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read"); } LIT.with(|lit| lit.clone()) };
                v2
            }
            query_verb::query_verb_Search => {
                let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search"); } LIT.with(|lit| lit.clone()) };
                v1
            }
            query_verb::query_verb_Tail => {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
                v5
            }
            query_verb::query_verb_Wait => {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                v6
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure10() -> Rc<dyn Fn(program_family) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(program_family) -> Rc<str>> = Rc::new(move |mut v0: program_family| -> Rc<str> {
        match v0 {
            program_family::program_family_Edits => {
                let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edits"); } LIT.with(|lit| lit.clone()) };
                v1
            }
            program_family::program_family_Facts => {
                let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("facts"); } LIT.with(|lit| lit.clone()) };
                v3
            }
            program_family::program_family_Git => {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
                v6
            }
            program_family::program_family_Jobs => {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jobs"); } LIT.with(|lit| lit.clone()) };
                v7
            }
            program_family::program_family_Labs => {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("labs"); } LIT.with(|lit| lit.clone()) };
                v8
            }
            program_family::program_family_Reads => {
                let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reads"); } LIT.with(|lit| lit.clone()) };
                v2
            }
            program_family::program_family_Runs => {
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                v4
            }
            program_family::program_family_Writes => {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("writes"); } LIT.with(|lit| lit.clone()) };
                v5
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_model_replace_count(v0: replace_count) -> replace_count {
    closure0()(v0)
}
pub fn kit_do_model_rule_matcher(v0: rule_matcher) -> rule_matcher {
    closure1()(v0)
}
pub fn kit_do_model_rule_list(v0: Rc<rule_list>) -> Rc<rule_list> {
    closure2()(v0)
}
pub fn kit_do_model_expectation(v0: expectation) -> expectation {
    closure3()(v0)
}
pub fn kit_do_model_write_mode(v0: write_mode) -> write_mode {
    closure4()(v0)
}
pub fn kit_do_model_query_verb(v0: query_verb) -> query_verb {
    closure5()(v0)
}
pub fn kit_do_model_program_family(v0: program_family) -> program_family {
    closure6()(v0)
}
pub fn kit_do_model_step(v0: step) -> step {
    closure7()(v0)
}
pub fn kit_do_model_steps(v0: Rc<steps>) -> Rc<steps> {
    closure8()(v0)
}
pub fn kit_do_model_query_verb_word(v0: query_verb) -> Rc<str> {
    closure9()(v0)
}
pub fn kit_do_model_program_family_word(v0: program_family) -> Rc<str> {
    closure10()(v0)
}
