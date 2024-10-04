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
}