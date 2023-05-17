use crate::Shell;

pub fn fg(shell: Shell, color: u8) -> String {
    sgr_code(shell, &format!("38;5;{}", color))
}

pub fn bg(shell: Shell, color: u8) -> String {
    sgr_code(shell, &format!("48;5;{}", color))
}

pub fn reset(shell: Shell, fg: bool) -> String {
    sgr_code(shell, if fg {"39"} else {"49"})
}

pub fn as_bold(shell: Shell, text: &str) -> String {
    sgr_code(shell, "1") + text + &sgr_code(shell, "22")
}

pub fn sgr_code(shell: Shell, code: &str) -> String {
    match shell {
        Shell::Bare => format!("\x1b[{}m", code),
        Shell::Bash => format!("\\[\\e[{}m\\]", code),
        Shell::Zsh  => format!("%{{\x1b[{}m%}}", code)
    }
}

// pub fn root(shell: Shell) -> &'static str {
//     match shell {
//         Shell::Bare => "$",
//         Shell::Bash => "\\$",
//         Shell::Zsh  => "%#"
//     }
// }

pub fn escape(shell: Shell, string: &mut String) {
    if shell == Shell::Bare {
        return;
    }
    let mut output = String::with_capacity(string.len());
    for c in string.chars() {
        match shell {
            Shell::Bash => match c {
                '\\' => output.push_str("\\\\"),
                '$'  => output.push_str("\\$"),
                '"'  => output.push_str("\\\""),
                c    => output.push(c)
            },
            Shell::Zsh => match c {
                '%' => output.push_str("%%"),
                ')' => output.push_str("%)"),
                c   => output.push(c)
            },
            Shell::Bare => unreachable!()
        }
    }
    *string = output;
}
