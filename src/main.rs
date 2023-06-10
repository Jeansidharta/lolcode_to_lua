use clap::{Parser, Subcommand};
use lolcode_to_lua::ToLua;
use std::{path::PathBuf, process::exit};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Ast {
        filename: PathBuf,
    },
    Lua {
        filename: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Ast { filename } => {
            let ast = lolcode_to_lua::make_ast_from_file(filename);
            println!("{:#?}", ast);
        }
        Commands::Lua { filename } => {
            let ast = lolcode_to_lua::make_ast_from_file(filename);
            let lua = match ast {
                Ok(ast) => ast.into_lua(),
                Err(err) => {
                    println!("Err: {}", err);
                    exit(-1)
                }
            };
            println!("{}", lua);
        }
    };
}
