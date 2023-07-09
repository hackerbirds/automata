use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

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

    pub fn get_transition(&self, source_state: String, letter: String) -> Option<Vec<String>> {
        let mut all_transitions: Vec<String> = Vec::new();
        for transition in &self.transitions {
            if transition.0 == source_state && transition.1 == letter {
                all_transitions.push(transition.2.to_owned());
            }
        }
        if all_transitions.is_empty() {
            return None;
        } else {
            return Some(all_transitions);
        }
    }

    pub fn accepts(&self, word: String) -> Result<bool, bool> {
        // Start from the initial states
        let mut successor_states = self.get_initial_states().to_owned();

        // Iterate over each letter of the word
        for letter in word.chars() {
            let mut new_successor_states = HashSet::new();
            // For each state that we have to check, do the transition, then store the result in new_successors_states
            for state in successor_states {
                let succ_states: &Option<Vec<String>> =
                    &self.get_transition(state.to_owned(), letter.into());
                if let Some(succ_vec) = succ_states {
                    for succ in succ_vec {
                        new_successor_states.insert(succ.to_owned());
                    }
                }
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

    // Because we weren't storing the automata's alphabet
    // like we should've done, we retrieve the possible letters
    // our automata can process by scanning all the letters in
    // the transitions that we defined.
    pub fn alphabet(&self) -> Vec<String> {
        let mut alphabet = Vec::new();
        for transition in &self.transitions {
            if !alphabet.contains(&transition.1) {
                alphabet.push(transition.1.to_owned());
            }
        }
        return alphabet;
    }

    // Given a set of states, find the successors of this set for a given letter.
    // All it really does it just iterate over each state in the set, make the transition,
    // and store the result (= successor state) in a new set which we return at the end
    pub fn successors(&self, set: &Vec<String>, letter: String) -> Vec<String> {
        let mut succs_set = Vec::new();
        for state in set {
            if let Some(succs) = self.get_transition(state.to_owned(), letter.clone()) {
                for succ in succs {
                    if !succs_set.contains(&succ) {
                        succs_set.push(succ);
                    }
                }
            }
        }
        return succs_set;
    }

    pub fn determinize(&self) -> Self {
        // * STEP 1 *

        // We will add the determinised states and transition as we go along,
        // and return this automata at the end.
        let mut determinised_automata = Automata::new();

        // Our stack. It stores sets of states, so a Vec<String>.
        let mut stack: Vec<Vec<String>> = Vec::new();
        let mut processed_sets: Vec<Vec<String>> = Vec::new();

        // Our HashMap for sets of states <-> merged state.
        let mut hashmap: HashMap<Vec<String>, String> = HashMap::new();

        // * STEP 2 *

        let initial_states_set: HashSet<String> = self.initial_states.clone();

        // Stack takes Vec<>, so we have to uglily convert our HashSet into Vec<>.
        stack.push(initial_states_set.clone().into_iter().collect());

        // If one of the initial states is also a terminal state, then the merged initial
        // states should also be terminal
        let is_terminal: bool = !&self.initial_states.is_disjoint(&self.terminal_states);

        // We merge the initial states into one. Don't forget to add the relation in the hashmap,
        // and update determinised_automata of course.
        let merged_initial_states: String = format!("{:?}", &initial_states_set);

        hashmap.insert(
            initial_states_set.into_iter().collect(),
            merged_initial_states.clone(),
        );
        determinised_automata.add_state(merged_initial_states, true, is_terminal);

        // * STEP 3 *
        let alphabet = self.alphabet();
        while !&stack.is_empty() {
            let state_set = stack.pop().unwrap();
            for letter in &alphabet {
                // Find all the successors of our set given `letter`
                let successors_set: Vec<String> = self.successors(&state_set, letter.clone());
                if !processed_sets.contains(&state_set) && !successors_set.is_empty() {
                    let merged_succ = match hashmap.get(&successors_set) {
                        Some(state) => state.to_owned(),
                        None => {
                            let merged_succs_state = format!("{:?}", &successors_set);
                            hashmap.insert(
                                successors_set.clone().into_iter().collect(),
                                merged_succs_state.clone(),
                            );
                            merged_succs_state
                        }
                    };

                    // Add transition from source merged state to new merged state
                    let source_merged_state = hashmap.get(&state_set).unwrap();

                    determinised_automata.add_transition(
                        source_merged_state.to_owned(),
                        letter.clone(),
                        merged_succ.clone(),
                    );

                    let mut is_merged_state_terminal = false;
                    for succ_state in &successors_set {
                        if self.terminal_states.contains(succ_state) {
                            is_merged_state_terminal = true;
                        }
                    }

                    determinised_automata.add_state(merged_succ, false, is_merged_state_terminal);
                    // * STEP 4 *
                    stack.push(successors_set);
                }
            }
            processed_sets.push(state_set);
            // * STEP 5 * : Stack empty?
        }
        // * STEP 6 * : We're done.
        return determinised_automata;
    }
}
