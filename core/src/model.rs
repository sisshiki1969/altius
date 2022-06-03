use crate::{
    node::{NodeArena, NodeId},
    tensor::Tensor,
    value::{ValueArena, ValueId},
};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Default)]
pub struct Model {
    pub nodes: NodeArena,
    pub values: ValueArena,
    pub inits: FxHashMap<ValueId, Tensor>,
    pub inputs: Vec<ValueId>,
    pub outputs: Vec<ValueId>,
}

impl Model {
    pub fn get_value_users(&self) -> FxHashMap<ValueId, FxHashSet<NodeId>> {
        let mut value_users: FxHashMap<ValueId, FxHashSet<NodeId>> = FxHashMap::default();

        for (node_id, node) in self.nodes.iter() {
            for &input in node.inputs.iter() {
                value_users.entry(input).or_default().insert(node_id);
            }
        }

        value_users
    }

    pub fn topo_sort_nodes(&self) -> Vec<NodeId> {
        let value_users = self.get_value_users();

        let mut nodes = vec![];
        let mut num_node_inputs = FxHashMap::default();
        let mut que = vec![];

        let mut consts = self.inits.keys().copied().collect::<FxHashSet<_>>();
        consts.insert(self.inputs[0]);
        for (id, node) in self.nodes.iter() {
            let inputs = &node.inputs.clone().into_iter().collect::<FxHashSet<_>>() - &consts;
            num_node_inputs.insert(id, inputs.len());
            if inputs.is_empty() {
                que.push(id);
            }
        }

        while let Some(id) = que.pop() {
            nodes.push(id);
            for output in self.nodes[id].outputs.iter() {
                if self.outputs.contains(output) {
                    continue;
                }
                for n in value_users[output].iter() {
                    *num_node_inputs.get_mut(n).unwrap() -= 1;
                    if *num_node_inputs.get(n).unwrap() == 0 {
                        que.push(*n);
                    }
                }
            }
        }

        nodes
    }
}

#[test]
fn mnist_model() {
    use crate::node::{Node, Op};

    let mut m = Model::default();

    let conv0_in = m.values.new_val(); // Input tensor [1, 1, 28, 28]
    let conv0_weight = m.values.new_val();
    let conv0_out = m.values.new_val();
    let _conv0 = Node::new(Op::Conv2d)
        .with_attr("SAME_UPPER".into())
        .with_attr(vec![5, 5].into())
        .with_attr(vec![1, 1].into())
        .with_attr(vec![].into())
        .with_in(conv0_in)
        .with_in(conv0_weight)
        .with_out(conv0_out)
        .alloc(&mut m.nodes);

    let add0_const = m.values.new_val();
    let add0_out = m.values.new_val();
    let _add0 = Node::new(Op::Add)
        .with_in(conv0_out)
        .with_in(add0_const)
        .with_out(add0_out)
        .alloc(&mut m.nodes);

    let relu0_out = m.values.new_val();
    let _relu0 = Node::new(Op::ReLU)
        .with_in(add0_out)
        .with_out(relu0_out)
        .alloc(&mut m.nodes);

    let maxpool0_out = m.values.new_val();
    let _maxpool0 = Node::new(Op::MaxPool)
        .with_attr(vec![2, 2].into())
        .with_attr(vec![2, 2].into())
        .with_in(relu0_out)
        .with_out(maxpool0_out)
        .alloc(&mut m.nodes);

    let conv1_weight = m.values.new_val();
    let conv1_out = m.values.new_val();
    let _conv1 = Node::new(Op::Conv2d)
        .with_attr("SAME_UPPER".into())
        .with_attr(vec![5, 5].into())
        .with_attr(vec![1, 1].into())
        .with_attr(vec![2, 2].into())
        .with_in(maxpool0_out)
        .with_in(conv1_weight)
        .with_out(conv1_out)
        .alloc(&mut m.nodes);

    let add1_const = m.values.new_val();
    let add1_out = m.values.new_val();
    let _add1 = Node::new(Op::Add)
        .with_in(conv1_out)
        .with_in(add1_const)
        .with_out(add1_out)
        .alloc(&mut m.nodes);

    let relu1_out = m.values.new_val();
    let _relu1 = Node::new(Op::ReLU)
        .with_in(add1_out)
        .with_out(relu1_out)
        .alloc(&mut m.nodes);

    let maxpool1_out = m.values.new_val();
    let _maxpool1 = Node::new(Op::MaxPool)
        .with_in(relu1_out)
        .with_out(maxpool1_out)
        .with_attr(vec![3, 3].into())
        .with_attr(vec![3, 3].into())
        .alloc(&mut m.nodes);

    let reshape0_const = m.values.new_val();
    let reshape0_out = m.values.new_val();
    let _reshape0 = Node::new(Op::Reshape)
        .with_in(maxpool1_out)
        .with_in(reshape0_const)
        .with_out(reshape0_out)
        .alloc(&mut m.nodes);

    let reshape1_const0 = m.values.new_val();
    let reshape1_const1 = m.values.new_val();
    let reshape1_out = m.values.new_val();
    let _reshape1 = Node::new(Op::Reshape)
        .with_in(reshape1_const0)
        .with_in(reshape1_const1)
        .with_out(reshape1_out)
        .alloc(&mut m.nodes);

    let matmul0_out = m.values.new_val();
    let _matmul0 = Node::new(Op::MatMul)
        .with_in(reshape0_out)
        .with_in(reshape1_out)
        .with_out(matmul0_out)
        .alloc(&mut m.nodes);

    let add2_const = m.values.new_val();
    let add2_out = m.values.new_val();
    let _add2 = Node::new(Op::Add)
        .with_in(matmul0_out)
        .with_in(add2_const)
        .with_out(add2_out)
        .alloc(&mut m.nodes);

    m.inputs.push(conv0_in);
    m.outputs.push(add2_out);

    m.inits
        .insert(add0_const, Tensor::new(vec![8, 1, 5, 5].into()));
    m.inits
        .insert(add1_const, Tensor::new(vec![8, 1, 1].into()));
    m.inits
        .insert(add2_const, Tensor::new(vec![16, 1, 1].into()));
    m.inits
        .insert(conv0_weight, Tensor::new(vec![8, 1, 5, 5].into()));
    m.inits
        .insert(conv1_weight, Tensor::new(vec![16, 8, 5, 5].into()));
    m.inits.insert(
        reshape0_const,
        Tensor::new(vec![2].into()).with_data(vec![1, 256].into()),
    );
    m.inits
        .insert(reshape1_const0, Tensor::new(vec![16, 4, 4, 10].into()));
    m.inits.insert(
        reshape1_const1,
        Tensor::new(vec![2].into()).with_data(vec![256, 10].into()),
    );

    let order = m.topo_sort_nodes();
    // println!(
    //     "{:#?}",
    //     order.iter().map(|&n| m.nodes[n].op).collect::<Vec<_>>()
    // );
    insta::assert_debug_snapshot!(order);
}
