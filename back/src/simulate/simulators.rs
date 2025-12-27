use crate::simulate::{Circuit, Simulation, gates};

pub struct Ctx<'a> {
	pub sim: &'a Simulation,
	pub circuit: &'a mut Circuit,
	pub comp_data: u64,
	pub inputs: &'a [usize],
	pub outputs: &'a [usize],
	pub add_data: &'a [u64],
	pub reg_offset: usize,
}
impl Ctx<'_> {
	pub fn prelude(&mut self) -> (u64, &mut [u64], &[usize], &[usize]) {
		(self.comp_data, &mut self.circuit.wires, self.inputs, self.outputs)
	}
}
pub type CompSimulator<'a> = fn(Ctx<'a>);

#[inline(always)]
pub fn dispatch_simulator(typeid: u16, ctx: Ctx) {
	match typeid {
		0 => gates::not(ctx),
		1 => gates::and(ctx),
		2 => gates::or(ctx),
		3 => gates::xor(ctx),
		4 => gates::nand(ctx),
		5 => gates::nor(ctx),
		6 => gates::imply(ctx),
		7 => gates::xnor(ctx),
		8 => gates::bclear(ctx),
		_ => unreachable!(),
	}
}
