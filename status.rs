#![feature(termination_trait_lib)]

pub struct Status(pub i32, pub Option<String>);

impl std::process::Termination for Status {
    fn report(self) -> i32 {
        if let Some(message) = self.1 {
            eprintln!("{}", message);
        }
        self.0
    }
}

#[macro_export]
macro_rules! status {
    ($code:expr) => {
        $crate::Status(
            $code,
            std::option::Option::None,
        )
    };
    ($code:expr, $fmt:expr) => {
        $crate::Status(
            $code,
            std::option::Option::Some(
                std::string::String::from($fmt),
            ),
        )
    };
    ($code:expr, $fmt:literal $(, $arg:expr)+$(,)?) => {
        $crate::Status(
            $code,
            std::option::Option::Some(
                format!($fmt $(, $arg)*),
            ),
        )
    };
}
