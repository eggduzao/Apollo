use parking_lot::RwLock;

type WarningFunction = fn(&str, ApolloWarning);
static WARNING_FUNCTION: RwLock<WarningFunction> = RwLock::new(eprintln);

fn eprintln(fmt: &str, warning: ApolloWarning) {
    eprintln!("{warning:?}: {fmt}");
}

/// Set the function that will be called by the `apollo_warn!` macro.
/// You can use this to set logging in apollo.
pub fn set_warning_function(function: WarningFunction) {
    *WARNING_FUNCTION.write() = function;
}

pub fn get_warning_function() -> WarningFunction {
    *WARNING_FUNCTION.read()
}

#[derive(Debug)]
pub enum ApolloWarning {
    Deprecation,
    UserWarning,
    CategoricalRemappingWarning,
    MapWithoutReturnDtypeWarning,
}

#[macro_export]
macro_rules! apollo_warn {
    ($variant:ident, $fmt:literal $(, $arg:expr)* $(,)?) => {
        {{
        let func = $crate::get_warning_function();
        let warn = $crate::ApolloWarning::$variant;
        func(format!($fmt, $($arg)*).as_ref(), warn)
        }}
    };
    ($fmt:literal $(, $arg:expr)*) => {
        {{
        let func = $crate::get_warning_function();
        func(format!($fmt, $($arg),*).as_ref(), $crate::ApolloWarning::UserWarning)
        }}
    };
}
