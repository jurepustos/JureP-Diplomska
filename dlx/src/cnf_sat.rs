use libdlx::dlxc;

pub enum Var {
    Positive(usize),
    Negated(usize)
}

enum Item {
    ClauseVar(usize, usize),
    Variable(usize)
} 

pub fn cnf_sat_dlx(or_clauses: &[Vec<Var>]) {
    
}