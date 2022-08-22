#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        use colorful::Colorful;
        print!("{}", "INFO: ".green().bold());
        println!($($arg)*);
    }
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        use colorful::Colorful;
        print!("{}", "WARN: ".yellow().bold());
        println!($($arg)*);
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        use colorful::Colorful;
        print!("{}", "ERRO: ".red().bold());
        println!($($arg)*);
    }
}
