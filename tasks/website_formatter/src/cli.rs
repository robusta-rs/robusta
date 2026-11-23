use bpaf::Parser;
use robusta_cli::cli::{
    Command, check_command, config_command, format_command, init_command, robusta,
};
use website_common::generate_cli_docs;

#[expect(clippy::print_stdout)]
pub fn print_cli() {
    println!("{}", generate_cli());
}

/// Render the commands that format, and the shared configuration commands, as
/// website-ready markdown.
fn generate_cli() -> String {
    let check = check_command().map(Command::Check);
    let format = format_command().map(Command::Format);
    let init = init_command().map(Command::Init);
    let config = config_command().map(Command::Config);
    let commands = robusta(bpaf::construct!([check, format, init, config]));
    generate_cli_docs(&commands.render_markdown("robusta"))
}

#[cfg(test)]
mod tests {
    use super::generate_cli;

    #[test]
    fn cli_reference_matches_the_snapshot() {
        let reference = generate_cli();
        assert!(reference.starts_with("---\nsearch: false\n---\n"));
        assert!(!reference.lines().any(|line| line.starts_with("# ")));
        insta::with_settings!({ prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(reference);
        });
    }
}
