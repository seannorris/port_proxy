use std::io::{Read, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{env, io, net};
use std::fs::File;
use std::collections::HashSet;
use std::net::{Shutdown, TcpStream};

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        eprintln!("Argument(s) not provided: <src port> <dst port>");
        return;
    }

    let pid = std::process::id();
    let mut file = File::create(format!("{}\\port_proxy_{}_{}.lock", std::env::var("TEMP").expect("failed to read TEMP var."), args[1], args[2])).expect("failed to open lock file.");
    file.write_all(format!("{}", pid).as_ref()).expect("Failed to write to lock file.");
    drop(file);

    println!("Source: {}, Destination: {}", args[1], args[2]);
    let src_port = &args[1];
    let dst_listener = net::TcpListener::bind(format!("0.0.0.0:{}", args[2]))
        .expect("failed to bind to dst port.");
    dst_listener
        .set_nonblocking(true)
        .expect("Failed to set non-blocking for dst port.");

    let mut connections: Vec<Connection> = Vec::new();
    let mut remove_set: std::collections::HashSet<usize> = HashSet::new();

    let mut last_write = Instant::now();
    let mut blocked_all: bool;
    loop
    {
        blocked_all = true;

        //Check each connection.
        for index in 0..connections.len()
        {
            blocked_all &= check_conn(&mut connections, index, false, &mut remove_set);
            blocked_all &= check_conn(&mut connections, index, true, &mut remove_set);
        }

        //Check for new connections.
        let mut drop_all = false;
        match dst_listener.accept()
        {
            Ok(stream) => {
                match get_conn(src_port)
                {
                    Ok(src_stream) => {
                        connections.push(Connection {
                            src_stream,
                            dst_stream: stream.0,
                        });
                        println!("Got a connection!");
                    }
                    Err(_e) => drop_all = true
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => blocked_all &= true,
            Err(_e) => eprintln!("Failed to accept connection.")
        };

        //Drop old connections.
        if remove_set.len() > 0
        {
            let mut new_connections: Vec<Connection> = Vec::new();
            for index in 0..connections.len()
            {
                let connection = connections.remove(0);
                if drop_all || remove_set.contains(&index)
                {
                    println!("Dropped a connection!");
                    connection.dst_stream.shutdown(Shutdown::Both).expect("Failed to close dst connection.");
                    drop(connection.dst_stream);
                    connection.src_stream.shutdown(Shutdown::Both).expect("Failed to close dst connection.");
                    drop(connection.src_stream);
                }
                else
                {
                    new_connections.push(connection);
                }
            }
            connections.append(&mut new_connections);
            remove_set.clear();
        }

        if blocked_all
        {
            if last_write.elapsed().as_secs() > 1
            {
                sleep(Duration::from_millis(50));
            }
        }
        else
        {
            last_write = Instant::now();
        }
    }
}

fn check_conn(vec: &mut Vec<Connection>, index: usize, reverse_order: bool, remove_queue: &mut HashSet<usize>) -> bool
{
    let conn = &vec[index];
    let mut from = if reverse_order { &conn.dst_stream } else { &conn.src_stream };
    let mut to = if reverse_order { &conn.src_stream } else { &conn.dst_stream };
    let mut buf:Vec<u8> = vec![0; 4096]; //For some reason a new buffer each time MASSIVELY improves performance.
    match from.read(&mut *buf)
    {
        Ok(bytes) => {
            buf.truncate(bytes);
            to.write_all(&*buf).expect("Failed to write.");
            to.flush().unwrap();
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return true,
        Err(_e) => {remove_queue.insert(index);}
    };
    return false;
}

fn get_conn(port: &str) -> Result<TcpStream, &'static str>
{
    return match net::TcpStream::connect(format!("127.0.0.1:{}", port))
    {
        Ok(stream) => {
            stream
                .set_nonblocking(true)
                .expect(&*format!("set_nonblocking failed for port {}.", port));
            stream
                .set_nodelay(true)
                .expect(&*format!("set_nodelay failed for port {}.", port));
            Ok(stream)
        }
        Err(_e) => Err("Failed to bind.")
    }
}

struct Connection
{
    src_stream: net::TcpStream,
    dst_stream: net::TcpStream,
}
