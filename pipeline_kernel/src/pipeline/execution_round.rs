use std::collections::VecDeque;
use crate::PipelineExecutionTask;

pub struct PipelineExecutionTaskRound {
	tasks: VecDeque<PipelineExecutionTask>,
}

impl PipelineExecutionTaskRound {
	pub fn new() -> Self {
		Self {
			tasks: VecDeque::new(),
		}
	}

	pub fn add_task(&mut self, task: PipelineExecutionTask) {
		self.tasks.push_back(task);
	}

	pub fn add_tasks(&mut self, tasks: Vec<PipelineExecutionTask>) {
		self.tasks.extend(tasks);
	}

	pub fn has_task(&self) -> bool {
		!self.tasks.is_empty()
	}

	pub fn take_task(&mut self) -> Option<PipelineExecutionTask> {
		self.tasks.pop_front()
	}
}
