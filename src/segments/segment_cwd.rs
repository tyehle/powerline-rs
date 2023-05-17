use crate::{format, Powerline, Segment, Shell};
use std::{env, path::PathBuf};

fn simple_cwd_string(shell: Shell, cwd_max_depth: u8) -> String {
    // are we in the home dir?
    let mut in_home = false;
    let mut path = env::current_dir().unwrap_or_else(|_| PathBuf::from("error"));
    if let Some(home) = dirs::home_dir() {
        if let Ok(new) = path.strip_prefix(&home) {
            in_home = true;
            path = new.to_path_buf();
        } else if let Ok(new) = path.strip_prefix("/") {
            path = new.to_path_buf();
        }
    }

    let length = path.iter().count();

    // are there too many elements in the path?
    let to_skip = if cwd_max_depth > 0 && length > cwd_max_depth as usize {
        length - cwd_max_depth as usize
    } else {
        0
    };

    if length == 0 {
        let loc = if in_home { "~" } else { "/" };
        return format::as_bold(shell, loc);
    }

    let mut out = if in_home { "~".to_owned() } else { "".to_owned() };

    if to_skip > 0 {
        out += "/…"
    }

    let mut path_iter = path.iter().skip(to_skip);
    let mut next = path_iter.next();
    while let Some(dir) = next {
        next = path_iter.next();
        out += "/";
        let name = &dir.to_string_lossy();
        if next.is_none() {
            out += &format::as_bold(shell, name);
        } else {
            out += &name;
        }
    }

    return out;
}

pub fn segment_cwd(p: &mut Powerline, cwd_max_depth: u8) {
    p.segments.push(Segment::new(p.theme.path_bg, p.theme.path_fg, simple_cwd_string(p.shell, cwd_max_depth)).dont_escape());
}
