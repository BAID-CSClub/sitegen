#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        print!("{}", "INFO: ".green().bold());
        println!($($arg)*);
    }
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        print!("{}", "WARN: ".yellow().bold());
        println!($($arg)*);
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        print!("{}", "ERRO: ".red().bold());
        println!($($arg)*);
    }
}