// For now, just an empty stub.

#[macro_export]
macro_rules! html {
    ($($tt:tt)*) => {
        compile_error!("html! macro not implemented yet");
    };
}
