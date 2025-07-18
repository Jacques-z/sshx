use std::process::ExitCode;

use ansi_term::Color::{Cyan, Fixed, Green, Red};
use anyhow::Result;
use clap::Parser;
use sshx::{controller::Controller, runner::Runner, terminal::get_default_shell};
use tokio::signal;
use tracing::error;

/// A secure web-based, collaborative terminal.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Address of the remote sshx server.
    #[clap(long, env = "SSHX_SERVER")]
    server: String,

    /// Local shell command to run in the terminal.
    #[clap(long)]
    shell: Option<String>,

    /// Quiet mode, only prints the URL to stdout.
    #[clap(short, long)]
    quiet: bool,

    /// Session name displayed in the title (defaults to user@hostname).
    #[clap(long)]
    name: Option<String>,

    /// Sub path to use
    #[clap(long, default_value = "")]
    addr: String,

    /// Encryption key
    #[clap(long, default_value = "")]
    encryption_key: String,

    /// Enable everyone to run arbitrary commands.
    #[clap(long)]
    no_write_protection: bool,

    /// Write permission key
    #[clap(long, default_value = "")]
    write_password: String,
}

fn print_greeting(shell: &str, controller: &Controller) {
    let version_str = match option_env!("CARGO_PKG_VERSION") {
        Some(version) => format!("v{version}"),
        None => String::from("[dev]"),
    };
    if let Some(write_password) = controller.write_password() {
        println!(
            r#"
  {sshx} {version}

  {arr}  link:            {link}
  {arr}  Read-only link:  {link_v}
  {arr}  Safe write link: {link_s}
  {arr}  Writable link:   {link_e} ⚠️ INSECURE
  {arr}  Shell:           {shell_v}

  {eye}  View:           {encryption_key}
  {key}  Key:            {write_password}
"#,
            sshx = Green.bold().paint("sshx"),
            version = Green.paint(&version_str),
            arr = Green.paint("➜"),
            eye = "👀",
            key = "🔑",
            link = Cyan.underline().paint(controller.link()),
            link_v = Cyan.underline().paint(controller.url()),
            link_s = Cyan
                .underline()
                .paint(controller.url().to_owned() + ",manually"),
            encryption_key = Cyan.underline().paint(controller.encryption_key()),
            write_password = Cyan.underline().paint(write_password),
            link_e = Red
                .underline()
                .paint(controller.url().to_owned() + "," + write_password),
            shell_v = Fixed(8).paint(shell),
        );
    } else {
        println!(
            r#"
  {sshx} {version}

  {arr}  Link:            {link}
  {arr}  Link with auth:  {link_v}
  {arr}  Shell:           {shell_v}

  {key}  Key:             {encryption_key}
"#,
            sshx = Green.bold().paint("sshx"),
            version = Green.paint(&version_str),
            arr = Green.paint("➜"),
            key = "🔑",
            link = Cyan.underline().paint(controller.link()),
            link_v = Cyan.underline().paint(controller.url()),
            encryption_key = Cyan.underline().paint(controller.encryption_key()),
            shell_v = Fixed(8).paint(shell),
        );
    }
}

#[tokio::main]
async fn start(args: Args) -> Result<()> {
    let shell = match args.shell {
        Some(shell) => shell,
        None => get_default_shell().await,
    };

    let name = args.name.unwrap_or_else(|| {
        let mut name = whoami::username();
        if let Ok(host) = whoami::fallible::hostname() {
            // Trim domain information like .lan or .local
            let host = host.split('.').next().unwrap_or(&host);
            name += "@";
            name += host;
        }
        name
    });

    let runner = Runner::Shell(shell.clone());
    let mut controller = Controller::new(
        &args.server,
        &name,
        &args.addr,
        &args.encryption_key,
        runner,
        !args.no_write_protection,
        &args.write_password,
    )
    .await?;
    if args.quiet {
        if let Some(write_password) = controller.write_password() {
            println!("{}", controller.url().to_owned() + "," + write_password);
        } else {
            println!("{}", controller.url());
        }
    } else {
        print_greeting(&shell, &controller);
    }

    let exit_signal = signal::ctrl_c();
    tokio::pin!(exit_signal);
    tokio::select! {
        _ = controller.run() => unreachable!(),
        Ok(()) = &mut exit_signal => (),
    };
    controller.close().await?;

    Ok(())
}

fn main() -> ExitCode {
    let args = Args::parse();

    let default_level = if args.quiet { "error" } else { "info" };

    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or(default_level.into()))
        .with_writer(std::io::stderr)
        .init();

    match start(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            error!("{err:?}");
            ExitCode::FAILURE
        }
    }
}
