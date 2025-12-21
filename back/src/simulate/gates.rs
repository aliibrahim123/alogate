use crate::simulate::{LogicCircuit, LogicComp, SimContext};

pub fn and(ctx: &SimContext, circuit: &mut LogicCircuit, comp: &LogicComp) {
	let LogicCircuit { wires, .. } = circuit;
	let LogicComp { inputs, outputs, .. } = comp;
	assert!(inputs.len() < 9 && inputs.len() > 1);
	wires[outputs[0]] = comp.inputs.iter().map(|i| wires[*i]).reduce(|a, b| a & b).unwrap();
}
