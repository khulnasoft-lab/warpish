//! Multiple Choice Question Handling
//! 
//! This module provides functionality to parse, present, and evaluate multiple choice questions in a terminal environment.

use std::collections::HashMap;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleChoiceQuestion {
    pub id: String,
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub difficulty: QuestionDifficulty,
    pub category: String,
    pub explanation: Option<String>,
    pub time_limit: Option<u32>, // in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionResponse {
    pub question_id: String,
    pub selected_answer: usize,
    pub is_correct: bool,
    pub response_time: u32, // in seconds
    pub timestamp: DateTime<Utc>,
}

impl MultipleChoiceQuestion {
    pub fn new(
        id: String,
        question: String,
        options: Vec<String>,
        correct_answer: usize,
        difficulty: QuestionDifficulty,
        category: String,
    ) -> Self {
        Self {
            id,
            question,
            options,
            correct_answer,
            difficulty,
            category,
            explanation: None,
            time_limit: None,
        }
    }

    pub fn with_explanation(mut self, explanation: String) -> Self {
        self.explanation = Some(explanation);
        self
    }

    pub fn with_time_limit(mut self, time_limit: u32) -> Self {
        self.time_limit = Some(time_limit);
        self
    }

    pub fn ask(&self) -> Option<usize> {
        println!("\n{}", self.question);
        println!("Category: {} | Difficulty: {:?}", self.category, self.difficulty);
        if let Some(time_limit) = self.time_limit {
            println!("Time limit: {} seconds", time_limit);
        }
        println!();
        
        for (index, option) in self.options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }

        loop {
            print!("\nSelect your answer (1-{}): ", self.options.len());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(num) if num > 0 && num <= self.options.len() => return Some(num - 1),
                _ => println!("Invalid selection, try again."),
            }
        }
    }

    pub fn evaluate(&self, answer: usize) -> bool {
        answer == self.correct_answer
    }
    
    pub fn get_correct_answer_text(&self) -> &str {
        &self.options[self.correct_answer]
    }
}

/// A struct to manage quizzing sessions
pub struct Quiz {
    pub questions: Vec<MultipleChoiceQuestion>,
    pub responses: Vec<QuestionResponse>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Quiz {
    pub fn new() -> Self {
        Self {
            questions: Vec::new(),
            responses: Vec::new(),
            start_time: None,
            end_time: None,
        }
    }

    pub fn add_question(&mut self, question: MultipleChoiceQuestion) {
        self.questions.push(question);
    }

    pub fn conduct(&mut self) {
        if self.questions.is_empty() {
            println!("No questions to conduct.");
            return;
        }

        self.start_time = Some(Utc::now());
        
        println!("\n=== Quiz Started ===");
        println!("Total questions: {}", self.questions.len());
        
        for (index, question) in self.questions.iter().enumerate() {
            println!("\n--- Question {} of {} ---", index + 1, self.questions.len());
            
            let question_start = Utc::now();
            let answer = question.ask();
            let question_end = Utc::now();
            
            if let Some(selected) = answer {
                let is_correct = question.evaluate(selected);
                let response_time = (question_end - question_start).num_seconds() as u32;
                
                let response = QuestionResponse {
                    question_id: question.id.clone(),
                    selected_answer: selected,
                    is_correct,
                    response_time,
                    timestamp: question_end,
                };
                
                self.responses.push(response);
                
                if is_correct {
                    println!("✓ Correct!");
                } else {
                    println!("✗ Incorrect. The correct answer was: {}", question.get_correct_answer_text());
                }
                
                if let Some(explanation) = &question.explanation {
                    println!("Explanation: {}", explanation);
                }
            }
        }
        
        self.end_time = Some(Utc::now());
        self.print_results();
    }

    fn print_results(&self) {
        println!("\n=== Quiz Results ===");
        
        let correct_count = self.responses.iter().filter(|r| r.is_correct).count();
        let total_questions = self.responses.len();
        let score_percentage = if total_questions > 0 {
            (correct_count as f32 / total_questions as f32) * 100.0
        } else {
            0.0
        };
        
        println!("Score: {}/{} ({:.1}%)", correct_count, total_questions, score_percentage);
        
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            let total_time = (end - start).num_seconds();
            println!("Total time: {} seconds", total_time);
        }
        
        let avg_response_time = if !self.responses.is_empty() {
            self.responses.iter().map(|r| r.response_time).sum::<u32>() / self.responses.len() as u32
        } else {
            0
        };
        
        println!("Average response time: {} seconds", avg_response_time);
        
        // Performance by difficulty
        let mut difficulty_stats = HashMap::new();
        for response in &self.responses {
            if let Some(question) = self.questions.iter().find(|q| q.id == response.question_id) {
                let stats = difficulty_stats.entry(question.difficulty.clone()).or_insert((0, 0));
                stats.1 += 1; // total
                if response.is_correct {
                    stats.0 += 1; // correct
                }
            }
        }
        
        println!("\nPerformance by difficulty:");
        for (difficulty, (correct, total)) in difficulty_stats {
            let percentage = if total > 0 { (correct as f32 / total as f32) * 100.0 } else { 0.0 };
            println!("  {:?}: {}/{} ({:.1}%)", difficulty, correct, total, percentage);
        }
    }
    
    pub fn get_score(&self) -> f32 {
        let correct_count = self.responses.iter().filter(|r| r.is_correct).count();
        let total_questions = self.responses.len();
        if total_questions > 0 {
            (correct_count as f32 / total_questions as f32) * 100.0
        } else {
            0.0
        }
    }
}

impl Default for Quiz {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_evaluation() {
        let question = MultipleChoiceQuestion::new(
            "q1".to_string(),
            "What is the capital of France?".to_string(),
            vec!["Berlin".to_string(), "Paris".to_string(), "Madrid".to_string()],
            1,
            QuestionDifficulty::Easy,
            "Geography".to_string(),
        );

        assert!(question.evaluate(1));
        assert!(!question.evaluate(0));
        assert_eq!(question.get_correct_answer_text(), "Paris");
    }
    
    #[test]
    fn test_quiz_creation() {
        let mut quiz = Quiz::new();
        let question = MultipleChoiceQuestion::new(
            "q1".to_string(),
            "What is 2 + 2?".to_string(),
            vec!["3".to_string(), "4".to_string(), "5".to_string()],
            1,
            QuestionDifficulty::Easy,
            "Math".to_string(),
        );
        
        quiz.add_question(question);
        assert_eq!(quiz.questions.len(), 1);
    }
    
    #[test]
    fn test_question_with_explanation() {
        let question = MultipleChoiceQuestion::new(
            "q1".to_string(),
            "What is the largest planet?".to_string(),
            vec!["Earth".to_string(), "Jupiter".to_string(), "Saturn".to_string()],
            1,
            QuestionDifficulty::Medium,
            "Science".to_string(),
        ).with_explanation("Jupiter is the largest planet in our solar system.".to_string());
        
        assert!(question.explanation.is_some());
    }
}
