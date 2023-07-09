pub mod automata;

use crate::automata::Automata;

pub fn main() {
    let mut automata = Automata::new();
    automata.add_state("0".into(), true, false);
    automata.add_state("B".into(), false, false);
    automata.add_state("C".into(), false, false);
    automata.add_state("1".into(), false, true);

    automata.add_transition("0".into(), "a".into(), "B".into());
    automata.add_transition("0".into(), "a".into(), "C".into());

    automata.add_transition("B".into(), "b".into(), "B".into());
    automata.add_transition("C".into(), "c".into(), "C".into());

    automata.add_transition("B".into(), "a".into(), "1".into());
    automata.add_transition("C".into(), "a".into(), "1".into());

    println!("{:?}", &automata.determinize());
}
