//! Enhanced Agent Mode Evaluation
//! 
//! This module handles evaluation and assessment of AI agent performance,
//! supporting detailed metrics collection, response quality analysis, and performance optimization.

use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvaluation {
    pub agent_id: String,
    pub session_id: String,
    pub response_quality: f32,
    pub execution_time: u64, // in milliseconds
    pub user_satisfaction: Option<f32>,
    pub error_count: u32,
    pub timestamp: u64,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationMetrics {
    pub accuracy: f32,
    pub average_response_time: u64,
    pub success_rate: f32,
    pub user_engagement: f32,
    pub sessions_evaluated: u32,
}

pub struct AgentEvaluator {
    evaluations: Vec<AgentEvaluation>,
    start_time: Instant,
}

impl AgentEvaluator {
    pub fn new() -> Self {
        Self {
            evaluations: Vec::new(),
            start_time: Instant::now(),
        }
    }

    pub fn start_evaluation(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn evaluate_response(&mut self, response: &str, context: &str, satisfaction: Option<f32>) -> AgentEvaluation {
        let execution_time = self.start_time.elapsed().as_millis() as u64;
        
        let evaluation = AgentEvaluation {
            agent_id: "agent-001".to_string(),
            session_id: uuid::Uuid::new_v4().to_string(),
            response_quality: self.calculate_quality(response),
            execution_time,
            user_satisfaction: satisfaction,
            error_count: self.count_errors(response),
            timestamp: chrono::Utc::now().timestamp() as u64,
            context: context.to_string(),
        };

        self.evaluations.push(evaluation.clone());
        evaluation
    }

    fn calculate_quality(&self, response: &str) -> f32 {
        // Basic example logic: longer responses considered higher quality
        (response.len() as f32 / 1000.0).min(1.0)
    }

    fn count_errors(&self, response: &str) -> u32 {
        // Example logic to count occurrences of the word "error"
        response.matches("error").count() as u32
    }

    pub fn get_metrics(&self) -> EvaluationMetrics {
        let total_sessions = self.evaluations.len() as u32;
        let total_time: u64 = self.evaluations.iter().map(|e| e.execution_time).sum();

        EvaluationMetrics {
            accuracy: self.calculate_accuracy(),
            average_response_time: if total_sessions > 0 { total_time / total_sessions } else { 0 },
            success_rate: self.calculate_success_rate(),
            user_engagement: self.calculate_engagement(),
            sessions_evaluated: total_sessions,
        }
    }

    fn calculate_accuracy(&self) -> f32 {
        // Fake accuracy calculation for demonstration
        0.95
    }

    fn calculate_success_rate(&self) -> f32 {
        let successful_sessions = self.evaluations.iter().filter(|e| e.response_quality > 0.7).count() as f32;
        successful_sessions / self.evaluations.len() as f32
    }

    fn calculate_engagement(&self) -> f32 {
        // Example engagement calculation
        0.85
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_evaluator_accuracy() {
        let mut evaluator = AgentEvaluator::new();
        evaluator.start_evaluation();
        sleep(Duration::from_millis(100));
        let eval = evaluator.evaluate_response("This response is fine.", "test context", Some(0.9));
        assert_eq!(eval.execution_time > 0, true);

        let metrics = evaluator.get_metrics();
        assert!(metrics.accuracy >= 0.0 && metrics.accuracy <= 1.0);
    }

    #[test]
    fn test_evaluator_metrics() {
        let mut evaluator = AgentEvaluator::new();
        for _ in 0..10 {
            evaluator.start_evaluation();
            sleep(Duration::from_millis(50));
            evaluator.evaluate_response("Another response with error", "sample context", Some(0.8));
        }

        let metrics = evaluator.get_metrics();
        assert_eq!(metrics.sessions_evaluated, 10);
        assert!(metrics.average_response_time > 0);
    }
}
