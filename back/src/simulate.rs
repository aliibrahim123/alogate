use std::{
	collections::VecDeque,
	sync::{Arc, RwLock, RwLockReadGuard},
	thread, u64,
};

mod gates;
mod simulation;
mod simulators;

pub(self) use {
	sim_info::Info, simulation::Circuit, simulation::CircuitSchema, simulation::Comp,
	simulation::Simulation,
};

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum SimCommand {
	#[default]
	None,
	Run {
		target_clock: u64,
	},
	Pause,
	End,
	Kill,
}

mod sim_info {
	use super::*;

	#[derive(Debug, Default, PartialEq, Clone, Copy)]
	pub enum Status {
		#[default]
		Stopped,
		Running,
		Paused,
	}
	#[derive(Debug, Default)]
	pub struct General {
		pub clock: u64,
		pub status: Status,
	}
	#[derive(Debug, Default)]
	pub struct Config {
		pub ticks_per_btick: u64,
	}
	#[derive(Debug, Default)]
	pub struct Info {
		pub general: RwLock<General>,
		pub command: RwLock<SimCommand>,
		pub config: RwLock<Config>,
	}
	impl Info {
		pub fn general(&self) -> RwLockReadGuard<'_, General> {
			self.general.read().unwrap()
		}
		pub fn command(&self) -> RwLockReadGuard<'_, SimCommand> {
			self.command.read().unwrap()
		}
		pub fn config(&self) -> RwLockReadGuard<'_, Config> {
			self.config.read().unwrap()
		}
	}
}

#[derive(Debug, Default, Clone)]
pub struct Simulator {
	info: Arc<Info>,
}

impl Simulator {
	pub fn new() -> Simulator {
		let sim = Simulator::default();
		let info = sim.info.clone();
		thread::spawn(move || {
			let mut ctx = Simulation::new(info);
			ctx.thrive();
		});
		sim
	}

	pub fn command(&self, command: SimCommand) {
		*self.info.command.write().unwrap() = command;
	}
	pub fn run_endless(&self) {
		self.command(SimCommand::Run { target_clock: u64::MAX });
	}
	pub fn run_for(&self, target_clock: u64) {
		self.command(SimCommand::Run { target_clock });
	}
	pub fn pause(&self) {
		self.command(SimCommand::Pause);
	}
	pub fn end(&self) {
		self.command(SimCommand::End);
	}
}
