use crate::{dim::Dimensions, tensor::Tensor, value::ValueId};
use id_arena::{Arena, Id};

pub type NodeId = Id<Node>;
pub type NodeArena = Arena<Node>;

#[derive(Debug, Clone)]
pub struct Node {
    pub op: Op,
    pub inputs: Vec<ValueId>,
    pub outputs: Vec<ValueId>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Conv2d(Conv2d),
    Add,
    Mul,
    ReLU,
    MaxPool(MaxPool),
    GlobalAveragePool,
    Reshape,
    Flatten(Flatten),
    MatMul,
    Gemm(Gemm),
    HardSigmoid(HardSigmoid),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Conv2d {
    pub auto_pad: String,
    pub group: i64,
    pub kernel_shape: Dimensions,
    pub strides: Dimensions,
    pub padding: Dimensions,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MaxPool {
    pub kernel_shape: Dimensions,
    pub strides: Dimensions,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HardSigmoid {
    pub alpha: f32,
    pub beta: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Flatten {
    pub axis: i64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gemm {
    pub alpha: f32,
    pub beta: f32,
    pub trans_a: bool,
    pub trans_b: bool,
}

impl Node {
    pub const CONV2D_IN: usize = 0;
    pub const CONV2D_WEIGHT: usize = 1;
    pub const CONV2D_OUT: usize = 0;

    pub const ADD_IN_A: usize = 0;
    pub const ADD_IN_B: usize = 1;
    pub const ADD_OUT: usize = 0;

    pub const MUL_IN_A: usize = 0;
    pub const MUL_IN_B: usize = 1;
    pub const MUL_OUT: usize = 0;

    pub const RELU_IN: usize = 0;
    pub const RELU_OUT: usize = 0;

    pub const MAXPOOL_IN: usize = 0;
    pub const MAXPOOL_OUT: usize = 0;

    pub const GLOBALAVERAGEPOOL_IN: usize = 0;
    pub const GLOBALAVERAGEPOOL_OUT: usize = 0;

    pub const RESHAPE_IN: usize = 0;
    pub const RESHAPE_SHAPE: usize = 1;
    pub const RESHAPE_OUT: usize = 0;

    pub const FLATTEN_IN: usize = 0;
    pub const FLATTEN_OUT: usize = 0;

    pub const MATMUL_IN_A: usize = 0;
    pub const MATMUL_IN_B: usize = 1;
    pub const MATMUL_OUT: usize = 0;

    pub const GEMM_IN_A: usize = 0;
    pub const GEMM_IN_B: usize = 1;
    pub const GEMM_OUT: usize = 0;

    pub const HARDSIGMOID_IN: usize = 0;
    pub const HARDSIGMOID_OUT: usize = 0;

    pub fn new(op: Op) -> Self {
        Self {
            op,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn with_in(mut self, id: ValueId) -> Self {
        self.inputs.push(id);
        self
    }

    pub fn with_ins(mut self, mut ids: Vec<ValueId>) -> Self {
        self.inputs.append(&mut ids);
        self
    }

    pub fn with_out(mut self, id: ValueId) -> Self {
        self.outputs.push(id);
        self
    }

    pub fn with_outs(mut self, mut ids: Vec<ValueId>) -> Self {
        self.outputs.append(&mut ids);
        self
    }

    pub fn alloc(self, arena: &mut NodeArena) -> NodeId {
        let id = arena.alloc(self);
        id
    }
}

/// Computes the output shape for a node given `inputs`.
/// `attrs` could be overwritten. (e.g. paddings given auto_pad)
pub fn compute_output_shapes(op: &mut Op, inputs: &[Tensor]) -> Vec<Dimensions> {
    let mut shapes = vec![];
    match op {
        Op::Conv2d(conv) => {
            let auto_pad = &conv.auto_pad;
            let kernel = &conv.kernel_shape;
            let stride = &conv.strides;
            let padding = &conv.padding;
            let kernel = kernel.as_slice();
            let stride = stride.as_slice();
            let mut padding = padding.as_slice();
            let input = inputs[Node::CONV2D_IN].dims().as_slice();
            let weight = inputs[Node::CONV2D_WEIGHT].dims().as_slice();

            if !auto_pad.is_empty() && auto_pad != "NOTSET" {
                let out0 = (input[2] as f32 / stride[0] as f32).ceil() as usize;
                let out1 = (input[3] as f32 / stride[1] as f32).ceil() as usize;
                let pad0 =
                    ((out0 - 1) * stride[0] + ((kernel[0] - 1) * 1 + 1)).saturating_sub(input[2]);
                let pad1 =
                    ((out1 - 1) * stride[1] + ((kernel[1] - 1) * 1 + 1)).saturating_sub(input[3]);
                assert!(auto_pad == "SAME_UPPER");
                let new_padding = vec![pad0 / 2, pad1 / 2, pad0 - pad0 / 2, pad1 - pad1 / 2];
                conv.padding = new_padding.into();
                padding = conv.padding.as_slice();
            }

            let h_in = input[2];
            let w_in = input[3];
            let output_shape = vec![
                input[0],
                weight[0],
                (h_in + 2 * padding[0] - 1 * (kernel[0] - 1) - 1) / stride[0] + 1,
                (w_in + 2 * padding[1] - 1 * (kernel[1] - 1) - 1) / stride[1] + 1,
            ];
            shapes.push(output_shape.into());
        }
        Op::Add => {
            let in_a = inputs[Node::ADD_IN_A].dims();
            let in_b = inputs[Node::ADD_IN_B].dims();
            assert!(
                in_a == in_b || {
                    in_a.len() == 4
                        && in_b.len() == 3
                        && in_a.as_slice()[1] == in_b.as_slice()[0]
                        && in_b.as_slice()[1] == 1
                        && in_b.as_slice()[2] == 1
                }
            ); // TODO: Support broadcasting.
            shapes.push(in_a.clone());
        }
        Op::Mul => {
            let in_a = inputs[Node::MUL_IN_A].dims();
            let in_b = inputs[Node::MUL_IN_B].dims();
            println!("{:?} {:?}", in_a, in_b);
            assert!(
                in_a == in_b || {
                    in_a.len() == 4
                        && in_b.len() == 4
                        && in_a.as_slice()[0] == in_b.as_slice()[0]
                        && in_a.as_slice()[1] == in_b.as_slice()[1]
                        && in_a.as_slice()[2] == 1
                        && in_a.as_slice()[3] == 1
                }
            ); // TODO: Support broadcasting.
            shapes.push(in_b.clone());
        }
        Op::MaxPool(maxpool) => {
            let kernel = &maxpool.kernel_shape;
            let stride = &maxpool.strides;
            let input = &inputs[Node::MAXPOOL_IN].dims();

            let h_in = input.as_slice()[2];
            let w_in = input.as_slice()[3];
            let output_shape = vec![
                input.as_slice()[0],
                input.as_slice()[1],
                (h_in + 2 * 0 - 1 * (kernel.as_slice()[0] - 1) - 1) / stride.as_slice()[0] + 1,
                (w_in + 2 * 0 - 1 * (kernel.as_slice()[1] - 1) - 1) / stride.as_slice()[1] + 1,
            ];
            shapes.push(output_shape.into());
        }
        Op::GlobalAveragePool => {
            let input = &inputs[Node::GLOBALAVERAGEPOOL_IN].dims().as_slice();
            assert!(input.len() == 4);
            shapes.push(vec![input[0], input[1], 1, 1].into());
        }
        Op::Reshape => {
            let shape = inputs[Node::RESHAPE_SHAPE]
                .data()
                .as_i64()
                .unwrap()
                .iter()
                .map(|&x| x as usize)
                .collect::<Vec<_>>();
            shapes.push(shape.into());
        }
        Op::Flatten(flatten) => {
            let dims = inputs[Node::FLATTEN_IN].dims().as_slice();
            assert!(flatten.axis >= 0);
            let x: Dimensions = dims[..flatten.axis as usize].to_vec().into();
            let y: Dimensions = dims[flatten.axis as usize..].to_vec().into();
            shapes.push(vec![x.total_elems(), y.total_elems()].into())
        }
        Op::MatMul => {
            let in_a = &inputs[Node::MATMUL_IN_A].dims();
            let in_b = &inputs[Node::MATMUL_IN_B].dims();
            assert_eq!(in_a.as_slice()[1], in_b.as_slice()[0]);
            shapes.push(vec![in_a.as_slice()[0], in_b.as_slice()[1]].into());
        }
        Op::Gemm(gemm) => {
            let in_a = &inputs[Node::GEMM_IN_A].dims().as_slice();
            let (in_a0, in_a1) = if gemm.trans_a {
                (in_a[1], in_a[0])
            } else {
                (in_a[0], in_a[1])
            };
            let in_b = &inputs[Node::GEMM_IN_B].dims().as_slice();
            let (in_b0, in_b1) = if gemm.trans_b {
                (in_b[1], in_b[0])
            } else {
                (in_b[0], in_b[1])
            };
            assert_eq!(in_a1, in_b0);
            shapes.push(vec![in_a0, in_b1].into());
        }
        Op::ReLU => {
            let input = inputs[Node::RELU_IN].dims();
            shapes.push(input.clone());
        }
        Op::HardSigmoid(_) => {
            let input = inputs[Node::HARDSIGMOID_IN].dims();
            shapes.push(input.clone());
        }
    }
    shapes
}
