use async_std::{
    io::{stdin, BufReader},
    net::TcpStream,
    prelude::*,
    task,
};

use local_ip_address::local_ip;
use colored::Colorize;
use clap::{command, Arg};
use futures::{select, FutureExt};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn logo() {

    let img = &r#"

                    ⠀⠀⠀⠀⠀⠀⠀⣠⡤⠶⡄⠀⠀⠀⠀⠀⠀⠀⢠⠶⣦⣀⠀⠀⠀⠀⠀⠀⠀
                    ⠀⠀⠀⠀⢀⣴⣿⡟⠀⠈⣀⣾⣝⣯⣿⣛⣷⣦⡀⠀⠈⢿⣿⣦⡀⠀⠀⠀⠀
                    ⠀⠀⠀⣴⣿⣿⣿⡇⠀⢼⣿⣽⣿⢻⣿⣻⣿⣟⣷⡄⠀⢸⣿⣿⣾⣄⠀⠀⠀
                    ⠀⠀⣞⣿⣿⣿⣿⣷⣤⣸⣟⣿⣿⣻⣯⣿⣿⣿⣿⣀⣴⣿⣿⣿⣿⣯⣆⠀⠀
                    ⠀⡼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣜⡆⠀
                    ⢠⣟⣯⣿⣿⣿⣷⢿⣫⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣬⣟⠿⣿⣿⣿⣿⡷⣾⠀
                    ⢸⣯⣿⣿⡏⠙⡇⣾⣟⣿⡿⢿⣿⣿⣿⣿⣿⢿⣟⡿⣿⠀⡟⠉⢹⣿⣿⢿⡄
                    ⢸⣯⡿⢿⠀⠀⠱⢈⣿⢿⣿⡿⣏⣿⣿⣿⣿⣿⣿⣿⣿⣀⠃⠀⢸⡿⣿⣿⡇
                    ⢸⣿⣇⠈⢃⣴⠟⠛⢉⣸⣇⣹⣿⣿⠚⡿⣿⣉⣿⠃⠈⠙⢻⡄⠎⠀⣿⡷⠃
                    ⠈⡇⣿⠀⠀⠻⣤⠠⣿⠉⢻⡟⢷⣝⣷⠉⣿⢿⡻⣃⢀⢤⢀⡏⠀⢠⡏⡼⠀
                    ⠀⠘⠘⡅⠀⣔⠚⢀⣉⣻⡾⢡⡾⣻⣧⡾⢃⣈⣳⢧⡘⠤⠞⠁⠀⡼⠁⠀⠀
                    ⠀⠀⠀⠸⡀⠀⢠⡎⣝⠉⢰⠾⠿⢯⡘⢧⡧⠄⠀⡄⢻⠀⠀⠀⢰⠁⠀⠀⠀
                    ⠀⠀⠀⠀⠁⠀⠈⢧⣈⠀⠘⢦⠀⣀⠇⣼⠃⠰⣄⣡⠞⠀⠀⠀⠀⠀⠀⠀⠀
                    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⢤⠼⠁⠀⠀⠳⣤⡼⠀⠀⠀⠀⠀⠀⠀⠀
                        
    "#.green().bold();

    println!("{}", img);
}

async fn client(ip: &str, port: &str, username: &str, password: &str) -> Result<()> {

    let local_ip_address: std::net::IpAddr = local_ip().unwrap();
        
    match TcpStream::connect(format!("{}:{}", ip, port)).await {
        Ok(sucess) => { 
            
            println!("[{}] Connected!\n", "✔".white().on_green().bold());    
            let (reader, mut writer) = (&sucess, &sucess); 
            
            writer.write_all(username.as_bytes()).await?;
            writer.write_all(b"\n").await?; 

            if !password.is_empty() {
                writer.write_all(password.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
            
            let mut lines_from_server = BufReader::new(reader).lines();
            let mut lines_from_stdin = BufReader::new(stdin()).lines(); 

            loop {
                select! { 
                    line = lines_from_server.next().fuse() => match line {
                        Some(line) => {
                            let line = line?;
                            println!("{}", line);
                        },
                        
                        None => break,
                    },

                    line = lines_from_stdin.next().fuse() => match line {
                        Some(line) => {

                            let message = line?.clone();

                            if !message.is_empty() {
                                let formatted_line = format!(
                                    "[{}] > {}", 
                                    format!("{}{}{}", username, "@".white().bold(), local_ip_address).truecolor(12, 254, 47),
                                    message
                                );
                                writer.write_all(formatted_line.as_bytes()).await?;
                                writer.write_all(b"\n").await?;
                            }
                        }

                        None => break,
                    }
                }
            }
        },
        Err(e) => println!("[{}] Connection failure: {}", "X".white().on_red().bold(), e),
    };

    Ok(())
}

fn main() -> Result<()> {
    
    let arguments = command!()
        .about("[!] Lovecraft Chat for casual use")
        .arg(Arg::new("User name").help("Create Username").short('u').long("username").default_value("User"))
        .arg(Arg::new("Server Password").help("Enter Server Password").short('P').long("password").default_value(""))
        .arg(Arg::new("Server IP").help("Server IP").short('i').long("ip").required(true))
        .arg(Arg::new("Server Port").help("Server Port").short('p').long("port").default_value("6060"))
        .get_matches();
        
    let ip = arguments.get_one::<String>("Server IP").expect("IP is required");
    let port = arguments.get_one::<String>("Server Port").unwrap();
    let server_password = arguments.get_one::<String>("Server Password").unwrap();
    let username = arguments.get_one::<String>("User name").unwrap();

    logo();
    let _ = task::block_on(client(ip, port, username, server_password));

    Ok(())
}
