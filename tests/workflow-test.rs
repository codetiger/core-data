use serde_json::json;

use core_data::models::workflow::*;
use core_data::models::task::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_builder_defaults() {
        let workflow = WorkflowBuilder::default().build().unwrap();

        assert_eq!(workflow.name, "");
        assert_eq!(workflow.description, "");
        assert_eq!(workflow.version, 0);
        assert!(workflow.tags.is_empty());
        assert_eq!(workflow.status, WorkflowStatus::Draft);
        assert!(workflow.tasks.is_empty());
    }

    #[test]
    fn test_workflow_custom_values() {
        let task = TaskBuilder::default()
            .task_id(String::from("task_1"))
            .name(String::from("Task 1"))
            .description(String::from("First task"))
            .trigger_condition(json!({"condition": "value"}))
            .function(FunctionType::Validate)
            .build()
            .unwrap();

        let workflow = WorkflowBuilder::default()
            .name(String::from("Workflow 1"))
            .description(String::from("Test workflow"))
            .version(1)
            .tags(vec![String::from("tag1"), String::from("tag2")])
            .status(WorkflowStatus::Active)
            .tasks(vec![task.clone()])
            .build()
            .unwrap();

        assert_eq!(workflow.name, String::from("Workflow 1"));
        assert_eq!(workflow.description, String::from("Test workflow"));
        assert_eq!(workflow.version, 1);
        assert_eq!(workflow.tags, vec![String::from("tag1"), String::from("tag2")]);
        assert_eq!(workflow.status, WorkflowStatus::Active);
        assert_eq!(workflow.tasks.len(), 1);
        assert_eq!(workflow.tasks[0], task);
    }

    #[test]
    fn test_workflow_empty_tasks() {
        let workflow = WorkflowBuilder::default()
            .name(String::from("Empty Workflow"))
            .description(String::from("Workflow with no tasks"))
            .build()
            .unwrap();

        assert_eq!(workflow.name, String::from("Empty Workflow"));
        assert_eq!(workflow.description, String::from("Workflow with no tasks"));
        assert!(workflow.tasks.is_empty());
    }

    #[test]
    fn test_workflow_multiple_tasks() {
        let task1 = TaskBuilder::default()
            .task_id(String::from("task_1"))
            .name(String::from("Task 1"))
            .description(String::from("First task"))
            .trigger_condition(json!({"condition": "value"}))
            .function(FunctionType::Validate)
            .build()
            .unwrap();

        let task2 = TaskBuilder::default()
            .task_id(String::from("task_2"))
            .name(String::from("Task 2"))
            .description(String::from("Second task"))
            .trigger_condition(json!({"condition": "value"}))
            .function(FunctionType::Enrich)
            .build()
            .unwrap();

        let workflow = WorkflowBuilder::default()
            .name(String::from("Workflow with Multiple Tasks"))
            .description(String::from("Workflow containing multiple tasks"))
            .tasks(vec![task1.clone(), task2.clone()])
            .build()
            .unwrap();

        assert_eq!(workflow.name, String::from("Workflow with Multiple Tasks"));
        assert_eq!(workflow.description, String::from("Workflow containing multiple tasks"));
        assert_eq!(workflow.tasks.len(), 2);
        assert_eq!(workflow.tasks[0], task1);
        assert_eq!(workflow.tasks[1], task2);
    }
}