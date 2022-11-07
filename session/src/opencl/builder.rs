use altius_core::model::Model;
use opencl3::{
    command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE},
    context::Context,
    device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU},
};

use crate::SessionError;

use super::session::OpenclSession;

#[derive(Default)]
pub struct OpenclSessionBuilder<'a> {
    #[allow(dead_code)] // TODO: Remove later.
    model: Option<&'a Model>,
}

impl<'a> OpenclSessionBuilder<'a> {
    pub const fn new() -> Self {
        Self { model: None }
    }

    pub fn with_model(mut self, model: &'a Model) -> Self {
        self.model = Some(model);
        self
    }

    pub fn build(self) -> Result<OpenclSession<'a>, SessionError> {
        let model = self
            .model
            .ok_or(SessionError::Message("Model is not set".to_string()))?;

        let device_id = *get_all_devices(CL_DEVICE_TYPE_GPU)
            .map_err(|e| SessionError::Message(format!("Failed to get device: {}", e.to_string())))?
            .first()
            .unwrap();
        let device = Device::new(device_id);

        let context = Context::from_device(&device).map_err(|e| {
            SessionError::Message(format!("Failed to create context: {}", e.to_string()))
        })?;

        let queue = unsafe {
            CommandQueue::create_with_properties(
                &context,
                device_id,
                CL_QUEUE_PROFILING_ENABLE,
                0, /* TODO: What does queue_size mean? */
            )
        }
        .map_err(|e| {
            SessionError::Message(format!("Failed to create command queue: {}", e.to_string()))
        })?;

        Ok(OpenclSession {
            model,
            device,
            context,
            queue,
        })

        //
        //             // Create a command_queue on the Context's device
        //             let queue = CommandQueue::create_default(&context, CL_QUEUE_PROFILING_ENABLE)
        //                 .expect("CommandQueue::create_default failed");
        //
        //             // Build the OpenCL program source and create the kernel.
        //             let program = Program::create_and_build_from_source(&context, PROGRAM_SOURCE, "")
        //                 .expect("Program::create_and_build_from_source failed");
        //             let kernel = Kernel::create(&program, KERNEL_NAME).expect("Kernel::create failed");
        //
        //             /////////////////////////////////////////////////////////////////////
        //             // Compute data
        //
        //             // The input data
        //             const ARRAY_SIZE: usize = 1 << 29;
        //             // let ones: [cl_float; ARRAY_SIZE] = [1.0; ARRAY_SIZE];
        //             let ones = vec![1.0; ARRAY_SIZE];
        //             let mut sums = vec![0.0; ARRAY_SIZE];
        //             let start = Instant::now();
        //             for i in 0..ARRAY_SIZE {
        //                 sums[i] = 1.0 + 1.0 * i as cl_float;
        //             }
        //             println!("cpu: {:?}", start.elapsed());
        //
        //             // Create OpenCL device buffers
        //             let mut x = unsafe {
        //                 Buffer::<cl_float>::create(&context, CL_MEM_READ_ONLY, ARRAY_SIZE, ptr::null_mut())?
        //             };
        //             let mut y = unsafe {
        //                 Buffer::<cl_float>::create(&context, CL_MEM_READ_ONLY, ARRAY_SIZE, ptr::null_mut())?
        //             };
        //             let z = unsafe {
        //                 Buffer::<cl_float>::create(
        //                     &context,
        //                     CL_MEM_WRITE_ONLY,
        //                     ARRAY_SIZE,
        //                     ptr::null_mut(),
        //                 )?
        //             };
        //
        //             // Blocking write
        //             let _x_write_event =
        //                 unsafe { queue.enqueue_write_buffer(&mut x, cl_blocking, 0, &ones, &[])? };
        //
        //             // non-blocking write, wait for y_write_event
        //             let y_write_event =
        //                 unsafe { queue.enqueue_write_buffer(&mut y, cl_non_blocking, 0, &sums, &[])? };
        //
        //             // a value for the kernel function
        //             let a: cl_float = 300.0;
        //
        //             // use the executekernel builder to set the kernel buffer and
        //             // cl_float value arguments, before setting the one dimensional
        //             // global_work_size for the call to enqueue_nd_range.
        //             // unwraps the result to get the kernel execution event.
        //             let kernel_event = unsafe {
        //                 executekernel::new(&kernel)
        //                     .set_arg(&z)
        //                     .set_arg(&x)
        //                     .set_arg(&y)
        //                     .set_arg(&a)
        //                     .set_global_work_size(array_size)
        //                     .set_wait_event(&y_write_event)
        //                     .enqueue_nd_range(&queue)?
        //             };
        //
        //             let mut events: vec<cl_event> = vec::default();
        //             events.push(kernel_event.get());
        //
        //             // create a results array to hold the results from the opencl device
        //             // and enqueue a read command to read the device buffer into the array
        //             // after the kernel event completes.
        //             // let mut results: [cl_float; array_size] = [0.0; array_size];
        //             let mut results = vec![0.0; array_size];
        //             let start = instant::now();
        //             let read_event = unsafe {
        //                 queue.enqueue_read_buffer(&z, cl_non_blocking, 0, &mut results, &events)?
        //             };
        //
        //             // wait for the read_event to complete.
        //             read_event.wait()?;
        //             println!("gpu {:?}", start.elapsed());
        //
        //             // output the first and last results
        //             println!("results front: {}", results[0]);
        //             println!("results back: {}", results[array_size - 1]);
        //
        //             // calculate the kernel duration, from the kernel_event
        //             let start_time = kernel_event.profiling_command_start()?;
        //             let end_time = kernel_event.profiling_command_end()?;
        //             let duration = end_time - start_time;
        //             println!(
        //                 "kernel execution duration (ms): {}",
        //                 duration as f64 / 1000. / 1000.0
        //             );
        //
        //             ok(())
        //         }
    }
}

#[test]
fn test_build() {
    let model = Model::default();
    let _ = OpenclSessionBuilder::new()
        .with_model(&model)
        .build()
        .unwrap();
}