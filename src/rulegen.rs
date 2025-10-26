


struct Transition<State, StateGroup, const N : usize>
{
    neighbourhood: [StateGroup; N],
    output: State,
}


impl<State,StateGroup, const N : usize> Transition<State,StateGroup, N> {
    
    fn create_line(&self, outp)
}