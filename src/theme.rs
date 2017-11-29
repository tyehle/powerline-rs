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
    pub hostname_bg: u8,
    pub hostname_fg: u8,

    pub jobs_bg: u8,
    pub jobs_fg: u8,

    pub time_bg: u8,
    pub time_fg: u8,

    pub ssh_bg: u8,
    pub ssh_fg: u8,

    pub ro_bg: u8,
    pub ro_fg: u8,

    pub git_clean_bg: u8,
    pub git_clean_fg: u8,
    pub git_dirty_bg: u8,
    pub git_dirty_fg: u8,
    pub git_ahead_bg:  u8,
    pub git_ahead_fg:  u8,
    pub git_behind_bg: u8,
    pub git_behind_fg: u8,
    pub git_conflicted_bg: u8,
    pub git_conflicted_fg: u8,
    pub git_notstaged_bg: u8,
    pub git_notstaged_fg: u8,
    pub git_staged_bg:    u8,
    pub git_staged_fg:    u8,
    pub git_untracked_bg: u8,
    pub git_untracked_fg: u8,

    pub cmd_passed_bg: u8,
    pub cmd_passed_fg: u8,
    pub cmd_failed_bg: u8,
    pub cmd_failed_fg: u8
}

pub const DEFAULT: Theme = Theme {
    separator_fg: 244,

    home_bg: 31,
    home_fg: 15,
    path_bg: 237,
    path_fg: 250,
    cwd_fg: 254,

    username_bg: 240,
    username_fg: 250,
    username_root_bg: 124,
    hostname_bg: 238,
    hostname_fg: 250,

    jobs_bg: 238,
    jobs_fg: 39,

    time_bg: 238,
    time_fg: 250,

    ssh_bg: 166,
    ssh_fg: 254,

    ro_bg: 124,
    ro_fg: 254,

    git_clean_bg: 148,
    git_clean_fg: 0,
    git_dirty_bg: 161,
    git_dirty_fg: 15,
    git_ahead_bg: 240,
    git_ahead_fg: 250,
    git_behind_bg: 240,
    git_behind_fg: 250,
    git_conflicted_bg: 9,
    git_conflicted_fg: 15,
    git_notstaged_bg: 130,
    git_notstaged_fg: 15,
    git_staged_bg: 22,
    git_staged_fg: 15,
    git_untracked_bg: 52,
    git_untracked_fg: 15,

    cmd_passed_bg: 236,
    cmd_passed_fg: 15,
    cmd_failed_bg: 161,
    cmd_failed_fg: 15
};