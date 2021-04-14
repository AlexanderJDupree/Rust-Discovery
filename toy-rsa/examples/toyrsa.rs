//! CLI example program capable of generating RSA keys and encrypting/decryption
//! of messages.
//!
//! Alexander DuPree 2021
use clap::{AppSettings, Clap};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::{Path, PathBuf};
use toy_rsa::*;

/// toyrsa supports generating 64bit RSA keys and encrypting/decrypting messages
/// WARNING! Do not use this for actual crytographic activites!
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Alexander DuPree <adupree@pdx.edu>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Genkey(Genkey),
    Encrypt(Encrypt),
    Decrypt(Decrypt),
}

/// Generate a public/private key pair
#[derive(Clap)]
struct Genkey {
    /// File prefix name
    #[clap(short, long, default_value = "toyrsa")]
    file_prefix: String,

    /// output file location, outputs <prefix>.prv and <prefix>.pub files
    #[clap(short, long, default_value = "./")]
    output_path: PathBuf,
}

/// Encrypt a numeric message with a public key
#[derive(Clap)]
struct Encrypt {
    /// Location of public key file
    public_key_file: PathBuf,

    /// Numeric message to encrypt
    message: u32,
}

/// Decrypt a numeric message with a private key
#[derive(Clap)]
struct Decrypt {
    /// Location of private key file
    private_key_file: PathBuf,

    /// Numeric message to decrypt
    message: u64,
}

/// Print a usage error message and exit
fn main() {
    let args = Args::parse();

    match args.subcmd {
        SubCommand::Genkey(cmd) => {
            generate_key_pair(&cmd.output_path, &cmd.file_prefix)
                .expect("Failed to write key files");
        }
        SubCommand::Encrypt(cmd) => {
            encrypt_message(&cmd.public_key_file, cmd.message).expect("Failed to encrypt message");
        }
        SubCommand::Decrypt(cmd) => {
            decrypt_message(&cmd.private_key_file, cmd.message).expect("Failed to decrypt message");
        }
    }
}

fn generate_key_pair(path: &Path, file_prefix: &str) -> Result<(), Error> {
    let private_key = genkey();
    let public_key = (private_key.0 as u64) * (private_key.1 as u64);

    let pub_path = path.join(format!("{}.pub", file_prefix));
    let prv_path = path.join(format!("{}.prv", file_prefix));

    let mut pub_file = File::create(pub_path)?;
    let mut prv_file = File::create(prv_path)?;

    write!(pub_file, "{}", public_key)?;
    write!(prv_file, "{}\n{}", private_key.0, private_key.1)?;

    Ok(())
}

fn encrypt_message(public_key_path: &Path, msg: u32) -> Result<(), Error> {
    let mut buffer = String::new();

    let public_key_file = File::open(public_key_path)?;

    BufReader::new(public_key_file)
        .read_line(&mut buffer)
        .expect("Failed to read public key file");

    let public_key: u64 = buffer
        .parse()
        .expect("Failed to parse public key from file");

    let encrypted_msg = encrypt(public_key, msg);

    println!("{}", encrypted_msg);

    Ok(())
}

fn decrypt_message(private_key_path: &Path, encrypted_msg: u64) -> Result<(), Error> {
    let private_key_file = File::open(private_key_path)?;

    let key: Vec<u32> = BufReader::new(private_key_file)
        .lines()
        .take(2)
        .map(|s| {
            s.expect("Failed to read private key file")
                .parse()
                .expect("Failed to parse private key from file")
        })
        .collect();

    let msg = decrypt((key[0], key[1]), encrypted_msg);

    println!("{}", msg);

    Ok(())
}
