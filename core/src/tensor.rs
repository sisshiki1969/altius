use crate::dim::{Dimension, Dimensions};

#[derive(Debug, Clone)]
pub struct Tensor2 {
    dims: Dimensions,
    stride: Dimensions,
    data: TensorData2,
}

#[derive(Debug, Clone)]
pub enum TensorData2 {
    F32(Vec<f32>),
    I64(Vec<i64>),
}

impl TensorData2 {
    pub fn new_raw_f32(data: Vec<f32>) -> Self {
        Self::F32(data)
    }

    pub fn len(&self) -> usize {
        match self {
            Self::F32(data) => data.len(),
            Self::I64(data) => data.len(),
        }
    }

    pub fn as_f32(&self) -> Option<&[f32]> {
        match self {
            Self::F32(data) => Some(data.as_slice()),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<&[i64]> {
        match self {
            Self::I64(data) => Some(data.as_slice()),
            _ => None,
        }
    }
}

impl Tensor2 {
    pub fn new(dims: Dimensions) -> Self {
        Self {
            stride: compute_strides(&dims),
            data: TensorData2::F32(vec![]),
            dims,
        }
    }

    pub fn with_data(mut self, data: TensorData2) -> Self {
        self.data = data;
        #[cfg(debug_assertions)]
        assert!(self.verify());
        self
    }

    pub fn reshape_into(mut self, dims: Dimensions) -> Self {
        self.stride = compute_strides(&dims);
        self.dims = dims;
        debug_assert!(self.verify());
        self
    }

    pub fn verify(&self) -> bool {
        self.data.len() == self.dims.total_elems()
    }
}

impl From<Vec<f32>> for TensorData2 {
    fn from(data: Vec<f32>) -> Self {
        Self::F32(data)
    }
}

impl From<Vec<i64>> for TensorData2 {
    fn from(data: Vec<i64>) -> Self {
        Self::I64(data)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tensor {
    dims: Dimensions,
    stride: Dimensions,
    data: TensorData,
}

#[derive(Debug, Clone, Default)]
pub struct TensorData {
    // elem_ty: Type,
    data: Vec<f32>,
}

impl Tensor {
    pub fn new(dims: Dimensions) -> Self {
        Self {
            stride: compute_strides(&dims),
            data: TensorData::new_raw(vec![0.0; dims.total_elems()]),
            dims,
        }
    }

    pub fn with_data(mut self, data: TensorData) -> Self {
        self.data = data;
        #[cfg(debug_assertions)]
        assert!(self.verify());
        self
    }

    pub fn reshape_into(mut self, dims: Dimensions) -> Self {
        self.stride = compute_strides(&dims);
        self.dims = dims;
        debug_assert!(self.verify());
        self
    }

    pub fn at(&self, indices: &[Dimension]) -> f32 {
        let mut index = 0;
        for (idx, d) in indices.iter().zip(self.stride.as_slice().iter()) {
            index += d * idx;
        }
        self.data.data[index]
    }

    pub fn at_mut(&mut self, indices: &[Dimension]) -> &mut f32 {
        let mut index = 0;
        for (idx, d) in indices.iter().zip(self.stride.as_slice().iter()) {
            index += d * idx;
        }
        &mut self.data.data[index]
    }

    pub fn at_2d(&self, x: Dimension, y: Dimension) -> f32 {
        self.data.data[self.stride.as_slice()[0] * x + self.stride.as_slice()[1] * y]
    }

    pub fn at_2d_mut(&mut self, x: Dimension, y: Dimension) -> &mut f32 {
        &mut self.data.data[self.stride.as_slice()[0] * x + self.stride.as_slice()[1] * y]
    }

    pub fn at_3d(&self, x: Dimension, y: Dimension, z: Dimension) -> f32 {
        self.data.data[self.stride.as_slice()[0] * x
            + self.stride.as_slice()[1] * y
            + self.stride.as_slice()[2] * z]
    }

    pub fn at_3d_mut(&mut self, x: Dimension, y: Dimension, z: Dimension) -> &mut f32 {
        &mut self.data.data[self.stride.as_slice()[0] * x
            + self.stride.as_slice()[1] * y
            + self.stride.as_slice()[2] * z]
    }

    pub fn at_4d(&self, x: Dimension, y: Dimension, z: Dimension, u: Dimension) -> f32 {
        self.data.data[self.stride.as_slice()[0] * x
            + self.stride.as_slice()[1] * y
            + self.stride.as_slice()[2] * z
            + self.stride.as_slice()[3] * u]
    }

    pub fn at_4d_mut(
        &mut self,
        x: Dimension,
        y: Dimension,
        z: Dimension,
        u: Dimension,
    ) -> &mut f32 {
        &mut self.data.data[self.stride.as_slice()[0] * x
            + self.stride.as_slice()[1] * y
            + self.stride.as_slice()[2] * z
            + self.stride.as_slice()[3] * u]
    }

    pub fn data(&self) -> &[f32] {
        &self.data.data
    }

    pub fn data_mut(&mut self) -> &mut [f32] {
        &mut self.data.data
    }

    pub fn data_vec(&self) -> &Vec<f32> {
        &self.data.data
    }

    pub fn data_vec_mut(&mut self) -> &mut Vec<f32> {
        &mut self.data.data
    }

    pub fn dims(&self) -> &Dimensions {
        &self.dims
    }

    pub fn verify(&self) -> bool {
        self.data.len() == self.dims.total_elems()
    }
}

fn compute_strides(dims: &Dimensions) -> Dimensions {
    let mut strides = vec![];
    for i in 0..dims.as_slice().len() {
        strides.push(dims.as_slice()[i + 1..].iter().product());
    }
    strides.into()
}

impl TensorData {
    pub fn new_empty() -> Self {
        Self { data: vec![] }
    }

    pub fn new_raw(data: Vec<f32>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl From<Vec<f32>> for TensorData {
    fn from(data: Vec<f32>) -> TensorData {
        Self { data }
    }
}

#[test]
fn create_tensors() {
    let _ = Tensor::new(Dimensions(vec![1, 1, 28, 28]));
    let t = Tensor::new(Dimensions(vec![4, 4])).with_data(TensorData::new_raw(vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ]));
    assert!(t.verify());
}