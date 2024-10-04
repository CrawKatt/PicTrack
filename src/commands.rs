use clap_derive::Subcommand;

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
    }
}