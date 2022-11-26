#[cfg(target_arch = "wasm32")]
use std::marker::PhantomData;

#[cfg(not(target_arch = "wasm32"))]
use threadpool::ThreadPool;

pub struct ThreadCtx {
    #[cfg(not(target_arch = "wasm32"))]
    pub tp: ThreadPool,

    #[cfg(not(target_arch = "wasm32"))]
    num_threads: usize,
}

pub struct Scope<'a> {
    #[cfg(not(target_arch = "wasm32"))]
    ctx: &'a ThreadCtx,

    #[cfg(target_arch = "wasm32")]
    _phantom: PhantomData<&'a fn()>,
}

impl<'a> Scope<'a> {
    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'a,
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.ctx.num_threads() == 1 {
                f()
            } else {
                let f = unsafe {
                    std::mem::transmute::<
                        Box<dyn FnOnce() + Send + 'a>,
                        Box<dyn FnOnce() + Send + 'static>,
                    >(Box::new(f))
                };
                self.ctx.tp.execute(f)
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            f()
        }
    }
}

impl ThreadCtx {
    #[allow(dead_code)]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        let tp = ThreadPool::new(1);
        tp.execute(move || {
            core_affinity::set_for_current(core_affinity::CoreId { id: 0 });
        });
        tp.join();
        Self { tp, num_threads: 1 }
    }

    #[allow(dead_code)]
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        Self {}
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_with_num_threads(n: usize) -> Self {
        let tp = ThreadPool::new(n);
        for i in 0..n {
            tp.execute(move || {
                core_affinity::set_for_current(core_affinity::CoreId { id: i });
            })
        }
        tp.join();
        Self { tp, num_threads: n }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_with_num_threads(_n: usize) -> Self {
        Self {}
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub const fn num_threads(&self) -> usize {
        self.num_threads
    }

    #[cfg(target_arch = "wasm32")]
    pub fn num_threads(&self) -> usize {
        1
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn scope<F>(&self, mut f: F)
    where
        F: FnMut(&Scope),
    {
        let scope = Scope { ctx: self };
        f(&scope);
        if self.num_threads != 1 {
            self.tp.join();
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn scope<F>(&self, mut f: F)
    where
        F: FnMut(&Scope),
    {
        let scope = Scope {
            _phantom: PhantomData::default(),
        };
        f(&scope);
    }
}
