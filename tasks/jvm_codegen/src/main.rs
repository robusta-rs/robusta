use std::{fmt::Write as _, fs, io, path::PathBuf};

use robusta_config::PRUNED_TRAVERSAL_DIRS;

const OUTPUT_PATH: &str =
    "jvm/common/src/main/generated/rs/robusta/plugin/common/PrunedDirectories.java";

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn generate() -> String {
    let mut source = String::from(concat!(
        "// Auto-generated code, DO NOT EDIT DIRECTLY!\n",
        "// To regenerate: `just jvmgen`\n\n",
        "package rs.robusta.plugin.common;\n\n",
        "final class PrunedDirectories {\n",
        "    private PrunedDirectories() {}\n\n",
        "    static boolean contains(String name) {\n",
        "        switch (name) {\n",
    ));
    for directory in PRUNED_TRAVERSAL_DIRS {
        writeln!(
            source,
            "            case {}:",
            serde_json::to_string(directory).expect("serialize directory name")
        )
        .unwrap();
    }
    source.push_str(concat!(
        "                return true;\n",
        "            default:\n",
        "                return false;\n",
        "        }\n",
        "    }\n",
        "}\n",
    ));
    source
}

fn main() -> io::Result<()> {
    let path = workspace_root().join(OUTPUT_PATH);
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, generate())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use cow_utils::CowUtils;

    use super::{OUTPUT_PATH, generate, workspace_root};

    #[test]
    fn committed_policy_matches_generated_output() {
        let committed = fs::read_to_string(workspace_root().join(OUTPUT_PATH))
            .expect("generated policy should be committed");
        assert_eq!(
            committed.cow_replace("\r\n", "\n"),
            generate(),
            "JVM traversal policy is stale; run `just jvmgen`"
        );
    }
}
