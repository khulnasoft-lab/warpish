use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

/// A simple fuzzy finder.
pub struct FuzzyFinder {
    matcher: SkimMatcherV2,
}

impl Default for FuzzyFinder {
    fn default() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }
}

impl FuzzyFinder {
    /// Finds and ranks matches from a list of choices based on a query.
    /// Returns a sorted list of the original items that matched.
    pub fn find<'a, T: AsRef<str>>(&self, query: &str, choices: &'a [T]) -> Vec<&'a T> {
        if query.is_empty() {
            return choices.iter().collect();
        }

        let mut matches: Vec<(i64, &'a T)> = choices
            .iter()
            .filter_map(|choice| {
                self.matcher.fuzzy_match(choice.as_ref(), query)
                    .map(|score| (score, choice))
            })
            .collect();
        
        // Sort by score, highest first
        matches.sort_by(|a, b| b.0.cmp(&a.0));
        
        matches.into_iter().map(|(_, choice)| choice).collect()
    }
}