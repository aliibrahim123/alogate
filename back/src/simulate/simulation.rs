use std::{
	cell::RefCell,
	sync::Arc,
	thread::sleep,
	time::{Duration, Instant},
};

use crate::simulate::{
	Info, SimCommand,
	sim_info::Status,
	simulators::{self, dispatch_simulator},
};

#[derive(Debug)]
pub struct Comp {
	pub type_id: u16,
	pub data: u64,
	pub inputs: u8,
	pub outputs: u8,
	pub regs: u16,
	pub add_data: u16,
}
#[derive(Debug)]
pub struct CircuitSchema {
	pub comps: Box<[Comp]>,
	pub connections: Box<[usize]>,
	pub wire_count: u16,
	pub reg_count: u16,
	pub comp_data: Box<[u64]>,
}
#[derive(Debug)]
pub struct Circuit {
	pub id: u32,
	pub schema_id: u32,
	pub wires: Box<[u64]>,
	pub regs: Box<[u64]>,
}

#[derive(Debug, Default)]
pub struct Simulation {
	pub info: Arc<Info>,
	pub status: Status,
	pub clock: u64,

	target_clock: u64,
	batch_size: u64,
	reached_golden_batch: bool,

	pub schemas: Vec<Arc<CircuitSchema>>,
	pub circuit_instances: Vec<RefCell<Circuit>>,
}

impl Simulation {
	pub fn new(info: Arc<Info>) -> Simulation {
		Simulation { batch_size: 1, info, ..Default::default() }
	}
	fn set_status(&mut self, status: Status) {
		self.status = status;
		self.info.general.write().unwrap().status = status;
	}
	const BIG_TICK_TIME: u64 = 20;
	pub fn thrive(&mut self) {
		let sleep = || sleep(Duration::from_millis(Self::BIG_TICK_TIME));
		loop {
			let command = *self.info.command();
			*self.info.command.write().unwrap() = SimCommand::None;
			let status = self.status;
			match command {
				SimCommand::None => match status {
					Status::Running => self.step_btick(),
					_ => sleep(),
				},
				SimCommand::Run { target_clock } => {
					self.target_clock = target_clock;
					match status {
						Status::Stopped => self.start_simulate(target_clock),
						_ => self.step_btick(),
					}
				}
				SimCommand::Pause => {
					self.set_status(Status::Paused);
					sleep();
				}
				SimCommand::End => {
					self.end_simulate();
					sleep();
				}
				SimCommand::Kill => {
					if status == Status::Running {
						self.end_simulate();
						sleep()
					}
					return;
				}
			}
		}
	}
	fn start_simulate(&mut self, target_clock: u64) {
		self.target_clock = target_clock;
		self.set_status(Status::Running);
		self.step_btick();
	}
	fn end_simulate(&mut self) {
		self.set_status(Status::Stopped);
	}
	fn step_btick(&mut self) {
		self.set_status(Status::Running);
		const BT_TIME: u64 = Simulation::BIG_TICK_TIME;
		let mut ticks_stepped = 0u64;
		let start_time = Instant::now();
		if !self.reached_golden_batch {
			loop {
				let batch_start = Instant::now();
				let reached_end = self.step_batch(&mut ticks_stepped);
				if reached_end || start_time.elapsed().as_millis() >= BT_TIME as _ {
					return;
				}
				self.batch_size *=
					match batch_start.elapsed().as_micros() / (BT_TIME * 1000) as u128 {
						500.. => {
							self.reached_golden_batch = true;
							break;
						}
						250..500 => 2,
						125..250 => 4,
						62..125 => 8,
						_ => 16,
					};
			}
		}

		loop {
			let reached_end = self.step_batch(&mut ticks_stepped);
			if reached_end || start_time.elapsed().as_millis() >= BT_TIME as _ {
				return;
			}
		}
	}
	fn step_batch(&mut self, ticks_stepped: &mut u64) -> bool {
		let ticks_per_btick = self.info.config().ticks_per_btick;
		for _ in 0..self.batch_size {
			self.simulate(0);
			self.clock += 1;
			*ticks_stepped += 1;

			if self.clock >= self.target_clock {
				self.end_simulate();
				return true;
			}
			if *ticks_stepped >= ticks_per_btick {
				return true;
			}
		}
		false
	}
	pub fn simulate(&self, circuit_id: usize) {
		let circuit = &mut self.circuit_instances[circuit_id].borrow_mut();
		let schema = &self.schemas[circuit.schema_id as usize];

		let (mut data_ind, mut reg_ind, mut io_ind) = (0usize, 0usize, 0usize);

		for comp in &schema.comps {
			#[rustfmt::skip]
			dispatch_simulator(comp.type_id,
			simulators::Ctx {
				sim: self, circuit, comp_data: comp.data, reg_offset: reg_ind,
				inputs: &schema.connections[io_ind..comp.inputs as usize + io_ind],
				outputs: &schema.connections
					[io_ind..(comp.outputs + comp.inputs) as usize + io_ind],
				add_data: &schema.comp_data[data_ind..comp.add_data as usize + data_ind],
			});

			data_ind += comp.add_data as usize;
			reg_ind += comp.regs as usize;
			io_ind += comp.inputs as usize + comp.outputs as usize;
		}
	}
}
