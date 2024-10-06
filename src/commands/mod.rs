use clap_derive::{Parser, Subcommand};

pub mod commit;
pub mod log;
pub mod branch;
pub mod checkout;
pub mod current_branch;
pub mod status;
pub mod reset;
pub mod merge;
pub mod compare;
pub mod init;
pub mod diff;

#[derive(Parser)]
#[command(name = "Image Versioner")]
#[command(about = "Sistema de control de versiones para imágenes", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Add {
        #[arg(help = "Ruta de la imagen a agregar")]
        path: String,
    },
    Compare {
        #[arg(help = "Primera imagen para comparar")]
        image1: String,
        #[arg(help = "Segunda imagen para comparar")]
        image2: String,
    },
    Commit {
        #[arg(help = "Ruta de la imagen a versionar")]
        path: String,
        #[arg(help = "Mensaje del commit")]
        message: String,
    },
    Log,
    Branch {
        #[arg(help = "Nombre de la rama (si se proporciona)")]
        name: Option<String>,
    },
    Checkout {
        #[arg(help = "Nombre de la rama a la cual cambiar")]
        branch: String,
    },
    CurrentBranch,
    Merge {
        #[arg(help = "Nombre de la rama a fusionar con la actual")]
        source_branch: String,
    },
    Status,
    Reset {
        #[arg(help = "Hash del commit a eliminar")]
        commit_hash: String,
    },
    Diff {
        #[arg(help = "Primera imagen para comparar")]
        image1: String,
        #[arg(help = "Segunda imagen para comparar")]
        image2: String,
        #[arg(help = "Ruta de la imagen de salida")]
        output: String,
    }
}