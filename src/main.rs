pub mod automata;

use crate::automata::Automata;

pub fn main() {
    let mut automata = Automata::new();
    automata.add_state("0".into(), true, false);
    automata.add_state("1".into(), false, true);
    automata.add_transition("0".into(), "a".into(), "0".into());
    automata.add_transition("0".into(), "b".into(), "1".into());

    println!("{}", automata.accepts("aaa".to_string()).unwrap_or(false));
    println!("{}", automata.accepts("aaaab".to_string()).unwrap_or(false));
    println!("{}", automata.accepts("abbaaa".to_string()).unwrap_or(false));
    println!("{}", automata.accepts("ba".to_string()).unwrap_or(false));
    println!("{}", automata.accepts("aaaaaaaaaab".to_string()).unwrap_or(false));
}
