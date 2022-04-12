//! Args config to use in file_handler and diff
//!
//! It contains file's names.

pub struct ArgsConfig {
    pub filename1: String,
    pub filename2: String,
}

impl ArgsConfig {
    /// Reutrns struct ArgConfig
    ///
    /// args: &[String]: Is string vec with 2 filenames
    pub fn new(args: &[String]) -> Result<ArgsConfig, String> {
        let len = args.len();

        if len != 3 {
            return Err(format!(
                "ArgsError: Diff takes exactly 2 arguments ({} given)",
                len - 1
            ));
        }

        Ok(ArgsConfig {
            filename1: args[1].clone(),
            filename2: args[2].clone(),
        })
    }
}
