#[derive(Clone)]
pub struct Theme {
    pub separator_fg: u8,

    pub home_bg: u8,
    pub home_fg: u8,
    pub path_bg: u8,
    pub path_fg: u8,
    pub cwd_fg:  u8,

    pub username_bg: u8,
    pub username_fg: u8,
    pub username_root_bg: u8,
    pub username_root_fg: u8,
    pub hostname_bg: u8,
    pub hostname_fg: u8,

    pub jobs_bg: u8,
    pub jobs_fg: u8,

    pub time_bg: u8,
    pub time_fg: u8,

    pub ssh_bg: u8,
    pub ssh_fg: u8,

    pub ssh_char: char,

    pub ro_bg: u8,
    pub ro_fg: u8,

    pub ro_char: char,

    pub git_clean_bg: u8,
    pub git_clean_fg: u8,
    pub git_dirty_bg: u8,
    pub git_dirty_fg: u8,
    pub git_detached_bg: u8,
    pub git_detached_fg: u8,

    pub git_ahead_bg:  u8,
    pub git_ahead_fg:  u8,
    pub git_behind_bg: u8,
    pub git_behind_fg: u8,
    pub git_conflicted_bg: u8,
    pub git_conflicted_fg: u8,
    pub git_changed_bg:   u8,
    pub git_changed_fg:   u8,
    pub git_staged_bg:    u8,
    pub git_staged_fg:    u8,
    pub git_untracked_bg: u8,
    pub git_untracked_fg: u8,
    pub git_stashed_bg:   u8,
    pub git_stashed_fg:   u8,

    pub git_ahead_char: char,
    pub git_behind_char: char,
    pub git_staged_char: char,
    pub git_changed_char: char,
    pub git_untracked_char: char,
    pub git_conflicted_char: char,
    pub git_stashed_char: char,

    pub cmd_passed_bg: u8,
    pub cmd_passed_fg: u8,
    pub cmd_failed_bg: u8,
    pub cmd_failed_fg: u8,

    pub ps_bg: u8,
    pub ps_fg: u8,

    pub virtual_env_bg: u8,
    pub virtual_env_fg: u8,

    pub nixshell_bg: u8,
    pub nixshell_fg: u8,
}

pub const DEFAULT: Theme = Theme {
    separator_fg: 244,

    home_bg: 31,
    home_fg: 15,
    path_bg: 236,
    path_fg: 252,
    cwd_fg: 254,

    username_bg: 32,
    username_fg: 231,
    username_root_bg: 124,
    username_root_fg: 231,
    hostname_bg: 24,
    hostname_fg: 231,

    jobs_bg: 238,
    jobs_fg: 39,
    ps_bg: 238,
    ps_fg: 39,

    time_bg: 238,
    time_fg: 250,

    ssh_bg: 166,
    ssh_fg: 254,

    ssh_char: '',

    ro_bg: 172,
    ro_fg: 231,

    ro_char: '',

    git_clean_bg:    236,
    git_dirty_bg:    236,
    git_detached_bg: 236,
    git_clean_fg:    2,
    git_dirty_fg:    98,
    git_detached_fg: 220,

    git_ahead_bg:     236,
    git_behind_bg:    236,
    git_staged_bg:    236,
    git_changed_bg:   236,
    git_untracked_bg: 236,
    git_conflicted_bg: 236,
    git_stashed_bg:   236,
    git_ahead_fg:     248,
    git_behind_fg:    248,
    git_staged_fg:    2,
    git_changed_fg:   166,
    git_untracked_fg: 214,
    git_conflicted_fg: 160,
    git_stashed_fg:   31,

    git_ahead_char:     '⬆',
    git_behind_char:    '⬇',
    git_staged_char:    '●',
    git_changed_char:   '✚',
    git_untracked_char: '…',
    git_conflicted_char: '✖',
    git_stashed_char:   '⚑',

    cmd_passed_bg: 162,
    cmd_passed_fg: 231,
    cmd_failed_bg: 124,
    cmd_failed_fg: 231,

    virtual_env_bg: 233,
    virtual_env_fg: 162,

    nixshell_bg: 237,
    nixshell_fg: 130,
};

use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct ErrCorrupt;

impl StdError for ErrCorrupt {}
impl fmt::Display for ErrCorrupt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Corrupt theme file")
    }
}

pub fn load(file: &str) -> Result<Theme, Box<dyn StdError>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut theme = DEFAULT.clone();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.chars().all(char::is_whitespace) {
            continue;
        }
        let mut parts = line.splitn(2, '=');

        let variable = parts.next().map(|inner| inner.trim()).ok_or_else(|| ErrCorrupt)?;
        let value    = parts.next().map(|inner| inner.trim()).ok_or_else(|| ErrCorrupt)?;

        if variable.ends_with("char") {
            let index = theme_index_char(&mut theme, variable).ok_or_else(|| ErrCorrupt)?;

            if value.chars().count() == 1 {
                *index = value.parse()?;
            } else {
                let codepoint = u32::from_str_radix(value, 16)?;
                *index = std::char::from_u32(codepoint).ok_or_else(|| ErrCorrupt)?;
            }
        } else {
            let index = theme_index_u8(&mut theme, variable).ok_or_else(|| ErrCorrupt)?;
            *index = value.parse()?;
        }
    }

    Ok(theme)
}

fn theme_index_u8<'a>(theme: &'a mut Theme, name: &str) -> Option<&'a mut u8> {
    match name {
        "separator_fg" => Some(&mut theme.separator_fg),

        "home_bg" => Some(&mut theme.home_bg),
        "home_fg" => Some(&mut theme.home_fg),
        "path_bg" => Some(&mut theme.path_bg),
        "path_fg" => Some(&mut theme.path_fg),
        "cwd_fg"  => Some(&mut theme.cwd_fg),

        "username_bg" => Some(&mut theme.username_bg),
        "username_fg" => Some(&mut theme.username_fg),
        "username_root_bg" => Some(&mut theme.username_root_bg),
        "username_root_fg" => Some(&mut theme.username_root_fg),
        "hostname_bg" => Some(&mut theme.hostname_bg),
        "hostname_fg" => Some(&mut theme.hostname_fg),

        "jobs_bg" => Some(&mut theme.jobs_bg),
        "jobs_fg" => Some(&mut theme.jobs_fg),

        "time_bg" => Some(&mut theme.time_bg),
        "time_fg" => Some(&mut theme.time_fg),

        "ssh_bg" => Some(&mut theme.ssh_bg),
        "ssh_fg" => Some(&mut theme.ssh_fg),

        "ro_bg" => Some(&mut theme.ro_bg),
        "ro_fg" => Some(&mut theme.ro_fg),

        "git_clean_bg" => Some(&mut theme.git_clean_bg),
        "git_clean_fg" => Some(&mut theme.git_clean_fg),
        "git_dirty_bg" => Some(&mut theme.git_dirty_bg),
        "git_dirty_fg" => Some(&mut theme.git_dirty_fg),
        "git_detached_bg" => Some(&mut theme.git_detached_bg),
        "git_detached_fg" => Some(&mut theme.git_detached_fg),
        "git_ahead_bg" => Some(&mut theme.git_ahead_bg),
        "git_ahead_fg" => Some(&mut theme.git_ahead_fg),
        "git_behind_bg" => Some(&mut theme.git_behind_bg),
        "git_behind_fg" => Some(&mut theme.git_behind_fg),
        "git_conflicted_bg" => Some(&mut theme.git_conflicted_bg),
        "git_conflicted_fg" => Some(&mut theme.git_conflicted_fg),
        "git_changed_bg" => Some(&mut theme.git_changed_bg),
        "git_changed_fg" => Some(&mut theme.git_changed_fg),
        "git_staged_bg" => Some(&mut theme.git_staged_bg),
        "git_staged_fg" => Some(&mut theme.git_staged_fg),
        "git_untracked_bg" => Some(&mut theme.git_untracked_bg),
        "git_untracked_fg" => Some(&mut theme.git_untracked_fg),
        "git_stashed_bg" => Some(&mut theme.git_stashed_bg),
        "git_stashed_fg" => Some(&mut theme.git_stashed_fg),

        "cmd_passed_bg" => Some(&mut theme.cmd_passed_bg),
        "cmd_passed_fg" => Some(&mut theme.cmd_passed_fg),
        "cmd_failed_bg" => Some(&mut theme.cmd_failed_bg),
        "cmd_failed_fg" => Some(&mut theme.cmd_failed_fg),

        "virtual_env_bg" => Some(&mut theme.virtual_env_bg),
        "virtual_env_fg" => Some(&mut theme.virtual_env_fg),

        "nixshell_bg" => Some(&mut theme.nixshell_bg),
        "nixshell_fg" => Some(&mut theme.nixshell_fg),

        _ => None
    }
}

fn theme_index_char<'a>(theme: &'a mut Theme, name: &str) -> Option<&'a mut char> {
    match name {
        "ssh_char" => Some(&mut theme.ssh_char),
        "ro_char" => Some(&mut theme.ro_char),

        "git_ahead_char" => Some(&mut theme.git_ahead_char),
        "git_behind_char" => Some(&mut theme.git_behind_char),
        "git_staged_char" => Some(&mut theme.git_staged_char),
        "git_changed_char" => Some(&mut theme.git_changed_char),
        "git_untracked_char" => Some(&mut theme.git_untracked_char),
        "git_conflicted_char" => Some(&mut theme.git_conflicted_char),
        "git_stashed_char" => Some(&mut theme.git_stashed_char),

        _ => None
    }
}
