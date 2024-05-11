use std::{collections::HashSet, net::SocketAddr, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::{broadcast, Mutex},
};

use colored::Colorize;
use clap::{command, Arg};

fn logo() -> &'static str {
    r#"
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣴⣶⣶⣤⣤⣤⣶⣤⡄⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣿⣿⡿⠟⠛⠛⠛⠿⣿⣿⣧⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠃⠀⠀⠀⠀⠀⠀⠘⣿⣿⣷⣶⡄
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣯⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠛⠁
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢿⣿⣦⡀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣤⣄
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠿⠿⠿⠿⠿⠋⠀⠀⢸⣿⣿⣿⣿ ,
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⠋⠉⠁
        ⠀⠀⠀⠀⣠⣶⣶⣶⣶⣶⣶⣤⣄⣀⣀⣀⣀⣤⣶⣿⣿⣿⡀⠀⠀
        ⠀⠀⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀
        ⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠿⢿⣿⣿⣿⠉⠀⠀⠉⠉⠀⠀⠀
        ⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡄⠀⠸⠿⠿⠿⠀⠀⠀⠀⠀⠀⠀⠀
        ⢠⣿⣿⣿⣿⣿⡿⠛⢿⣿⡿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠸⠿⠿⠿⠿⠿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"#
}

async fn handle_client(
    
    mut socket: tokio::net::TcpStream,
    addr: SocketAddr,
    tx: broadcast::Sender<(String, SocketAddr)>,
    mut rx: broadcast::Receiver<(String, SocketAddr)>,
    users: String,
    server_messages: Arc<Mutex<Vec<String>>>, 
) {
    let (mut read, mut write) = socket.split();
    let mut read = BufReader::new(&mut read);
    let mut text: String = String::new();

    {
        let server_messages_guard = server_messages.lock().await;
        for message in &*server_messages_guard {
            write.write_all(message.as_bytes()).await.unwrap();
        }
    }

    loop {
        tokio::select! {
            result = read.read_line(&mut text) => {
                if result.unwrap() == 0 {
                    let disconnected_msg = format!("\n{} User {} disconnected!\n\n", "[+]".green().bold(), users.trim());
                    println!("{}", &disconnected_msg);
                    tx.send((disconnected_msg, addr)).unwrap();
                    break;
                }

                tx.send((text.to_string(), addr)).unwrap();
                println!("{}", text);

                let mut server_messages_guard = server_messages.lock().await;
                server_messages_guard.push(text.clone());
                text.clear();
            }

            result = rx.recv() => {
                let (msg, other_addr) = result.unwrap();
                
                if addr != other_addr {
                    write.write_all(msg.as_bytes()).await.unwrap();
                }
            }
        }
    }
}

async fn accept_clients(
    
    listener: TcpListener,
    tx: broadcast::Sender<(String, SocketAddr)>,
    rx: broadcast::Receiver<(String, SocketAddr)>,
    connected_users: Mutex<HashSet<String>>,
    server_messages: Arc<Mutex<Vec<String>>>,
    server_password: &str
) {
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let mut connected_users_guard = connected_users.lock().await;

        let mut name: String = String::new(); 
        let mut password: String = String::new();
        let mut read: BufReader<&mut tokio::net::TcpStream> = BufReader::new(&mut socket);

        read.read_line(&mut name).await.unwrap();
        read.read_line(&mut password).await.unwrap();

        if password.trim() != server_password {
            let _ = socket
                .write_all(
                    format!(
                        "[{}] The server password is incorrect ...\n",
                        "X".red().bold()
                    )
                    .as_bytes(),
                )
                .await;
            continue;
        }

        let new_con_msg: String = format!("\n[{}] New connection from {:?}\n\n", "✔".white().on_bright_green().bold(), name.trim());

        println!("{}", new_con_msg); 
        tx.send((new_con_msg, addr)).unwrap();

        if connected_users_guard.contains(&name) {
            let _ = socket
                .write_all(
                    format!(
                        "[{}] User with this name is already connected. Disconnecting...\n",
                        "X".red().bold()
                    )
                    .as_bytes(),
                )
                .await;
            continue;
        }

        connected_users_guard.insert(name.clone()); 
        let tx_clone: broadcast::Sender<(String, SocketAddr)> = tx.clone();
        let rx_clone: broadcast::Receiver<(String, SocketAddr)> = rx.resubscribe();

        let server_messages_clone = server_messages.clone();
        tokio::spawn(async move {
            handle_client(
                socket,
                addr,
                tx_clone,
                rx_clone,
                name,
                server_messages_clone,
            )
            .await;
        });
    }
}

async fn server(ip: &str, port: &str, password: &str) {

    let listener: TcpListener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();
    let (tx, _) = broadcast::channel(10);
    let connected_users: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    let server_messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let rx: broadcast::Receiver<(String, SocketAddr)> = tx.subscribe();

    accept_clients(listener, tx, rx, connected_users, server_messages, password).await;
}

#[tokio::main]
async fn main() {

    let arguments = command!()
        .about("[!] Lovecraft Chat for casual use")
        .arg(Arg::new("Server IP").default_value("localhost").help("Server IP to listen to connections"))
        .arg(Arg::new("Server Port").help("Server Port").default_value("6060"))
        .arg(Arg::new("Password").long("password").short('p').help("[Optional] Server Password"))
        .get_matches();

    let ip: &String = arguments.get_one::<String>("Server IP").expect("IP is required");
    let port: &String = arguments.get_one::<String>("Server Port").unwrap();
    let password: &String = arguments.get_one::<String>("Password").unwrap();

    println!("{}", logo().green().bold());
    server(ip, port, password).await;
}
