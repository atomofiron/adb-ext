use crate::core::ext::try_make_colored;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;
use termcolor::Color;

pub type CmdEditor = Editor<CmdHelper, DefaultHistory>;

pub type CmdHighlight = Rc<RefCell<Option<(bool, Range<usize>)>>>;

pub struct CmdHelper {
    cmds: Vec<String>,
    pub success: CmdHighlight,
}

impl Helper for CmdHelper {}

impl Validator for CmdHelper {}

impl Hinter for CmdHelper {
    type Hint = String;
}

impl Highlighter for CmdHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        match self.success.borrow().as_ref().cloned() {
            None => Borrowed(prompt),
            Some((true, range)) => Cow::Owned(try_make_colored(prompt, Color::Green, range)),
            Some((false, range)) => Cow::Owned(try_make_colored(prompt, Color::Red, range)),
        }
    }
}

impl CmdHelper {

    pub fn from(commands: &[&str], success: CmdHighlight) -> Self {
        CmdHelper {
            cmds: commands.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            success,
        }
    }
}

impl Completer for CmdHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let start = line[..pos]
            .rfind(|c: char| c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);

        let prefix = &line[start..pos];
        let mut out = Vec::new();
        for cmd in &self.cmds {
            if cmd.starts_with(prefix) {
                out.push(Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                });
            }
        }
        Ok((start, out))
    }
}
