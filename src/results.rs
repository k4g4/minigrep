use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
};

pub struct MinigrepResults {
    quiet: bool,
    findings: HashMap<PathBuf, Vec<String>>,
    ignored_dirs: Vec<PathBuf>,
}

impl MinigrepResults {
    pub fn new(quiet: bool) -> Self {
        let findings = HashMap::new();
        Self {
            quiet,
            findings,
            ignored_dirs: vec![],
        }
    }

    pub fn add_findings(&mut self, file_path: &Path, mut file_findings: Vec<String>) {
        self.findings
            .entry(file_path.to_path_buf())
            .and_modify(|f| f.append(&mut file_findings))
            .or_insert(file_findings);
    }

    pub fn findings(&self) -> &HashMap<PathBuf, Vec<String>> {
        &self.findings
    }

    pub fn add_ignored_dir(&mut self, dir_path: &Path) {
        self.ignored_dirs.push(dir_path.to_path_buf());
    }
}

impl Debug for MinigrepResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.quiet {
            if !self.ignored_dirs.is_empty() {
                writeln!(
                    f,
                    "Ignoring {} dir(s): (use -r to search into directories)",
                    self.ignored_dirs.len()
                )?;
                for dir_path in &self.ignored_dirs {
                    writeln!(f, "\t- {}", dir_path.display())?;
                }
            }
            writeln!(f, "Files searched: {}", self.findings.len())?;
            let total = self
                .findings
                .iter()
                .fold(0, |sum, (_, list)| sum + list.len());
            writeln!(f, "Total lines matched: {}", total)?;
        }
        for (file_path, findings_list) in &self.findings {
            writeln!(f)?;
            if !self.quiet {
                if findings_list.is_empty() {
                    writeln!(f, "No results in {}", file_path.display())?;
                } else {
                    writeln!(f, "{}:", file_path.display())?;
                }
            }
            for finding in findings_list {
                writeln!(f, "{finding}")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod results_tests {
    use super::*;

    #[test]
    fn results_add_finding_succeeds() {
        let mut results = MinigrepResults::new(false);
        results.add_findings(&PathBuf::from("test.txt"), vec!["finding 1".to_string()]);
        results.add_findings(&PathBuf::from("test.txt"), vec!["finding 2".to_string()]);

        assert_eq!(results.findings.len(), 1);
        assert_eq!(results.findings[&PathBuf::from("test.txt")].len(), 2);
    }
}
