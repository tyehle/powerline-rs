use crate::{Powerline, Segment};

pub fn segment_root(p: &mut Powerline, error: u8) {
    if error == 0 {
        p.segments.push(Segment::new(p.theme.cmd_passed_bg, p.theme.cmd_passed_fg, "𒁍 "));
        return;
    }
    p.segments.push(Segment::new(p.theme.cmd_failed_bg, p.theme.cmd_failed_fg, error.to_string()));
}
