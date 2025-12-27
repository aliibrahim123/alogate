use crate::simulate::{Circuit, Comp, Simulation, simulators::Ctx};

#[inline(always)]
pub fn not(mut ctx: Ctx) {
	let (_, wires, inputs, outputs) = ctx.prelude();
	wires[outputs[0]] = !wires[inputs[0]];
}

#[inline(always)]
fn logic_gate(mut ctx: Ctx, op: impl Fn(u64, u64) -> u64) {
	let (_, wires, inputs, outputs) = ctx.prelude();
	assert!(inputs.len() <= 8 && inputs.len() >= 2);
	let mut res = wires[inputs[0]];
	for i in &inputs[1..] {
		res = op(res, wires[*i]);
	}
	wires[outputs[0]] = res;
}

#[inline(always)]
pub fn and(ctx: Ctx) {
	logic_gate(ctx, |a, b| a & b);
}
#[inline(always)]
pub fn or(ctx: Ctx) {
	logic_gate(ctx, |a, b| a | b);
}
#[inline(always)]
pub fn xor(ctx: Ctx) {
	logic_gate(ctx, |a, b| a ^ b);
}
#[inline(always)]
pub fn nand(ctx: Ctx) {
	logic_gate(ctx, |a, b| !(a & b));
}
#[inline(always)]
pub fn nor(ctx: Ctx) {
	logic_gate(ctx, |a, b| !(a | b));
}
#[inline(always)]
pub fn xnor(ctx: Ctx) {
	logic_gate(ctx, |a, b| !(a ^ b));
}
#[inline(always)]
pub fn imply(ctx: Ctx) {
	logic_gate(ctx, |a, b| !a | b);
}
#[inline(always)]
pub fn bclear(ctx: Ctx) {
	logic_gate(ctx, |a, b| a & !b);
}
