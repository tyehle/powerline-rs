pub mod segment_cwd;
pub mod segment_host;
pub mod segment_jobs;
pub mod segment_nix;
pub mod segment_perms;
pub mod segment_ps;
pub mod segment_root;
pub mod segment_ssh;
pub mod segment_time;
pub mod segment_user;
pub mod segment_virtualenv;
pub mod segment_linebreak;

pub use self::segment_cwd::*;
pub use self::segment_host::*;
pub use self::segment_jobs::*;
pub use self::segment_nix::*;
pub use self::segment_perms::*;
pub use self::segment_ps::*;
pub use self::segment_root::*;
pub use self::segment_ssh::*;
pub use self::segment_time::*;
pub use self::segment_user::*;
pub use self::segment_virtualenv::*;
pub use self::segment_linebreak::*;

#[cfg(feature = "git2")] pub mod segment_git;
#[cfg(feature = "git2")] pub use self::segment_git::*;


use crate::Shell;
use crate::format::*;
use std::borrow::Cow;
use crate::theme::Theme;

pub struct Segment {
    bg: u8,
    fg: u8,

    before: &'static str,
    after: &'static str,
    conditional: bool,
    no_space_after: bool,
    bold: bool,

    escaped: bool,
    text: Cow<'static, str>
}
impl Segment {
    pub fn new<S>(bg: u8, fg: u8, text: S) -> Self
        where S: Into<Cow<'static, str>>
    {
        Segment {
            bg,
            fg,

            before: "",
            after: "",
            conditional: false,
            no_space_after: false,
            bold: false,

            escaped: false,
            text:  text.into()
        }
    }
    pub fn dont_escape(mut self) -> Self {
        self.escaped = true;
        self
    }
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    pub fn with_before(mut self, before: &'static str) -> Self {
        self.before = before;
        self
    }
    pub fn with_after(mut self, after: &'static str) -> Self {
        self.after = after;
        self
    }
    pub fn into_conditional(mut self) -> Self {
        self.conditional = true;
        self
    }
    pub fn is_conditional(&self) -> bool {
        self.conditional
    }
    pub fn with_no_space_after(mut self) -> Self {
        self.no_space_after = true;
        self
    }
    pub fn escape(&mut self, shell: Shell) {
        if self.escaped {
            return;
        }
        escape(shell, self.text.to_mut());
        self.escaped = true;
    }
    pub fn print(&self, next: Option<&Segment>, shell: Shell, _theme: &Theme) {
        print!("{}{}{} ", self.before, fg(shell, self.fg), bg(shell, self.bg));

        if self.bold {
            print!("{}", as_bold(shell, &self.text));
        } else {
            print!("{}", self.text);
        }

        if !self.no_space_after && (next.map(|n| n.bg != self.bg).unwrap_or(true)) {
            print!(" ")
        }
        match next {
            Some(next) if next.is_conditional() => {},
            Some(next) if next.bg == self.bg => {}, //print!("{}\u{e0b5}", fg(shell, theme.separator_fg)),
            Some(next) if self.bg == 0 => print!("{}{}\u{e0c6}", fg(shell, next.bg), bg(shell, next.bg)),
            Some(next) => print!("{}{}\u{e0c6}", fg(shell, self.bg), bg(shell, next.bg)),
            // Last tile resets colors
            None       => print!("{}{}\u{e0c6}{}", fg(shell, self.bg), reset(shell, false), reset(shell, true))
        }
        print!("{}", self.after);
    }
    pub fn print_rtl(&self, next: Option<&Segment>, shell: Shell, _theme: &Theme) {
        // Here, next is going leftwards - see how this func is called in main.rs .
        print!("{}", self.after);
        match next {
            Some(next) if next.is_conditional() => {},
            Some(next) if next.bg == self.bg => {},
                // print!("{}{}", fg(shell, theme.separator_fg), bg(shell, self.bg)),
            Some(next) => print!("{}{}",  fg(shell, self.bg), bg(shell, next.bg)),
            None       => print!("{}", fg(shell, self.bg))
        }
        print!("{}{} {}", fg(shell, self.fg), bg(shell, self.bg), self.text);

        if !self.no_space_after {
            print!(" ")
        }
        print!("{}{}{}", reset(shell, false), reset(shell, true), self.before);
    }
}
