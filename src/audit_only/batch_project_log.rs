//! Per-project logs of a corpus batch run. Diagnostic projection only: each project's
//! stderr is kept in its own file and copied into the console log as one block after the
//! project ends, so parallel projects cannot interleave lines. No musical decision, gate
//! or exit code reads these files; every failure here only loses a copy of diagnostics.

use std::io::Write;
use std::path::{Path, PathBuf};

const PROJECT_LOG_DIR: &str = "target/etuschlopf_logs/projects";

/// File name of a project's own batch log: unicode letters and digits plus `.`, `-`, `_`
/// are kept, every other character becomes `_`.
fn project_log_file_name(project: &str) -> String {
    let stem = project
        .chars()
        .map(|ch| if ch.is_alphanumeric() || matches!(ch, '.' | '-' | '_') { ch } else { '_' })
        .collect::<String>();
    format!("{stem}.console.txt")
}

pub(crate) struct BatchProjectLog {
    file: Option<(PathBuf, std::fs::File)>,
}

impl BatchProjectLog {
    pub(crate) fn open(project: &str) -> Self {
        let dir = Path::new(PROJECT_LOG_DIR);
        let file = std::fs::create_dir_all(dir).ok().and_then(|()| {
            let path = dir.join(project_log_file_name(project));
            std::fs::File::create(&path).ok().map(|file| (path, file))
        });
        Self { file }
    }

    /// The child's stderr: the project log, or the inherited stream when no log exists.
    pub(crate) fn child_stderr(&self) -> std::process::Stdio {
        self.file
            .as_ref()
            .and_then(|(_, file)| file.try_clone().ok())
            .map_or_else(std::process::Stdio::inherit, std::process::Stdio::from)
    }

    /// Appends the finished project's log to this process's stderr (the console log).
    pub(crate) fn append_to_console(&self, project: &str) {
        let Some((path, _)) = &self.file else {
            return;
        };
        let Ok(body) = std::fs::read(path) else {
            return;
        };
        let _ = std::io::stderr().lock().write_all(&console_block(project, path, body));
    }
}

fn console_block(project: &str, path: &Path, mut body: Vec<u8>) -> Vec<u8> {
    if !body.is_empty() && !body.ends_with(b"\n") {
        body.push(b'\n');
    }
    let mut block = format!("===== PROJEKT-LOG {project} ({}) =====\n", path.display()).into_bytes();
    block.extend_from_slice(&body);
    block.extend_from_slice(format!("===== ENDE PROJEKT-LOG {project} =====\n").as_bytes());
    block
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_log_file_names_are_safe_and_keep_letters() {
        assert_eq!(project_log_file_name("Stück Ä 7"), "Stück_Ä_7.console.txt");
        assert_eq!(project_log_file_name("a/b\\c:d"), "a_b_c_d.console.txt");
        assert_eq!(project_log_file_name("x.y-z_1"), "x.y-z_1.console.txt");
    }

    #[test]
    fn console_block_frames_the_body_and_ends_every_line() {
        let block = console_block("P", Path::new("p.txt"), b"a\nb".to_vec());
        assert_eq!(
            String::from_utf8(block).unwrap(),
            "===== PROJEKT-LOG P (p.txt) =====\na\nb\n===== ENDE PROJEKT-LOG P =====\n"
        );
        let empty = console_block("P", Path::new("p.txt"), Vec::new());
        assert_eq!(
            String::from_utf8(empty).unwrap(),
            "===== PROJEKT-LOG P (p.txt) =====\n===== ENDE PROJEKT-LOG P =====\n"
        );
    }
}
