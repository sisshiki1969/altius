use altius_core::{model::Model, op::infer_shapes, tensor::Tensor};
use rustc_hash::FxHashMap;
use thread_local::ThreadLocal;

use crate::{create_execution_plan, SessionError};

#[cfg(feature = "cuda")]
use super::session::SafeCudnnContext;
use super::{session::InterpreterSession, thread::ThreadCtx};
#[cfg(feature = "cuda")]
use cudnn::CudnnContext;

pub struct InterpreterSessionBuilder {
    model: Model,
    intra_op_num_threads: usize,
    enable_profiling: bool,
}

impl<'a> InterpreterSessionBuilder {
    pub const fn new(model: Model) -> Self {
        Self {
            model,
            intra_op_num_threads: 1,
            enable_profiling: false,
        }
    }

    pub const fn with_intra_op_num_threads(mut self, intra_op_num_threads: usize) -> Self {
        self.intra_op_num_threads = intra_op_num_threads;
        self
    }

    pub const fn with_profiling_enabled(mut self, enable_profiling: bool) -> Self {
        self.enable_profiling = enable_profiling;
        self
    }

    pub fn build(self) -> Result<InterpreterSession, SessionError> {
        let model = self.model;
        let enable_profiling = self.enable_profiling;
        let intra_op_num_threads = self.intra_op_num_threads;

        let sorted_nodes = model.topo_sort_nodes();
        let mut inferred_shapes = FxHashMap::default();
        infer_shapes(&model, &sorted_nodes, &mut inferred_shapes)?;

        #[cfg(feature = "blis")]
        {
            extern "C" {
                fn bli_thread_set_num_threads(n_threads: usize);
            }
            unsafe { bli_thread_set_num_threads(intra_op_num_threads) };
        }

        Ok(InterpreterSession {
            #[cfg(feature = "cuda")]
            cudnn_ctx: SafeCudnnContext(CudnnContext::new().expect("cudnn context init failed")),
            execution_plans: create_execution_plan(&model, &sorted_nodes),
            model,
            inferred_shapes,
            enable_profiling,
            values: ThreadLocal::new(),
            dummy_value: Tensor::zeros::<f32>(vec![0].into()),
            tctx: ThreadCtx::new_with_num_threads(intra_op_num_threads),
        })
    }
}
