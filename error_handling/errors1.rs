// errors1.rs
// this function refuses to generate text to be printed on a nametag if
// you pass it an empty string. it'd be nicer if it explained what the problem
// was, instead of just sometimes returning `none`. thankfully, rust has a similar
// construct to `option` that can be used to express error conditions. let's use it!
// execute `rustlings hint errors1` or use the `hint` watch subcommand for a hint.

pub fn generate_nametag_text(name: string) -> Result<string, &'static str> {
    if name.is_empty() {
        // empty names aren't allowed.
        err("`name` was empty; it must be nonempty.")
    } else {
        ok(format!("hi! my name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("beyoncé".into()),
            ok("hi! my name is beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // don't change this line
            err("`name` was empty; it must be nonempty.".into())
        );
    }
}
