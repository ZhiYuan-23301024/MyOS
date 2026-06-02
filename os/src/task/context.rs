// os/src/task/context.rs

#[repr(C)]
pub struct TaskContext {
    ra: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn goto_restore() -> Self {
        unsafe extern "C" { fn __restore(); }
        Self {
            ra: __restore as *const () as usize,
            s: [0; 12],
        }
    }
}

