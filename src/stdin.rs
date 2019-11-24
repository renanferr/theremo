use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::io;
use std::io::{Read, Write};
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

pub fn spawn() -> Receiver<u8> {
    let stdin = 0;
    let mut buffer: [u8; 1] = [0; 1];
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
  
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
  
    let (tx, rx) = mpsc::channel::<u8>();
  
    let stdout = io::stdout();
    let mut reader = io::stdin();
  
    thread::spawn(move || loop {
      stdout.lock().flush().unwrap();
      reader.read_exact(&mut buffer).unwrap();
      let key: u8 = buffer[0];
      println!("{}", key);
      tx.send(key).unwrap();
    });

    return rx;
  }