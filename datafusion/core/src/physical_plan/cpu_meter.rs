use std::usize;

use super::{accept, ExecutionPlan, ExecutionPlanVisitor};

#[allow(missing_docs)]
pub struct CpuMeterPlan<'a> {
    inner: &'a dyn ExecutionPlan,
}

impl<'a> CpuMeterPlan<'a> {
    #[allow(missing_docs)]
    pub fn new(inner: &'a dyn ExecutionPlan) -> Self {
        Self { inner }
    }

    #[allow(missing_docs)]
    pub fn cpu_time(&self) -> usize {
        let mut vistor = MeterVisitor::new();

        accept(self.inner, &mut vistor).unwrap();

        vistor.cpu_time
    }
}

struct MeterVisitor {
    cpu_time: usize,
}

impl MeterVisitor {
    pub fn new() -> Self {
        Self { cpu_time: 0 }
    }
}

impl ExecutionPlanVisitor for MeterVisitor {
    type Error = String;

    fn pre_visit(&mut self, plan: &dyn ExecutionPlan) -> Result<bool, Self::Error> {
        let metrics = plan.metrics();
        if let Some(m) = metrics {
            if let Some(cpu_time) = m.elapsed_compute() {
                self.cpu_time += cpu_time / 1000;
            }
        }
        Ok(true)
    }
}
