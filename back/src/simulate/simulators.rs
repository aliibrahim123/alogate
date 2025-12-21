use crate::simulate::{LogicCircuit, LogicComp, SimContext, gates::and};

pub static COMPONENTS_SIMULATERS: &[fn(&SimContext, &mut LogicCircuit, &LogicComp)] = &[and];
