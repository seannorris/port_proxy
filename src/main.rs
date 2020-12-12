use std::{
    env,
    net::{
        self,
        TcpStream,
        Shutdown::Both
    },
    thread,
    io::{Read, Write, Error},
    fs::File,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering}
    }
};

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

    loop
    {
        match dst_listener.accept()
        {
            Ok( stream) => {
                match get_conn(src_port)
                {
                    Ok(src_stream) => {
                        let src_stream_cloned = src_stream.try_clone().expect("Failed to clone src stream.");
                        let dst_stream = setup_conn(stream.0).unwrap();
                        let dst_stream_cloned = dst_stream.try_clone().expect("Failed to clone dst stream.");
                        let dropped = Arc::new(AtomicBool::new(false));
                        check_conn(src_stream_cloned, dst_stream_cloned, dropped.clone());
                        check_conn(dst_stream, src_stream, dropped);
                        println!("Got a connection!");
                    }
                    Err(_e) => eprintln!("Failed to bind to source port.")
                }
            },
            Err(_e) => eprintln!("Failed to accept connection.")
        };
    }
}

fn check_conn(mut from: TcpStream, mut to: TcpStream, dropped: Arc<AtomicBool>)
{
    thread::spawn(move || {
        let mut break_loop = false;
        loop
        {
            match do_read_write(&mut from, &mut to)
            {
                Ok(_) => (),
                Err(_e) => break_loop = true
            };
            if break_loop || dropped.load(Ordering::Relaxed)
            {
                break;
            }
        }
        if !dropped.load(Ordering::Relaxed)
        {
            dropped.store(true, Ordering::Relaxed);
            println!("Dropped a connection!");
        }
        let _ = from.shutdown(Both);
        let _ = to.shutdown(Both);
    });

}

fn do_read_write(from: &mut TcpStream, to: &mut TcpStream) -> Result<(), Error>
{
    let mut buf:Vec<u8> = vec![0; 4096]; //For some reason a new buffer each time MASSIVELY improves performance.
    let size = from.read(&mut *buf)?;
    buf.truncate(size);
    to.write_all(&*buf)?;
    Ok(to.flush()?)
}

fn get_conn(port: &str) -> Result<TcpStream, Error>
{
    setup_conn(net::TcpStream::connect(format!("127.0.0.1:{}", port))?)
}

fn setup_conn(stream: TcpStream) -> Result<TcpStream, Error>
{
    stream.set_nodelay(true)?;
    Ok(stream)
}
