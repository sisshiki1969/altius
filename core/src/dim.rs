use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

pub type Dimension = usize;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Dimensions(pub Vec<Dimension>);

impl Dimensions {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_scalar(&self) -> bool {
        self.is_empty() || (self.len() == 1 && matches!(self.0[0], 0 | 1))
    }

    pub fn total_elems(&self) -> usize {
        self.0.iter().product()
    }

    pub fn as_slice(&self) -> &[Dimension] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [Dimension] {
        self.0.as_mut_slice()
    }

    pub fn to_i32_vec(&self) -> Vec<i32> {
        self.0.iter().map(|&x| x as i32).collect()
    }

    pub fn from_i64(dims: &[i64]) -> Self {
        Self(dims.iter().map(|&x| x as Dimension).collect())
    }

    pub fn broadcast(&self, other: impl AsRef<Self>) -> Option<Self> {
        broadcast(&[self, other.as_ref()])
    }

    pub fn to_fixed_dims<const N: usize>(&self) -> [Dimension; N] {
        let mut dims: [Dimension; N] = [0; N];
        dims.copy_from_slice(&self.0);
        dims
    }
}

pub fn broadcast(shapes: &[impl AsRef<Dimensions>]) -> Option<Dimensions> {
    let mut shape = vec![];
    let max_len = shapes
        .iter()
        .map(AsRef::as_ref)
        .map(Dimensions::len)
        .max()?;
    for i in 0..max_len {
        let mut size = 1;
        for shape in shapes.iter().map(AsRef::as_ref) {
            let len = shape.len();
            let dim = if i < len { shape[len - i - 1] } else { 1 };
            if dim == 1 {
                continue;
            }
            if size != 1 && dim != size {
                return None;
            }
            size = dim
        }
        shape.push(size)
    }
    shape.reverse();
    Some(shape.into())
}

impl AsRef<Dimensions> for Dimensions {
    fn as_ref(&self) -> &Dimensions {
        self
    }
}

impl<I> Index<I> for Dimensions
where
    I: SliceIndex<[Dimension]>,
{
    type Output = <I as SliceIndex<[Dimension]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<I> IndexMut<I> for Dimensions
where
    I: SliceIndex<[Dimension]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Vec<Dimension>> for Dimensions {
    fn from(v: Vec<Dimension>) -> Dimensions {
        Dimensions(v)
    }
}

#[test]
fn total_elems() {
    assert_eq!(Dimensions(vec![1, 1, 28, 28]).total_elems(), 784)
}

#[test]
fn broadcast_1() {
    let one = Dimensions::from(vec![1]);
    let shape = broadcast(&[&one]).unwrap();
    assert_eq!(shape, one)
}

#[test]
fn broadcast_2() {
    let one = Dimensions::from(vec![1]);
    let four = Dimensions::from(vec![4, 1]);
    let shape = broadcast(&[one, four]).unwrap();
    assert_eq!(shape, vec![4, 1].into())
}

#[test]
fn broadcast_3() {
    let one = Dimensions::from(vec![1]);
    let four = Dimensions::from(vec![4, 1]);
    let shape = broadcast(&[four, one]).unwrap();
    assert_eq!(shape, vec![4, 1].into())
}

#[test]
#[should_panic]
fn broadcast_4() {
    let one = Dimensions::from(vec![10, 20]);
    let four = Dimensions::from(vec![10, 20, 30]);
    let _ = broadcast(&[four, one]).unwrap();
}

#[test]
fn broadcast_5() {
    let x = Dimensions::from(vec![1, 3, 3]);
    let y = Dimensions::from(vec![5, 1, 3, 3]);
    let shape = broadcast(&[x, y]).unwrap();
    assert_eq!(shape, vec![5, 1, 3, 3].into())
}

#[test]
fn broadcast_6() {
    let x = Dimensions::from(vec![1, 3, 1]);
    let y = Dimensions::from(vec![5, 3, 10]);
    let shape = broadcast(&[x, y]).unwrap();
    assert_eq!(shape, vec![5, 3, 10].into())
}

#[test]
fn broadcast_7() {
    let x = Dimensions::from(vec![1, 3, 4, 4]);
    let y = Dimensions::from(vec![3, 1, 1]);
    let shape = broadcast(&[x, y]).unwrap();
    assert_eq!(shape, vec![1, 3, 4, 4,].into())
}
