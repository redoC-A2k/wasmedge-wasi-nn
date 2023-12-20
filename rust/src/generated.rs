// This file is automatically generated, DO NOT EDIT
//
// To regenerate this file run the `crates/witx-bindgen` command

use core::fmt;
use core::mem::MaybeUninit;
pub type BufferSize = u32;
#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct NnErrno(u16);
pub const NN_ERRNO_SUCCESS: NnErrno = NnErrno(0);
pub const NN_ERRNO_INVALID_ARGUMENT: NnErrno = NnErrno(1);
pub const NN_ERRNO_INVALID_ENCODING: NnErrno = NnErrno(2);
pub const NN_ERRNO_MISSING_MEMORY: NnErrno = NnErrno(3);
pub const NN_ERRNO_BUSY: NnErrno = NnErrno(4);
pub const NN_ERRNO_RUNTIME_ERROR: NnErrno = NnErrno(5);
pub const NN_ERRNO_UNSUPPORTED_OPERATION: NnErrno = NnErrno(6);
pub const NN_ERRNO_TOO_LARGE: NnErrno = NnErrno(7);
pub const NN_ERRNO_NOT_FOUND: NnErrno = NnErrno(8);
impl NnErrno {
    pub const fn raw(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "SUCCESS",
            1 => "INVALID_ARGUMENT",
            2 => "INVALID_ENCODING",
            3 => "MISSING_MEMORY",
            4 => "BUSY",
            5 => "RUNTIME_ERROR",
            6 => "UNSUPPORTED_OPERATION",
            7 => "TOO_LARGE",
            8 => "NOT_FOUND",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "",
            1 => "",
            2 => "",
            3 => "",
            4 => "",
            5 => "",
            6 => "",
            7 => "",
            8 => "",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for NnErrno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NnErrno")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}
impl fmt::Display for NnErrno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (error {})", self.name(), self.0)
    }
}

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
impl std::error::Error for NnErrno {}

pub type TensorDimensions<'a> = &'a [u32];
#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TensorType(u8);
pub const TENSOR_TYPE_F16: TensorType = TensorType(0);
pub const TENSOR_TYPE_F32: TensorType = TensorType(1);
pub const TENSOR_TYPE_F64: TensorType = TensorType(2);
pub const TENSOR_TYPE_U8: TensorType = TensorType(3);
pub const TENSOR_TYPE_I32: TensorType = TensorType(4);
pub const TENSOR_TYPE_I64: TensorType = TensorType(5);
impl TensorType {
    pub const fn raw(&self) -> u8 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "F16",
            1 => "F32",
            2 => "F64",
            3 => "U8",
            4 => "I32",
            5 => "I64",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "",
            1 => "",
            2 => "",
            3 => "",
            4 => "",
            5 => "",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for TensorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TensorType")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

pub type TensorData<'a> = &'a [u8];
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Tensor<'a> {
    pub dimensions: TensorDimensions<'a>,
    pub type_: TensorType,
    pub data: TensorData<'a>,
}
pub type GraphBuilder<'a> = &'a [u8];
pub type GraphBuilderArray<'a> = &'a [GraphBuilder<'a>];
pub type Graph = u32;
#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct GraphEncoding(u8);
pub const GRAPH_ENCODING_OPENVINO: GraphEncoding = GraphEncoding(0);
pub const GRAPH_ENCODING_ONNX: GraphEncoding = GraphEncoding(1);
pub const GRAPH_ENCODING_TENSORFLOW: GraphEncoding = GraphEncoding(2);
pub const GRAPH_ENCODING_PYTORCH: GraphEncoding = GraphEncoding(3);
pub const GRAPH_ENCODING_TENSORFLOWLITE: GraphEncoding = GraphEncoding(4);
pub const GRAPH_ENCODING_AUTODETECT: GraphEncoding = GraphEncoding(5);
impl GraphEncoding {
    pub const fn raw(&self) -> u8 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "OPENVINO",
            1 => "ONNX",
            2 => "TENSORFLOW",
            3 => "PYTORCH",
            4 => "TENSORFLOWLITE",
            5 => "AUTODETECT",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "",
            1 => "",
            2 => "",
            3 => "",
            4 => "",
            5 => "",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for GraphEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphEncoding")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ExecutionTarget(u8);
pub const EXECUTION_TARGET_CPU: ExecutionTarget = ExecutionTarget(0);
pub const EXECUTION_TARGET_GPU: ExecutionTarget = ExecutionTarget(1);
pub const EXECUTION_TARGET_TPU: ExecutionTarget = ExecutionTarget(2);
impl ExecutionTarget {
    pub const fn raw(&self) -> u8 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "CPU",
            1 => "GPU",
            2 => "TPU",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "",
            1 => "",
            2 => "",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for ExecutionTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutionTarget")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

pub type GraphExecutionContext = u32;
pub unsafe fn load(
    builder: GraphBuilderArray<'_>,
    encoding: GraphEncoding,
    target: ExecutionTarget,
) -> Result<Graph, NnErrno> {
    let mut rp0 = MaybeUninit::<Graph>::uninit();
    let ret = wasi_ephemeral_nn::load(
        builder.as_ptr() as i32,
        builder.len() as i32,
        encoding.0 as i32,
        target.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Graph)),
        _ => Err(NnErrno(ret as u16)),
    }
}

pub unsafe fn load_by_name(name: &str) -> Result<Graph, NnErrno> {
    let mut rp0 = MaybeUninit::<Graph>::uninit();
    let ret = wasi_ephemeral_nn::load_by_name(
        name.as_ptr() as i32,
        name.len() as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Graph)),
        _ => Err(NnErrno(ret as u16)),
    }
}

pub unsafe fn load_by_name_with_config(name: &str, config: &str) -> Result<Graph, NnErrno> {
    let mut rp0 = MaybeUninit::<Graph>::uninit();
    let ret = wasi_ephemeral_nn::load_by_name_with_config(
        name.as_ptr() as i32,
        name.len() as i32,
        config.as_ptr() as i32,
        config.len() as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Graph)),
        _ => Err(NnErrno(ret as u16)),
    }
}

pub unsafe fn init_execution_context(graph: Graph) -> Result<GraphExecutionContext, NnErrno> {
    let mut rp0 = MaybeUninit::<GraphExecutionContext>::uninit();
    let ret = wasi_ephemeral_nn::init_execution_context(graph as i32, rp0.as_mut_ptr() as i32);
    match ret {
        0 => Ok(core::ptr::read(
            rp0.as_mut_ptr() as i32 as *const GraphExecutionContext
        )),
        _ => Err(NnErrno(ret as u16)),
    }
}

pub unsafe fn set_input(
    context: GraphExecutionContext,
    index: u32,
    tensor: Tensor,
) -> Result<(), NnErrno> {
    let ret =
        wasi_ephemeral_nn::set_input(context as i32, index as i32, &tensor as *const _ as i32);
    match ret {
        0 => Ok(()),
        _ => Err(NnErrno(ret as u16)),
    }
}

pub unsafe fn get_output(
    context: GraphExecutionContext,
    index: u32,
    out_buffer: *mut u8,
    out_buffer_max_size: BufferSize,
) -> Result<BufferSize, NnErrno> {
    let mut rp0 = MaybeUninit::<BufferSize>::uninit();
    let ret = wasi_ephemeral_nn::get_output(
        context as i32,
        index as i32,
        out_buffer as i32,
        out_buffer_max_size as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const BufferSize)),
        _ => Err(NnErrno(ret as u16)),
    }
}

pub unsafe fn get_output_single(
    context: GraphExecutionContext,
    index: u32,
    out_buffer: *mut u8,
    out_buffer_max_size: BufferSize,
) -> Result<BufferSize, NnErrno> {
    let mut rp0 = MaybeUninit::<BufferSize>::uninit();
    let ret = wasi_ephemeral_nn::get_output_single(
        context as i32,
        index as i32,
        out_buffer as i32,
        out_buffer_max_size as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const BufferSize)),
        _ => Err(NnErrno(ret as u16)),
    }
}

pub unsafe fn compute(context: GraphExecutionContext) -> Result<(), NnErrno> {
    let ret = wasi_ephemeral_nn::compute(context as i32);
    match ret {
        0 => Ok(()),
        _ => Err(NnErrno(ret as u16)),
    }
}

pub unsafe fn compute_single(context: GraphExecutionContext) -> Result<(), NnErrno> {
    let ret = wasi_ephemeral_nn::compute_single(context as i32);
    match ret {
        0 => Ok(()),
        _ => Err(NnErrno(ret as u16)),
    }
}

pub mod wasi_ephemeral_nn {
    #[link(wasm_import_module = "wasi_ephemeral_nn")]
    extern "C" {
        pub fn load(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        pub fn load_by_name(arg0: i32, arg1: i32, arg2: i32) -> i32;
        pub fn init_execution_context(arg0: i32, arg1: i32) -> i32;
        pub fn set_input(arg0: i32, arg1: i32, arg2: i32) -> i32;
        pub fn get_output(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        pub fn get_output_single(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        pub fn compute(arg0: i32) -> i32;
        pub fn compute_single(arg0: i32) -> i32;
        pub fn load_by_name_with_config(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
        ) -> i32;
    }
}
