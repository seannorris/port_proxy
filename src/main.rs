use std::io::{Read, Write, Error};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{env, io, net};
use std::fs::File;
use std::collections::HashSet;
use std::net::{Shutdown, TcpStream};
use std::path::Path;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        eprintln!("Argument(s) not provided: <src port> <dst port>");
        return;
    }

    let dir = match std::env::var("TEMP")
    {
        Ok(temp) => temp,
        Err(_) => "/dev/null".to_string()
    };
    let path = Path::new(&dir).join(format!("port_proxy_{}_{}.lock", args[1], args[2]));
    match File::create(path)
    {
        Ok(mut file) => {
            let _ = file.write_all(format!("{}", std::process::id()).as_ref());
        },
        Err(_) => eprintln!("Failed to create lock file.")
    }

    println!("Source: {}, Destination: {}", args[1], args[2]);
    let src_port = &args[1];
    let dst_listener = net::TcpListener::bind(format!("0.0.0.0:{}", args[2]))
        .expect("failed to bind to dst port.");
    dst_listener.set_nonblocking(true)
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
            let connection : &mut Connection = &mut connections[index];
            match check_conn(&mut connection.src_stream, &mut connection.dst_stream)
            {
                Ok(result) => blocked_all &= result,
                Err(_) => { remove_set.insert(index); }
            }
            if !remove_set.contains(&index)
            {
                match check_conn(&mut connection.dst_stream, &mut connection.src_stream)
                {
                    Ok(result) => blocked_all &= result,
                    Err(_) => { remove_set.insert(index); }
                }
            }
        }

        //Check for new connections.
        match dst_listener.accept()
        {
            Ok(stream) => {
                match get_conn(src_port)
                {
                    Ok(src_stream) => {
                        connections.push(Connection {
                            src_stream,
                            dst_stream: setup_conn(stream.0).unwrap(),
                        });
                        println!("Got a connection! (Currently connected: {})", connections.len());
                    }
                    Err(_e) => eprintln!("Failed to bind to source port.")
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => blocked_all &= true,
            Err(_e) => eprintln!("Failed to accept connection.")
        };

        //Drop old connections.
        if remove_set.len() > 0
        {
            for index in 0..connections.len()
            {
                let connection = connections.remove(0);
                if remove_set.contains(&index)
                {
                    println!("Dropped a connection! (Currently connected: {})", connections.len());
                    let _ = connection.dst_stream.shutdown(Shutdown::Both);
                    drop(connection.dst_stream);
                    let _ = connection.src_stream.shutdown(Shutdown::Both);
                    drop(connection.src_stream);
                }
                else
                {
                    connections.push(connection);
                }
            }
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

fn check_conn(from: &mut TcpStream, to: &mut TcpStream) -> Result<bool, Error>
{
    let mut buf:Vec<u8> = vec![0; 4096]; //For some reason a new buffer each time MASSIVELY improves performance.
    return match from.read(&mut *buf)
    {
        Ok(bytes) => {
            buf.truncate(bytes);
            to.write_all(&*buf)?;
            to.flush()?;
            return Ok(false);
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Ok(true),
        Err(e) => Err(e)
    };
}

fn get_conn(port: &str) -> Result<TcpStream, Error>
{
    setup_conn(net::TcpStream::connect(format!("127.0.0.1:{}", port))?)
}

fn setup_conn(stream: TcpStream) -> Result<TcpStream, Error>
{
    stream.set_nonblocking(true)?;
    stream.set_nodelay(true)?;
    Ok(stream)
}

struct Connection
{
    src_stream: net::TcpStream,
    dst_stream: net::TcpStream,
}
