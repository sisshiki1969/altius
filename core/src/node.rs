use crate::{
    dim::Dimensions,
    tensor::{Tensor, TensorElemType, TypedShape},
    value::ValueId,
};
use id_arena::{Arena, Id};

pub type NodeId = Id<Node>;
pub type NodeArena = Arena<Node>;

#[derive(Debug, Clone)]
pub struct Node {
    pub op: Op,
    pub name: Option<String>,
    pub inputs: Vec<ValueId>,
    pub outputs: Vec<ValueId>,
    pub deleted: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Conv2d(Conv2d),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Sqrt,
    ReLU,
    Gelu,
    LeakyReLU(LeakyReLU),
    Sigmoid,
    Erf,
    Clip,
    Softmax(Softmax),
    Cast(Cast),
    MaxPool(MaxPool),
    GlobalAveragePool,
    Reshape,
    Flatten(Flatten),
    Resize(Resize),
    Concat(Concat),
    Transpose(Transpose),
    Squeeze(Squeeze),
    Unsqueeze(Unsqueeze),
    ReduceMin(ReduceMin),
    ReduceMean(ReduceMean),
    Round,
    Exp,
    Loop,
    Tile,
    Slice,
    Gather(Gather),
    Shape(Shape),
    NonMaxSuppression,
    MatMul,
    Gemm(Gemm),
    BatchNormalization(BatchNormalization),
    HardSigmoid(HardSigmoid),
    Constant(Constant),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Conv2d {
    pub auto_pad: String,
    pub dilations: Dimensions,
    pub group: i64,
    pub kernel_shape: Dimensions,
    pub strides: Dimensions,
    pub padding: Dimensions,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MaxPool {
    pub auto_pad: String,
    pub kernel_shape: Dimensions,
    pub strides: Dimensions,
    pub padding: Dimensions,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HardSigmoid {
    pub alpha: f32,
    pub beta: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LeakyReLU {
    pub alpha: f32,
}

/// https://github.com/onnx/onnx/blob/main/docs/Operators.md#Softmax
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Softmax {
    pub axis: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cast {
    pub to: TensorElemType,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Flatten {
    pub axis: i64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Resize {
    pub coordinate_transformation_mode: String,
    pub cubic_coeff_a: f32,
    pub exclude_outside: i64,
    pub extrapolation_value: f32,
    pub mode: String,
    pub nearest_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Concat {
    pub axis: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Transpose {
    pub perm: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Squeeze {
    pub axes: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Unsqueeze {
    pub axes: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReduceMin {
    pub axes: Vec<i64>,
    pub keep_dims: bool,
}

/// https://github.com/onnx/onnx/blob/main/docs/Operators.md#ReduceMean
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReduceMean {
    pub axes: Vec<i64>,
    pub keep_dims: bool,
}

/// https://github.com/onnx/onnx/blob/main/docs/Operators.md#Gather
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Gather {
    pub axis: i64,
}

/// https://github.com/onnx/onnx/blob/main/docs/Operators.md#Shape
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Shape {
    pub end: Option<i64>,
    pub start: i64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gemm {
    pub alpha: f32,
    pub beta: f32,
    pub trans_a: bool,
    pub trans_b: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BatchNormalization {
    pub epsilon: f32,
    pub momentum: f32,
    pub training_mode: bool,
}

/// https://github.com/onnx/onnx/blob/main/docs/Operators.md#Constant
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant {
    pub value: Tensor,
    // TODO: Other attributes
}

impl Node {
    pub const CONV2D_IN: usize = 0;
    pub const CONV2D_WEIGHT: usize = 1;
    pub const CONV2D_BIAS: usize = 2;
    pub const CONV2D_OUT: usize = 0;

    pub const ADD_IN_A: usize = 0;
    pub const ADD_IN_B: usize = 1;
    pub const ADD_OUT: usize = 0;

    pub const SUB_IN_A: usize = 0;
    pub const SUB_IN_B: usize = 1;
    pub const SUB_OUT: usize = 0;

    pub const MUL_IN_A: usize = 0;
    pub const MUL_IN_B: usize = 1;
    pub const MUL_OUT: usize = 0;

    pub const DIV_IN_A: usize = 0;
    pub const DIV_IN_B: usize = 1;
    pub const DIV_OUT: usize = 0;

    pub const RELU_IN: usize = 0;
    pub const RELU_OUT: usize = 0;

    pub const LEAKYRELU_IN: usize = 0;
    pub const LEAKYRELU_OUT: usize = 0;

    pub const SIGMOID_IN: usize = 0;
    pub const SIGMOID_OUT: usize = 0;

    pub const CLIP_IN: usize = 0;
    pub const CLIP_OUT: usize = 0;

    pub const CAST_IN: usize = 0;
    pub const CAST_OUT: usize = 0;

    pub const ROUND_IN: usize = 0;
    pub const ROUND_OUT: usize = 0;

    pub const EXP_IN: usize = 0;
    pub const EXP_OUT: usize = 0;

    pub const MAXPOOL_IN: usize = 0;
    pub const MAXPOOL_OUT: usize = 0;

    pub const GLOBALAVERAGEPOOL_IN: usize = 0;
    pub const GLOBALAVERAGEPOOL_OUT: usize = 0;

    pub const RESHAPE_IN: usize = 0;
    pub const RESHAPE_SHAPE: usize = 1;
    pub const RESHAPE_OUT: usize = 0;

    pub const FLATTEN_IN: usize = 0;
    pub const FLATTEN_OUT: usize = 0;

    pub const RESIZE_IN_X: usize = 0;
    pub const RESIZE_IN_ROI: usize = 1;
    pub const RESIZE_IN_SCALES: usize = 2;
    pub const RESIZE_IN_SIZES: usize = 3;
    pub const RESIZE_OUT: usize = 0;

    pub const CONCAT_IN: usize = 0; // variadic
    pub const CONCAT_OUT: usize = 0;

    pub const TRANSPOSE_IN: usize = 0;
    pub const TRANSPOSE_OUT: usize = 0;

    pub const SQUEEZE_IN: usize = 0;
    pub const SQUEEZE_OUT: usize = 0;

    pub const UNSQUEEZE_IN: usize = 0;
    pub const UNSQUEEZE_OUT: usize = 0;

    pub const REDUCEMIN_IN: usize = 0;
    pub const REDUCEMIN_OUT: usize = 0;

    pub const TILE_IN: usize = 0;
    pub const TILE_REPEATS: usize = 1;
    pub const TILE_OUT: usize = 0;

    pub const SLICE_IN_DATA: usize = 0;
    pub const SLICE_IN_STARTS: usize = 1;
    pub const SLICE_IN_ENDS: usize = 2;
    pub const SLICE_IN_AXES: usize = 3;
    pub const SLICE_IN_STEPS: usize = 4;
    pub const SLICE_OUT: usize = 0;

    pub const NMS_IN_BOXES: usize = 0;
    pub const NMS_IN_SCORES: usize = 1;
    pub const NMS_IN_MAX_OUTPUT_BOXES_PER_CLASS: usize = 2;
    pub const NMS_IN_NMS_IOU_THRESHOLD: usize = 3;
    pub const NMS_IN_NMS_SCORE_THRESHOLD: usize = 4;
    pub const NMS_OUT: usize = 0;

    pub const MATMUL_IN_A: usize = 0;
    pub const MATMUL_IN_B: usize = 1;
    pub const MATMUL_OUT: usize = 0;

    pub const GEMM_IN_A: usize = 0;
    pub const GEMM_IN_B: usize = 1;
    pub const GEMM_IN_C: usize = 2;
    pub const GEMM_OUT: usize = 0;

    pub const BATCHNORM_IN_X: usize = 0;
    pub const BATCHNORM_IN_SCALE: usize = 1;
    pub const BATCHNORM_IN_B: usize = 2;
    pub const BATCHNORM_IN_INPUT_MEAN: usize = 3;
    pub const BATCHNORM_IN_INPUT_VAR: usize = 4;
    pub const BATCHNORM_OUT_Y: usize = 0;

    pub const HARDSIGMOID_IN: usize = 0;
    pub const HARDSIGMOID_OUT: usize = 0;

    pub fn new(op: Op) -> Self {
        Self {
            op,
            name: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
            deleted: false,
        }
    }

    pub fn with_name(mut self, name: impl Into<Option<String>>) -> Self {
        self.name = name.into();
        self
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
        arena.alloc(self)
    }
}

impl Op {
    pub fn name(&self) -> &'static str {
        match self {
            Op::Conv2d(_) => "Conv2d",
            Op::Add => "Add",
            Op::Sub => "Sub",
            Op::Mul => "Mul",
            Op::Div => "Div",
            Op::Pow => "Pow",
            Op::Sqrt => "Sqrt",
            Op::ReLU => "ReLU",
            Op::LeakyReLU(_) => "LeakyReLU",
            Op::Gelu => "Gelu",
            Op::Sigmoid => "Sigmoid",
            Op::Erf => "Erf",
            Op::Clip => "Clip",
            Op::Softmax(_) => "Softmax",
            Op::Cast(_) => "Cast",
            Op::MaxPool(_) => "MaxPool",
            Op::GlobalAveragePool => "GlobalAveragePool",
            Op::Reshape => "Reshape",
            Op::Flatten(_) => "Flatten",
            Op::Resize(_) => "Resize",
            Op::Concat(_) => "Concat",
            Op::Transpose(_) => "Transpose",
            Op::Squeeze(_) => "Squeeze",
            Op::Unsqueeze(_) => "Unsqueeze",
            Op::ReduceMin(_) => "ReduceMin",
            Op::ReduceMean(_) => "ReduceMean",
            Op::Round => "Round",
            Op::Exp => "Exp",
            Op::Loop => "Loop",
            Op::Tile => "Tile",
            Op::Slice => "Slice",
            Op::Gather(_) => "Gather",
            Op::Shape(_) => "Shape",
            Op::NonMaxSuppression => "NonMaxSuppression",
            Op::MatMul => "MatMul",
            Op::Gemm(_) => "Gemm",
            Op::BatchNormalization(_) => "BatchNormalization",
            Op::HardSigmoid(_) => "HardSigmoid",
            Op::Constant(_) => "Constant",
        }
    }
}

/// Computes the output shape for `op`.
/// `op` could be overwritten. (e.g. paddings given auto_pad)
pub fn compute_output_shapes(op: &mut Op, inputs: &[&Tensor]) -> Vec<TypedShape> {
    let mut shapes = vec![];
    match op {
        Op::Conv2d(conv) => {
            let auto_pad = &conv.auto_pad;
            let kernel = &conv.kernel_shape;
            let stride = &conv.strides;
            let padding = &conv.padding;
            let dilations = &conv.dilations;
            let input = inputs[Node::CONV2D_IN].dims();
            let weight = inputs[Node::CONV2D_WEIGHT].dims();

            assert_eq!(dilations.len(), 2);

            let pad_h;
            let pad_w;
            if !auto_pad.is_empty() && auto_pad != "NOTSET" {
                let out0 = (input[2] as f32 / stride[0] as f32).ceil() as usize;
                let out1 = (input[3] as f32 / stride[1] as f32).ceil() as usize;
                let pad0 =
                    ((out0 - 1) * stride[0] + ((kernel[0] - 1) + 1)).saturating_sub(input[2]);
                let pad1 =
                    ((out1 - 1) * stride[1] + ((kernel[1] - 1) + 1)).saturating_sub(input[3]);
                assert!(auto_pad == "SAME_UPPER");
                let new_padding = vec![pad0 / 2, pad1 / 2, pad0 - pad0 / 2, pad1 - pad1 / 2];
                conv.padding = new_padding.into();
                pad_h = pad0;
                pad_w = pad1;
            } else if padding.len() == 2 {
                pad_h = padding[0] * 2;
                pad_w = padding[1] * 2;
                conv.padding = vec![padding[0], padding[1], padding[0], padding[1]].into();
            } else if padding.len() == 4 {
                pad_h = padding[0] + padding[2];
                pad_w = padding[1] + padding[3];
            } else {
                todo!()
            }

            let h_in = input[2];
            let w_in = input[3];

            let output_shape = vec![
                input[0],
                weight[0],
                (h_in + pad_h - dilations[0] * (kernel[0] - 1) - 1) / stride[0] + 1,
                (w_in + pad_w - dilations[1] * (kernel[1] - 1) - 1) / stride[1] + 1,
            ];
            shapes.push(TypedShape::new(
                output_shape.into(),
                inputs[Node::CONV2D_IN].elem_ty(),
            ));
        }
        Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Pow => {
            let x = inputs[0].dims();
            let y = inputs[1].dims();
            let shape = x.broadcast(y).unwrap();
            shapes.push(TypedShape::new(shape, inputs[0].elem_ty()));
        }
        Op::MaxPool(maxpool) => {
            let auto_pad = &maxpool.auto_pad;
            let kernel = &maxpool.kernel_shape;
            let stride = &maxpool.strides;
            let input = &inputs[Node::MAXPOOL_IN].dims();
            let mut padding = &maxpool.padding;

            if !auto_pad.is_empty() && auto_pad != "NOTSET" {
                let out0 = (input[2] as f32 / stride[0] as f32).ceil() as usize;
                let out1 = (input[3] as f32 / stride[1] as f32).ceil() as usize;
                let pad0 =
                    ((out0 - 1) * stride[0] + ((kernel[0] - 1) + 1)).saturating_sub(input[2]);
                let pad1 =
                    ((out1 - 1) * stride[1] + ((kernel[1] - 1) + 1)).saturating_sub(input[3]);
                assert!(auto_pad == "SAME_UPPER");
                let new_padding = vec![pad0 / 2, pad1 / 2, pad0 - pad0 / 2, pad1 - pad1 / 2];
                maxpool.padding = new_padding.into();
                padding = &maxpool.padding;
            }

            let h_in = input[2];
            let w_in = input[3];
            let output_shape = vec![
                input[0],
                input[1],
                (h_in + (padding[0] + padding[2]) - (kernel[0] - 1) - 1) / stride[0] + 1,
                (w_in + (padding[1] + padding[3]) - (kernel[1] - 1) - 1) / stride[1] + 1,
            ];
            // shapes.push(output_shape.into());
            shapes.push(TypedShape::new(
                output_shape.into(),
                inputs[Node::MAXPOOL_IN].elem_ty(),
            ));
        }
        Op::GlobalAveragePool => {
            let input = &inputs[Node::GLOBALAVERAGEPOOL_IN].dims();
            assert!(input.len() == 4);
            shapes.push(TypedShape::new(
                vec![input[0], input[1], 1, 1].into(),
                inputs[Node::GLOBALAVERAGEPOOL_IN].elem_ty(),
            ));
        }
        Op::Reshape => {
            let shape = inputs[Node::RESHAPE_SHAPE]
                .data::<i64>()
                .iter()
                .map(|&x| {
                    if x == -1 {
                        let other_dims_sz: i64 = inputs[Node::RESHAPE_SHAPE]
                            .data::<i64>()
                            .iter()
                            .filter(|x| **x != -1)
                            .product();
                        inputs[Node::RESHAPE_IN].dims().total_elems() / other_dims_sz as usize
                    } else {
                        x as usize
                    }
                })
                .collect::<Vec<_>>();
            shapes.push(TypedShape::new(
                shape.into(),
                inputs[Node::RESHAPE_IN].elem_ty(),
            ))
        }
        Op::Flatten(flatten) => {
            let dims = inputs[Node::FLATTEN_IN].dims();
            assert!(flatten.axis >= 0);
            let x: Dimensions = dims[..flatten.axis as usize].to_vec().into();
            let y: Dimensions = dims[flatten.axis as usize..].to_vec().into();
            shapes.push(TypedShape::new(
                vec![x.total_elems(), y.total_elems()].into(),
                inputs[Node::FLATTEN_IN].elem_ty(),
            ));
        }
        Op::Resize(resize) => {
            if inputs.len() == 4 {
                let sizes = &inputs[Node::RESIZE_IN_SIZES];
                assert!(sizes.dims().len() == 1 && sizes.dims()[0] == 4);
                shapes.push(TypedShape::new(
                    Dimensions::from_i64(sizes.data::<i64>()),
                    inputs[Node::RESIZE_IN_X].elem_ty(),
                ))
            } else if inputs.len() == 3 {
                // TODO: Support other cases.
                assert!(resize.coordinate_transformation_mode == "asymmetric");
                assert!(resize.mode == "nearest");
                assert!(resize.nearest_mode == "floor");
                assert!(inputs.len() == 3);
                let x = &inputs[Node::RESIZE_IN_X];
                assert!(x.dims().len() == 4);
                let _roi = &inputs[Node::RESIZE_IN_ROI].data::<f32>(); // TODO: According to https://github.com/onnx/onnx/blob/main/docs/Operators.md#Resize,
                                                                       // it only takes effect when coordinate_transformation_mode is "tf_crop_and_resize".
                                                                       // Since we assume coordinate_transformation_mode is "asymmetric" for now, just ignore roi.
                let scales = &inputs[Node::RESIZE_IN_SCALES].data::<f32>();
                shapes.push(TypedShape::new(
                    vec![
                        (x.dims()[0] as f32 * scales[0]).floor() as usize,
                        (x.dims()[1] as f32 * scales[1]).floor() as usize,
                        (x.dims()[2] as f32 * scales[2]).floor() as usize,
                        (x.dims()[3] as f32 * scales[3]).floor() as usize,
                    ]
                    .into(),
                    inputs[Node::RESIZE_IN_X].elem_ty(),
                ))
                // NOTE: Use the following code when roi takes effect ... right?
                // shapes.push(TypedShape::new(
                //     vec![
                //         (x.dims()[0] as f32 * (roi[4] - roi[0]) * scales[0]).floor() as usize,
                //         (x.dims()[1] as f32 * (roi[5] - roi[1]) * scales[1]).floor() as usize,
                //         (x.dims()[2] as f32 * (roi[6] - roi[2]) * scales[2]).floor() as usize,
                //         (x.dims()[3] as f32 * (roi[7] - roi[3]) * scales[3]).floor() as usize,
                //     ]
                //     .into(),
                //     inputs[Node::RESIZE_IN_X].elem_ty(),
                // ))
            } else {
                todo!()
            }
        }
        Op::Concat(concat) => {
            let mut dims = inputs[Node::CONCAT_IN].dims().clone();
            let mut sum = 0;
            for i in inputs {
                sum += i.dims()[concat.axis as usize];
            }
            dims.as_mut_slice()[concat.axis as usize] = sum;
            shapes.push(TypedShape::new(dims, inputs[Node::CONCAT_IN].elem_ty()))
        }
        Op::Transpose(trans) => {
            assert!(!trans.perm.is_empty());
            let in_dims = inputs[Node::TRANSPOSE_IN].dims().as_slice();
            let mut dims = vec![0usize; in_dims.len()];
            for i in 0..in_dims.len() {
                dims[i] = in_dims[trans.perm[i] as usize];
            }
            shapes.push(TypedShape::new(
                dims.into(),
                inputs[Node::TRANSPOSE_IN].elem_ty(),
            ))
        }
        Op::Squeeze(squeeze) => {
            assert!(!squeeze.axes.is_empty());
            assert!(squeeze.axes.iter().all(|&x| x >= 0));
            let in_dims = inputs[Node::SQUEEZE_IN].dims().as_slice();
            let mut dims = vec![];
            for (i, &x) in in_dims.iter().enumerate() {
                if squeeze.axes.contains(&(i as i64)) {
                    continue;
                }
                dims.push(x);
            }
            shapes.push(TypedShape::new(
                dims.into(),
                inputs[Node::SQUEEZE_IN].elem_ty(),
            ))
        }
        Op::Unsqueeze(unsqueeze) => {
            assert!(!unsqueeze.axes.is_empty());
            assert!(unsqueeze.axes.iter().all(|&x| x >= 0));
            let in_dims = inputs[Node::UNSQUEEZE_IN].dims().as_slice().to_vec();
            let mut dims = in_dims;
            for &x in unsqueeze.axes.iter() {
                dims.insert(x as usize, 1);
            }
            shapes.push(TypedShape::new(
                dims.into(),
                inputs[Node::UNSQUEEZE_IN].elem_ty(),
            ))
        }
        Op::ReduceMin(rmin) => {
            let in_dims = inputs[0].dims();
            let keepdims = if rmin.keep_dims { Some(1) } else { None };
            let axes = rmin
                .axes
                .iter()
                .map(|&axis| {
                    if axis < 0 {
                        (in_dims.len() as i64 + axis) as usize
                    } else {
                        axis as usize
                    }
                })
                .collect::<Vec<_>>();
            let mut dims = in_dims
                .as_slice()
                .iter()
                .enumerate()
                .filter_map(|(i, &d)| if axes.contains(&i) { keepdims } else { Some(d) })
                .collect::<Vec<_>>();
            if dims.is_empty() {
                dims.push(1);
            }
            shapes.push(TypedShape::new(dims.into(), inputs[0].elem_ty()))
        }
        Op::ReduceMean(rmean) => {
            let in_dims = inputs[0].dims();
            let keepdims = if rmean.keep_dims { Some(1) } else { None };
            let axes = rmean
                .axes
                .iter()
                .map(|&axis| {
                    if axis < 0 {
                        (in_dims.len() as i64 + axis) as usize
                    } else {
                        axis as usize
                    }
                })
                .collect::<Vec<_>>();
            let mut dims = in_dims
                .as_slice()
                .iter()
                .enumerate()
                .filter_map(|(i, &d)| if axes.contains(&i) { keepdims } else { Some(d) })
                .collect::<Vec<_>>();
            if dims.is_empty() {
                dims.push(1);
            }
            shapes.push(TypedShape::new(dims.into(), inputs[0].elem_ty()))
        }
        Op::Loop => {
            assert!(inputs.len() == 3);
            let _m = inputs[0].data::<i64>();
            let cond = inputs[1].data::<u8>();
            assert!(cond[0] == 1);
            let v_initial = inputs[2].data::<i32>();
            assert!(v_initial[0] == 0);
            todo!()
            // shapes.push(vec![1].into());
            // shapes.push(vec![m[0] as usize].into());
        }
        Op::Tile => {
            let in_dims = inputs[Node::TILE_IN].dims();
            let reps = inputs[Node::TILE_REPEATS].data::<i64>();
            let mut dims = vec![];
            for (i, &x) in in_dims.as_slice().iter().enumerate() {
                dims.push(x * reps[i as usize] as usize);
            }
            shapes.push(TypedShape::new(
                dims.into(),
                inputs[Node::TILE_IN].elem_ty(),
            ));
        }
        Op::Slice => {
            let in_data_dims = inputs[Node::SLICE_IN_DATA].dims();
            let in_starts = inputs[Node::SLICE_IN_STARTS].data::<i64>();
            let in_ends = inputs[Node::SLICE_IN_ENDS].data::<i64>();
            let in_axes = inputs[Node::SLICE_IN_AXES].data::<i64>();
            let ones = vec![1i64; in_axes.len()];
            let in_steps = inputs
                .get(Node::SLICE_IN_STEPS)
                .map_or(ones.as_slice(), |s| s.data::<i64>());
            assert!(in_starts.iter().all(|&x| x >= 0));
            assert!(in_ends.iter().all(|&x| x >= 0));
            assert!(in_steps.iter().all(|&x| x >= 0));
            let mut dims = in_data_dims.clone();
            for (((start, end), &axis), step) in in_starts
                .iter()
                .zip(in_ends.iter())
                .zip(in_axes.iter())
                .zip(in_steps.iter())
            {
                let start = *start as usize;
                let end = *end as usize;
                let axis = if axis < 0 {
                    (in_data_dims.len() as i64 + axis) as usize
                } else {
                    axis as usize
                };
                let step = *step as usize;
                let out_dim = (end - start) / step;
                assert!(out_dim > 0);
                dims[axis] = out_dim;
            }
            shapes.push(TypedShape::new(dims, inputs[Node::SLICE_IN_DATA].elem_ty()))
        }
        Op::Gather(gather) => {
            let mut data = inputs[0].dims().0.to_owned();
            let indices = inputs[1].dims();
            assert!(gather.axis >= 0);
            assert!(
                indices.is_scalar() || (indices.len() == 2 && indices[0] == 1),
                "Unsupported indices shape: {indices:?}"
            );
            if indices.is_scalar() {
                data.remove(gather.axis as usize);
                shapes.push(TypedShape::new(data.into(), inputs[0].elem_ty()))
            } else {
                assert_eq!(gather.axis, 0);
                data.remove(gather.axis as usize);
                data.insert(0, 1);
                data.insert(1, indices[1]);
                shapes.push(TypedShape::new(data.into(), inputs[0].elem_ty()))
            }
        }
        Op::Shape(_) => {
            todo!()
        }
        Op::NonMaxSuppression => {
            todo!()
        }
        Op::MatMul => {
            let in_a = &inputs[Node::MATMUL_IN_A].dims();
            let in_b = &inputs[Node::MATMUL_IN_B].dims();
            assert!(
                in_a[1] == in_b[0]
                    || (in_a.len() == 3 && in_b.len() == 2 && in_a[2] == in_b[0])
                    || (in_a.len() == 3
                        && in_b.len() == 3
                        && in_a[0] == in_b[0]
                        && in_a[2] == in_b[1])
                    || (in_a.len() == 4
                        && in_b.len() == 4
                        && in_a[0] == 1
                        && in_b[0] == 1
                        && in_a[1] == in_b[1]),
                "A shape: {in_a:?}, B shape: {in_b:?}"
            );
            if in_a.len() == 4 && in_b.len() == 4 {
                shapes.push(TypedShape::new(
                    vec![in_a[0], in_a[1], in_a[2], in_b[3]].into(),
                    inputs[Node::MATMUL_IN_A].elem_ty(),
                ));
            } else if in_a.len() == 3 && in_b.len() == 2 {
                shapes.push(TypedShape::new(
                    vec![in_a[0], in_a[1], in_b[1]].into(),
                    inputs[Node::MATMUL_IN_A].elem_ty(),
                ));
            } else if in_a.len() == 3 && in_b.len() == 3 {
                shapes.push(TypedShape::new(
                    vec![in_a[0], in_a[1], in_b[2]].into(),
                    inputs[Node::MATMUL_IN_A].elem_ty(),
                ));
            } else {
                shapes.push(TypedShape::new(
                    vec![in_a[0], in_b[1]].into(),
                    inputs[Node::MATMUL_IN_A].elem_ty(),
                ));
            }
        }
        Op::Gemm(gemm) => {
            let in_a = &inputs[Node::GEMM_IN_A].dims();
            let (in_a0, in_a1) = if gemm.trans_a {
                (in_a[1], in_a[0])
            } else {
                (in_a[0], in_a[1])
            };
            let in_b = &inputs[Node::GEMM_IN_B].dims();
            let (in_b0, in_b1) = if gemm.trans_b {
                (in_b[1], in_b[0])
            } else {
                (in_b[0], in_b[1])
            };
            assert_eq!(in_a1, in_b0);
            shapes.push(TypedShape::new(
                vec![in_a0, in_b1].into(),
                inputs[Node::GEMM_IN_A].elem_ty(),
            ));
        }
        Op::Constant(_) => {
            todo!()
        }
        // Element-wise operations.
        Op::Sqrt
        | Op::ReLU
        | Op::LeakyReLU(_)
        | Op::Gelu
        | Op::Sigmoid
        | Op::Erf
        | Op::Clip
        | Op::Cast(_)
        | Op::HardSigmoid(_)
        | Op::Round
        | Op::Exp
        | Op::Softmax(_)
        | Op::BatchNormalization(_) => {
            let input = inputs[0];
            shapes.push(TypedShape::new(input.dims().clone(), input.elem_ty()));
        }
    }
    shapes
}
