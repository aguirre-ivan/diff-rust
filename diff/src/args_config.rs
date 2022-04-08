use std::process;
/// ! Args config to use in file_handler and diff
/// ! It contains file's names.
pub struct ArgsConfig {
    pub filename1: String,
    pub filename2: String,
}

impl ArgsConfig {
    /// Reutrns struct ArgConfig
    /// args: &[String]: string vec with 2 filenames
    pub fn new(args: &[String]) -> ArgsConfig {
        let len = args.len();

        if len != 3 {
            println!("ArgsError: Diff takes exactly 2 arguments ({} given)", len);
            process::exit(1);
        }

        ArgsConfig {
            filename1: args[1].clone(),
            filename2: args[2].clone(),
        }
    }
}
