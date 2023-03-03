#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::{prelude::PredicateBooleanExt, str::contains};

    #[test]
    fn cli_shows_help_with_all_available_commands() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .assert()
            .failure()
            .stderr(
                contains("Usage:")
                    .and(contains("Commands:"))
                    .and(contains("Options:")),
            );
    }
}
