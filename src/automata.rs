use std::collections::HashSet;

#[derive(Debug)]
pub struct Automata {
    states: HashSet<String>,
    transitions: HashSet<(String, String, String)>,
    initial_states: HashSet<String>,
    terminal_states: HashSet<String>,
}

impl Automata {
    pub fn new() -> Self {
        Automata {
            states: HashSet::new(),
            transitions: HashSet::new(),
            initial_states: HashSet::new(),
            terminal_states: HashSet::new(),
        }
    }

    pub fn add_state(&mut self, state: String, is_initial: bool, is_terminal: bool) {
        let _ = &self.states.insert(state.clone());

        if is_initial {
            let _ = &self.initial_states.insert(state.clone());
        }

        if is_terminal {
            let _ = &self.terminal_states.insert(state.clone());
        }
    }

    pub fn get_initial_states(&self) -> &HashSet<String> {
        &self.initial_states
    }

    pub fn get_terminal_states(&self) -> &HashSet<String> {
        &self.initial_states
    }

    pub fn add_transition(&mut self, source_state: String, letter: String, dest_state: String) {
        let _ = &self.transitions.insert((source_state, letter, dest_state));
    }

    pub fn get_transition(&self, source_state: String, letter: String) -> Option<String> {
        for transition in &self.transitions {
            if transition.0 == source_state && transition.1 == letter {
                let dest_state = transition.2.to_owned();
                return Some(dest_state);
            }
        }
        return None;
    }

    pub fn accepts(&self, word: String) -> Result<bool, bool> {
        // Start from the initial states
        let mut successor_states = self.get_initial_states().to_owned();

        // Iterate over each letter of the word
        for letter in word.chars() {
            let mut new_successor_states = HashSet::new();
            // For each state that we have to check, do the transition, then store the result in new_successors_states
            for state in successor_states {
                let succ_state = &self
                    .get_transition(state.to_owned(), letter.into())
                    .ok_or(false)?;
                new_successor_states.insert(succ_state.to_owned());
            }
            // Update successor_states then start the loop again
            successor_states = new_successor_states;
        }

        // Now that we've iterated over the word,
        // we accept it if and only if at least one of
        // the resulting states is a terminal state
        for state in successor_states {
            if self.terminal_states.contains(&state) {
                return Ok(true);
            }
        }
        
        // We didn't find any
        return Err(false);
    }
}
