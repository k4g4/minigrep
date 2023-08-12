use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
};

pub struct MinigrepResults {
    quiet: bool,
    findings: HashMap<PathBuf, Vec<String>>,
}

impl MinigrepResults {
    pub fn new(quiet: bool) -> Self {
        let findings = HashMap::new();
        Self { quiet, findings }
    }

    pub fn add_file(&mut self, file_path: &Path) {
        self.findings.insert(file_path.to_path_buf(), vec![]);
    }

    pub fn add_finding(&mut self, file_path: &Path, finding: &str) {
        if let Some(findings_list) = self.findings.get_mut(file_path) {
            findings_list.push(finding.to_string());
        } else {
            self.findings
                .insert(file_path.to_path_buf(), vec![finding.to_string()]);
        }
    }

    pub fn findings(&self) -> &HashMap<PathBuf, Vec<String>> {
        &self.findings
    }
}

impl Debug for MinigrepResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.quiet {
            writeln!(f, "Files searched: {}", self.findings.len())?;
        }
        for (file_path, findings_list) in &self.findings {
            if !self.quiet {
                writeln!(f, "\tFrom file '{}':", file_path.display())?;
            }
            for finding in findings_list {
                if !self.quiet {
                    write!(f, "\t\t")?;
                }
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
        results.add_finding(&PathBuf::from("test.txt"), "finding 1");
        results.add_finding(&PathBuf::from("test.txt"), "finding 2");

        assert_eq!(results.findings.len(), 1);
        assert_eq!(results.findings[&PathBuf::from("test.txt")].len(), 2);
    }
}
