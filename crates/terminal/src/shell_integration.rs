use std::{collections::HashMap, fs, path::PathBuf};

pub(crate) const MARKER_PREFIX: &str = "__wu_shell_integration;";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandStatus {
    Pending,
    Running,
    Success,
    Failed(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandMark {
    pub scrollback_position: i32,
    pub status: CommandStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellMarker {
    CommandStarted,
    CommandFinished(i32),
}

pub(crate) fn parse_marker(title: &str) -> Option<ShellMarker> {
    let rest = title.strip_prefix(MARKER_PREFIX)?;
    if rest == "run" {
        return Some(ShellMarker::CommandStarted);
    }
    let code = rest.strip_prefix("done;")?;
    Some(ShellMarker::CommandFinished(code.trim().parse().unwrap_or(0)))
}

const ZSHENV: &str = r#"WU_INJECT_ZDOTDIR=$ZDOTDIR
ZDOTDIR=${WU_USER_ZDOTDIR:-$HOME}
[[ -r $ZDOTDIR/.zshenv ]] && . $ZDOTDIR/.zshenv
ZDOTDIR=$WU_INJECT_ZDOTDIR
"#;

const ZPROFILE: &str = r#"ZDOTDIR=${WU_USER_ZDOTDIR:-$HOME}
[[ -r $ZDOTDIR/.zprofile ]] && . $ZDOTDIR/.zprofile
ZDOTDIR=$WU_INJECT_ZDOTDIR
"#;

const ZSHRC: &str = r#"[[ $HISTFILE == $WU_INJECT_ZDOTDIR/* ]] && HISTFILE=${WU_USER_ZDOTDIR:-$HOME}/.zsh_history
ZDOTDIR=${WU_USER_ZDOTDIR:-$HOME}
[[ -r $ZDOTDIR/.zshrc ]] && . $ZDOTDIR/.zshrc
__wu_precmd() {
  local __wu_status=$?
  print -rn -- $'\e]2;__wu_shell_integration;done;'$__wu_status$'\a'
  return $__wu_status
}
__wu_preexec() {
  print -rn -- $'\e]2;__wu_shell_integration;run\a'
}
precmd_functions=(__wu_precmd ${precmd_functions[@]})
preexec_functions=(${preexec_functions[@]} __wu_preexec)
unset WU_INJECT_ZDOTDIR
"#;

pub(crate) fn inject_zsh<S: std::hash::BuildHasher>(
    env: &mut HashMap<String, String, S>,
    version: &str,
) -> bool {
    let directory: PathBuf = std::env::temp_dir().join(format!("wu-shell-integration-{version}"));
    let written = fs::create_dir_all(&directory).is_ok()
        && [
            (".zshenv", ZSHENV),
            (".zprofile", ZPROFILE),
            (".zshrc", ZSHRC),
        ]
        .iter()
        .all(|(name, contents)| fs::write(directory.join(name), contents).is_ok());
    if !written {
        return false;
    }

    let user_zdotdir = env
        .get("ZDOTDIR")
        .cloned()
        .or_else(|| std::env::var("ZDOTDIR").ok());
    if let Some(user_zdotdir) = user_zdotdir {
        env.insert("WU_USER_ZDOTDIR".to_string(), user_zdotdir);
    }
    env.insert(
        "ZDOTDIR".to_string(),
        directory.to_string_lossy().into_owned(),
    );
    true
}

pub(crate) fn is_zsh(program: &str) -> bool {
    std::path::Path::new(program)
        .file_name()
        .is_some_and(|name| name == "zsh")
}
