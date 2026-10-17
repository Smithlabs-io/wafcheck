pub mod api;
pub mod cli;
pub mod error;
pub mod commands;




pub fn run(command: cli::Command) -> Result<String, error::AppError> {
    match command {
            cli::Command::Drift { file_path: _ } => {
                println!("Running Drift command...");
                Ok("Drift command executed successfully.".to_string())
                // Implement the logic for Drift command here
            },
            cli::Command::TestKey {}=> {
                println!("Running TestKey command...");
                let result = commands::test::test_key()?;
                Ok(result)
            },
        }
}


