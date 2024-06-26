use altius_core::onnx::load_onnx;
use altius_core::tensor::*;
use altius_session_interpreter::InterpreterSessionBuilder;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    env_logger::init();
    color_backtrace::install();

    let mnist_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models");
    let mnist = load_onnx(mnist_root.join("mnist-8.onnx")).unwrap();

    let mut inputs = vec![];
    for line in fs::read_to_string(Path::new(&mnist_root).join("MNIST_test.txt"))
        .unwrap()
        .split('\n')
    {
        if line.is_empty() {
            continue;
        }
        let nums: Vec<&str> = line.split(',').collect();
        let expected: i32 = nums[0].parse().unwrap();
        let pixels: Tensor = Tensor::new(
            vec![1, 1, 28, 28].into(),
            nums[1..]
                .iter()
                .map(|s| s.parse::<f32>().unwrap() / 255.0)
                .collect::<Vec<_>>(),
        );
        inputs.push((expected, pixels));
    }

    let start = Instant::now();

    let validation_count = 10000;
    let sess = InterpreterSessionBuilder::new(mnist).build().unwrap();

    let correct: i32 = inputs
        .iter()
        .take(validation_count)
        .map(|(expected, input)| {
            let v = sess.run(vec![input.clone()]).expect("Inference failed");
            let inferred = v[0]
                .data::<f32>()
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .map(|(index, _)| index)
                .unwrap();
            (*expected == inferred as i32) as i32
        })
        .sum();

    let end = start.elapsed();
    println!("elapsed: {end:?}");
    println!("fps: {:?}", (validation_count as f64) / end.as_secs_f64());

    // for (_expected, input) in &inputs {
    //     for x in 0..28 {
    //         for y in 0..28 {
    //             let pixel = input.at(&[0, 0, x, y]);
    //             print!("{}", if pixel > 0.5 { '#' } else { ' ' });
    //         }
    //         println!();
    //     }
    //     // break;
    // }

    println!("accuracy: {}", correct as f32 / validation_count as f32);
}
