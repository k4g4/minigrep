use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
};

pub struct MinigrepResults {
    findings: HashMap<PathBuf, Vec<String>>,
}

impl MinigrepResults {
    pub fn new() -> Self {
        let findings = HashMap::new();
        Self { findings }
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
}

impl Display for MinigrepResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Files searched: {}", self.findings.len())?;
        writeln!(f, "")
    }
}

#[cfg(test)]
mod results_tests {
    use super::*;

    #[test]
    fn results_add_finding_succeeds() {
        let mut results = MinigrepResults::new();
        results.add_finding(&PathBuf::from("test.txt"), "finding 1");
        results.add_finding(&PathBuf::from("test.txt"), "finding 2");

        assert_eq!(results.findings.len(), 1);
        assert_eq!(results.findings[&PathBuf::from("test.txt")].len(), 2);
    }
}
