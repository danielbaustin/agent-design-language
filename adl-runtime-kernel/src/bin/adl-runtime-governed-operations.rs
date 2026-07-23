#[path = "../governed_operations.rs"]
mod governed_operations;

use std::process::ExitCode;

use governed_operations::{execute_many, GovernedCommand, RuntimeConfig};

#[tokio::main]
async fn main() -> ExitCode {
    let config = match RuntimeConfig::from_env() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(78);
        }
    };
    let (commands, batch) = match read_commands() {
        Ok(commands) => commands,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(64);
        }
    };
    let outcomes = execute_many(config, commands).await;
    let payload = if batch {
        serde_json::to_value(&outcomes)
    } else {
        serde_json::to_value(&outcomes[0])
    };
    match payload.and_then(|value| serde_json::to_string(&value)) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("outcome encoding failed: {error}");
            return ExitCode::from(70);
        }
    }
    if outcomes.iter().all(|outcome| outcome.status == "completed") {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(77)
    }
}

fn read_commands() -> Result<(Vec<GovernedCommand>, bool), String> {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin()
        .take(1_048_577)
        .read_to_string(&mut input)
        .map_err(|_| "command_read_failed".to_owned())?;
    if input.is_empty() || input.len() > 1_048_576 {
        return Err("command_size_invalid".to_owned());
    }
    let value: serde_json::Value =
        serde_json::from_str(&input).map_err(|_| "command_invalid".to_owned())?;
    let batch = value.is_array();
    let commands = if batch {
        serde_json::from_value(value)
    } else {
        serde_json::from_value(value).map(|command| vec![command])
    }
    .map_err(|_| "command_invalid".to_owned())?;
    if commands.is_empty() || commands.len() > 3 {
        return Err("command_batch_size_invalid".to_owned());
    }
    Ok((commands, batch))
}
