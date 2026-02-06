use apollo_core::prelude::*;
use apollo_lazy::prelude::IntoLazy;
use apollo_plan::dsl::BaseColumnUdf;
use apollo_plan::prelude::UserDefinedFunction;
use apollo_sql::SQLContext;
use apollo_sql::function_registry::FunctionRegistry;

struct MyFunctionRegistry {
    functions: PlHashMap<String, UserDefinedFunction>,
}

impl MyFunctionRegistry {
    fn new(funcs: Vec<UserDefinedFunction>) -> Self {
        let functions = funcs.into_iter().map(|f| (f.name.to_string(), f)).collect();
        MyFunctionRegistry { functions }
    }
}

impl FunctionRegistry for MyFunctionRegistry {
    fn register(&mut self, name: &str, fun: UserDefinedFunction) -> ApolloResult<()> {
        self.functions.insert(name.to_string(), fun);
        Ok(())
    }

    fn get_udf(&self, name: &str) -> ApolloResult<Option<UserDefinedFunction>> {
        Ok(self.functions.get(name).cloned())
    }

    fn contains(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
}

#[test]
fn test_udfs() -> ApolloResult<()> {
    let my_custom_sum = UserDefinedFunction::new(
        "my_custom_sum".into(),
        BaseColumnUdf::new(
            move |c: &mut [Column]| {
                let first = c[0].as_materialized_series().clone();
                let second = c[1].as_materialized_series().clone();
                (first + second).map(Column::from)
            },
            |_: &Schema, fs: &[Field]| {
                // UDF is responsible for schema validation
                apollo_ensure!(fs.len() == 2, SchemaMismatch: "expected two arguments");
                let first = &fs[0];
                let second = &fs[1];

                if first.dtype() != second.dtype() {
                    apollo_bail!(SchemaMismatch: "mismatched types")
                }
                Ok(first.clone())
            },
        ),
    );

    let mut ctx = SQLContext::new()
        .with_function_registry(Arc::new(MyFunctionRegistry::new(vec![my_custom_sum])));

    let df = df! {
        "a" => &[1, 2, 3],
        "b" => &[1, 2, 3],
        "c" => &["a", "b", "c"]
    }
    .unwrap()
    .lazy();

    ctx.register("foo", df);
    let res = ctx.execute("SELECT a, b, my_custom_sum(a, b) FROM foo");
    assert!(res.is_ok());

    // schema is invalid so it will fail
    assert!(matches!(
        ctx.execute("SELECT a, b, my_custom_sum(c) as invalid FROM foo"),
        Err(ApolloError::SchemaMismatch(_))
    ));

    // create a new UDF to be registered on the context
    let my_custom_divide = UserDefinedFunction::new(
        "my_custom_divide".into(),
        BaseColumnUdf::new(
            move |c: &mut [Column]| {
                let first = c[0].as_materialized_series().clone();
                let second = c[1].as_materialized_series().clone();
                (first / second).map(Column::from)
            },
            |_: &Schema, fs: &[Field]| {
                // UDF is responsible for schema validation
                apollo_ensure!(fs.len() == 2, SchemaMismatch: "expected two arguments");
                let first = &fs[0];
                let second = &fs[1];

                if first.dtype() != second.dtype() {
                    apollo_bail!(SchemaMismatch: "mismatched types")
                }
                Ok(first.clone())
            },
        ),
    );

    // register a new UDF on an existing context
    ctx.registry_mut().register("my_div", my_custom_divide)?;

    // execute the query
    let res = ctx
        .execute("SELECT a, b, my_div(a, b) as my_div FROM foo")?
        .collect()?;
    let expected = df! {
        "a" => &[1, 2, 3],
        "b" => &[1, 2, 3],
        "my_div" => &[1, 1, 1]
    }?;
    assert!(expected.equals_missing(&res));

    Ok(())
}
