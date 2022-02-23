use vemt_mio_stdin::stdin::StdinStream;
use vemt_mio_stdin::{Events, Interest, Poll, Token, Waker};

use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const INPUT_TOKEN: Token = Token(0);
const TIMER_TOKEN: Token = Token(1);
const WAIT: u64 = 5;

struct Command {
    pub id: u64,
    pub instant: Instant,
}

fn start_timer_thread(
    rx: Receiver<Command>,
    current_id: Arc<Mutex<u64>>,
    waker: Waker,
) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let command = rx.recv().unwrap();
        let d = command.instant - Instant::now();
        thread::sleep(d);
        if *current_id.lock().unwrap() == command.id {
            waker.wake().unwrap();
        }
    })
}

fn main() -> io::Result<()> {
    env_logger::init();

    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Create stdin stream.
    let stdin = File::open("/dev/tty")?;
    let mut stdin = StdinStream::from_std(stdin);

    // Register the server with poll we can receive events for it.
    poll.registry()
        .register(&mut stdin, INPUT_TOKEN, Interest::READABLE)?;
    let waker = Waker::new(poll.registry(), TIMER_TOKEN).expect("unable to create waker");

    let id = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();
    let _timer_thread = start_timer_thread(rx, id.clone(), waker);

    println!("Demo for reading stdin with timeout.");
    println!();

    loop {
        print!("type some words ({} seconds wait...): ", WAIT);
        io::stdout().flush().unwrap();
        *id.lock().unwrap() += 1;

        let instant = Instant::now() + Duration::from_secs(WAIT);
        let command = Command {
            id: id.lock().unwrap().clone(),
            instant,
        };
        tx.send(command).unwrap();

        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                INPUT_TOKEN => {
                    let mut buf = [0_u8; 16];

                    dbg!(&events);
                    dbg!(stdin.read(&mut buf)?);
                    dbg!(&buf);
                }
                TIMER_TOKEN => {
                    println!();
                    println!("5 seconds passed.");
                }
                _ => unreachable!(),
            }
        }

        println!();
    }

    // timer_thread.join().unwrap();
}
