mod gates;
mod simulators;

use std::{cell::RefCell, collections::HashMap, sync::RwLock, time::Instant};

use crate::simulate::simulators::COMPONENTS_SIMULATERS;

#[derive(Debug)]
pub struct LogicComp {
	pub type_id: u16,
	pub data: u64,
	pub inputs: Vec<usize>,
	pub outputs: Vec<usize>,
}
#[derive(Debug)]
pub struct LogicCircuitSchema {
	pub comps: Vec<LogicComp>,
	pub wire_count: u16,
	pub reg_count: u16,
	pub comp_data: Vec<u64>,
}
#[derive(Debug)]
pub struct LogicCircuit {
	pub id: u32,
	pub schema_id: u32,
	pub wires: Box<[u64]>,
	pub regs: Box<[u64]>,
}

#[derive(Debug, Default, PartialEq)]
pub enum SimRequest {
	#[default]
	None,
	Pause,
	Stop,
}

#[derive(Debug, Default)]
pub struct SimContext<'s> {
	pub clock: u64,
	pub schemas: Vec<&'s LogicCircuitSchema>,
	pub circuit_instances: Vec<LogicCircuit>,
}

impl SimContext<'_> {
	pub fn new() -> SimContext<'static> {
		SimContext { ..Default::default() }
	}
	async fn simulate(&mut self, target_clock: u64, stop_listener: &fn() -> bool) {
		let mut batch_size = 1u64;
		loop {
			let start_time = Instant::now();
			let reached_end = self.simulate_batch(batch_size, target_clock);
			if reached_end || stop_listener() {
				break;
			}
			batch_size *= match start_time.elapsed().as_millis() {
				10.. => break,
				5..10 => 2,
				2..5 => 4,
				1..2 => 8,
				_ => 16,
			};
		}

		loop {
			let reached_end = self.simulate_batch(batch_size, target_clock);
			if reached_end || stop_listener() {
				break;
			}
		}
	}
	fn simulate_batch(&mut self, batch_size: u64, target_clock: u64) -> bool {
		for _ in 0..batch_size {
			simulate(self, 0);
			self.clock += 1;

			if self.clock >= target_clock {
				return true;
			}
		}
		false
	}
	pub fn step(&mut self, clock_steps: u64, stop_listener: &fn() -> bool) {
		self.simulate(self.clock + clock_steps, stop_listener);
	}
	pub fn start(&mut self, target_clock: Option<u64>, stop_listener: &fn() -> bool) {
		self.simulate(target_clock.unwrap_or(0), stop_listener);
	}

	pub(self) fn get_circuit(&self, circuit_id: usize) -> &mut LogicCircuit {
		todo!();
		//unsafe { &mut *(&mut *(&self.circuit_instances as *const _))[circuit_id] }
	}
}

fn simulate(ctx: &SimContext, circuit_id: usize) {
	todo!();
	/*let circuit = &mut ctx.circuit_instances[circuit_id];
	let schema = ctx.schemas[circuit.schema_id as usize];

	for comp in &schema.comps {
		COMPONENTS_SIMULATERS[comp.type_id as usize](ctx, circuit, comp);
	}*/
}
