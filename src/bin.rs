use clap::Parser;
use muisnow_upload_lib::{upload, sign_worker::{get_public_key, get_public_key_fingerprint}, config::{CONFIG, Config}};

#[derive(Parser, Debug, Default)]
#[command(author = "Muisnow", version, about = "This is a program makes you upload assets into muisnowdevs.one")]
struct Args {
    #[arg(help = "Files to upload")]
    file: Vec<String>,

    #[arg(short, long, help = "To disable detailed processing output")]
    verbose: bool,

    #[arg(long, help = "Show up public key")]
    public_key: bool,

    #[arg(long, help = "Show up public key fingerprint")]
    fingerprint: bool
}

fn show_public_key() {
    let key = get_public_key().expect("Cannot get public key");
    println!("{}", key);
}

fn show_public_key_fingerprint() {
    let fingerprint = get_public_key_fingerprint().expect("Cannot get fingerprint");
    println!("{}", fingerprint);
}

fn set_config(config: &Args) {
    *CONFIG.lock().unwrap() = Some(Config {
        invisible: !config.verbose
    });
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.public_key {
        show_public_key()
    }

    if args.fingerprint {
        show_public_key_fingerprint()
    }

    set_config(&args);

    for file in args.file {
        match upload(&file).await {
            Ok(_) => continue,
            Err(_) => println!("Cannot upload {} to server", file)
        };
    }
}