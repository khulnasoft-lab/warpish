mod generated_workflows;

pub use generated_workflows::workflows;
pub use starterm_workflows_types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflows_generated() {
        let workflow_list = workflows();
        assert_eq!(workflow_list.len(), 3, "Should have 3 workflows generated");

        // Test first workflow (backup_installed_dependencies)
        let first_workflow = &workflow_list[0];
        assert_eq!(
            first_workflow.name(),
            "Store / Backup all installed dependencies into Brewfile"
        );
        assert_eq!(first_workflow.command(), "brew bundle dump");
        assert!(first_workflow.tags().contains(&"homebrew".to_string()));

        // Test second workflow (update_all_packages)
        let second_workflow = &workflow_list[1];
        assert_eq!(second_workflow.name(), "Update all Homebrew packages");
        assert_eq!(second_workflow.command(), "brew update && brew upgrade");
        assert!(second_workflow.tags().contains(&"homebrew".to_string()));
        assert!(second_workflow.tags().contains(&"system".to_string()));

        // Test third workflow (commit_with_message) - with arguments
        let third_workflow = &workflow_list[2];
        assert_eq!(third_workflow.name(), "Git commit with message");
        assert_eq!(third_workflow.command(), "git add . && git commit -m '{{message}}'");
        assert!(third_workflow.tags().contains(&"git".to_string()));
        assert!(third_workflow.tags().contains(&"version-control".to_string()));

        // Test arguments
        assert_eq!(third_workflow.arguments().len(), 1);
        let arg = &third_workflow.arguments()[0];
        assert_eq!(arg.name(), "message");
        assert_eq!(arg.description(), &Some("Commit message".to_string()));
        assert_eq!(arg.default_value(), &Some("Update".to_string()));
    }
}
