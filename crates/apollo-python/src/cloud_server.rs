use apollo_core::error::{ApolloResult, apollo_err};
use apollo_expr::state::ExecutionState;
use apollo_mem_engine::create_physical_plan;
use apollo_plan::plans::{AExpr, IR, IRPlan};
use apollo_plan::prelude::{Arena, Node};
use apollo_utils::pl_serialize;
use pyo3::intern;
use pyo3::prelude::{PyAnyMethods, PyModule, Python, *};
use pyo3::types::IntoPyDict;

use crate::PyDataFrame;
use crate::error::PyApolloErr;
use crate::lazyframe::visit::NodeTraverser;
use crate::utils::EnterApolloExt;

/// Take a serialized `IRPlan` and execute it on the GPU engine.
///
/// This is done as a Python function because the `NodeTraverser` class created for this purpose
/// must exactly match the one expected by the `cudf_apollo` package.
#[pyfunction]
pub fn _execute_ir_plan_with_gpu(ir_plan_ser: Vec<u8>, py: Python) -> PyResult<PyDataFrame> {
    // Deserialize into IRPlan.
    let mut ir_plan: IRPlan =
        pl_serialize::deserialize_from_reader::<_, _, false>(ir_plan_ser.as_slice())
            .map_err(PyApolloErr::from)?;

    // Edit for use with GPU engine.
    gpu_post_opt(
        py,
        ir_plan.lp_top,
        &mut ir_plan.lp_arena,
        &mut ir_plan.expr_arena,
    )
    .map_err(PyApolloErr::from)?;

    // Convert to physical plan.
    let mut physical_plan = create_physical_plan(
        ir_plan.lp_top,
        &mut ir_plan.lp_arena,
        &mut ir_plan.expr_arena,
        None,
    )
    .map_err(PyApolloErr::from)?;

    // Execute the plan.
    let mut state = ExecutionState::new();
    py.enter_apollo_df(|| physical_plan.execute(&mut state))
}

/// Prepare the IR for execution by the Apollo GPU engine.
fn gpu_post_opt(
    py: Python<'_>,
    root: Node,
    lp_arena: &mut Arena<IR>,
    expr_arena: &mut Arena<AExpr>,
) -> ApolloResult<()> {
    // Get cuDF Python function.
    let cudf = PyModule::import(py, intern!(py, "cudf_apollo")).unwrap();
    let lambda = cudf.getattr(intern!(py, "execute_with_cudf")).unwrap();

    // Define cuDF config.
    let apollo = PyModule::import(py, intern!(py, "apollo")).unwrap();
    let engine = apollo.getattr(intern!(py, "GPUEngine")).unwrap();
    let kwargs = [("raise_on_fail", true)].into_py_dict(py).unwrap();
    let engine = engine.call((), Some(&kwargs)).unwrap();

    // Define node traverser.
    let nt = NodeTraverser::new(root, std::mem::take(lp_arena), std::mem::take(expr_arena));

    // Get a copy of the arenas.
    let arenas = nt.get_arenas();

    // Pass the node visitor which allows the Python callback to replace parts of the query plan.
    // Remove "cuda" or specify better once we have multiple post-opt callbacks.
    let kwargs = [("config", engine)].into_py_dict(py).unwrap();
    lambda
        .call((nt,), Some(&kwargs))
        .map_err(|e| apollo_err!(ComputeError: "'cuda' conversion failed: {}", e))?;

    // Unpack the arena's.
    // At this point the `nt` is useless.
    std::mem::swap(lp_arena, &mut *arenas.0.lock().unwrap());
    std::mem::swap(expr_arena, &mut *arenas.1.lock().unwrap());

    Ok(())
}
