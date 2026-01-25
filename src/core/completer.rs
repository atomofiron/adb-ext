use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

#[derive(Clone)]
pub struct CmdHelper {
    cmds: Vec<String>,
}

impl Helper for CmdHelper {}
impl Hinter for CmdHelper { type Hint = String; }
impl Highlighter for CmdHelper {}
impl Validator for CmdHelper {}

impl CmdHelper {
    pub fn from(commands: &[&str]) -> Self {
        CmdHelper {
            cmds: commands.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
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
