use clap::App;
use clap::Command;

use super::args::ArgBuilder;

pub trait CommandBuilder: ArgBuilder {

    fn append_params_command(app: App) -> App {
        let command = Command::new("params")
            .arg(Self::zkwasm_k_arg().required(true));
        app.subcommand(command)
    }

    fn append_setup_subcommand(app: App) -> App {
        let command = Command::new("setup")
        .arg(Self::output_path_arg())
        .arg(Self::zkwasm_k_arg());
        app.subcommand(command)
    }

    fn append_create_aggregate_proof_subcommand(app: App) -> App {
        let command = Command::new("batch")
            .arg(Self::zkwasm_k_arg())
            .arg(Self::proof_name_arg())
            .arg(Self::proof_load_info_arg());
        app.subcommand(command)
    }

    fn append_verify_aggregate_verify_subcommand(app: App) -> App {
        let command = Command::new("verify")
            .arg(Self::zkwasm_k_arg())
            .arg(Self::proof_load_info_arg());

        app.subcommand(command)
    }

    fn append_generate_solidity_verifier(app: App) -> App {
        let command = Command::new("solidity")
            .arg(Self::zkwasm_k_arg())
            .arg(Self::sol_dir_arg())
            .arg(Self::proof_load_info_arg());
        app.subcommand(command)
    }
}
