use closestmatch::ClosestMatch;

/// String utillity for finding closest matches
#[derive(Clone)]            // just in case
pub struct StringMatcher {
    pub words : Vec<String>,
    pub matcher : ClosestMatch
}

impl StringMatcher {
    /// Creates a new StringMatcher with the given words
    // words is the word that we are searching for
    // constructor function
    pub fn new(words : Vec<String>) -> Self {
        // create bag of words based on minimum length
        // uses closure to determine the smallest val within a single line
        let min_length = words.iter().map(|w| w.len()).min().unwrap_or(0)       // default error handler val : 0

        // another method that doesn't require iterating using for loop
        let bag_sizes : Vec<usize> = (1..=min_length).collect();

        // form a new instance of string matcher struct
        StringMatcher {
            matcher : ClosestMatch::new(words.clone(), bag_sizes),
            words 
        }        
    }

    /// Find the cloest matching string
    // immutable method, simply searches and returns a string
    pub fn find_closest(&self, input : &str) -> Option<String> {
        self.matcher.get_closest(input.to_string())
    }
}

