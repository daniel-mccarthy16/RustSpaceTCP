use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use std::thread;
use std::io::Read;
use lazy_static::lazy_static;

const SOCKET_PATH: &str = "/tmp/wtfistcp_unix_socket";

struct ClientConnection {
    stream: Arc<Mutex<UnixStream>>,
    socket_state: SocketState,
    bound_port: Option<u16>
}


pub struct UnixSocketManager {
}

lazy_static! {
    static ref CONNECTIONS_TABLE: Mutex<HashMap<u32, ClientConnection>> = Mutex::new(HashMap::new());
    static ref ID_COUNTER: Mutex<u32> = Mutex::new(0);
}


impl UnixSocketManager {

    
    pub fn initialize() -> Result<(),std::io::Error>  {
        let listener = UnixListener::bind(SOCKET_PATH)?;
        thread::spawn(move || {
            for stream in listener.incoming() {
            // accept connections and process them, spawning a new thread for each one
                match stream {
                    Ok(stream) => {
                        let stream_arc = Arc::new(Mutex::new(stream));
                        thread::spawn(|| Self::handle_client(stream_arc));
                    }
                    Err(_err) => {
                        /* connection failed */
                        eprintln!("[ERROR]: failed to handle inbound unix socket connection");
                        break;
                    }
                }
            }
        });
        Ok(())
    }

    fn handle_client (  stream: Arc<Mutex<UnixStream>> ) {
        let mut payload_buffer : Vec<u8>;
        let mut header_buffer = [0u8; 5];

        loop {
            let mut stream_guard = stream.lock().unwrap();
            match stream_guard.read_exact(&mut header_buffer) {
                Ok(()) => {
                    let payload_size = u32::from_be_bytes(header_buffer[1..4].try_into().unwrap()) as usize;
                    let message_type = MessageType::from_byte(header_buffer[0]);

                    payload_buffer = vec![0u8; payload_size];

                    match stream_guard.read_exact(&mut payload_buffer) {
                        Ok(()) => {
                            let mut response_buffer = Vec::new();
                            Self::handle_message( message_type, &payload_buffer, &mut response_buffer, &stream);
                        }
                        Err(e) => {
                            eprintln!("[ERROR]: problem when reading into unix socket payload buffer {e}");
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from stream: {}", e);
                    break;
                }
            }
        }
    }

    fn handle_message(  message_type: Result<MessageType, &'static str>, payload: &[u8], response_buffer: &mut Vec::<u8>, stream: &Arc<Mutex<UnixStream>> ) {
        match message_type {
            Ok(mt) => {
                match mt {
                    MessageType::Connect => {
                    },
                    MessageType::Send => {
                    },
                    MessageType::Receive => {
                    },
                    MessageType::Close => {
                    },
                    MessageType::Accept => {
                    },
                    MessageType::Listen => {
                        Self::handle_listen_message(payload, response_buffer, stream).unwrap();
                    },
                    MessageType::Bind => {
                        Self::handle_bind_message(payload, response_buffer, stream).unwrap();
                    },
                    MessageType::Socket => {
                        Self::handle_socket_message( &payload, response_buffer, stream).unwrap();
                    },
                }
            }
            Err(_) => {
                eprintln!("[ERROR]: Invalid message type received");
            }
        }
    }

    fn handle_socket_message(  payload: &[u8], response_buffer: &mut Vec::<u8>, stream: &Arc<Mutex<UnixStream>> ) -> Result<(), &'static str> {
        // let port_number: u16 = u16::from_be_bytes([payload[0], payload[1]]);
        let new_client_connection = ClientConnection {
            stream: stream.clone(),
            bound_port: None,
            socket_state: SocketState::Created
        };
        let mut connections_table_lock = CONNECTIONS_TABLE.lock().unwrap();
        let unique_fd = Self::get_next_unique_fd_id();
        connections_table_lock.insert(unique_fd, new_client_connection);
        //TODO - add some kind of response here?
        Ok(())
    }

    fn handle_bind_message(  payload: &[u8], response_buffer: &mut Vec::<u8>, stream: &Arc<Mutex<UnixStream>> ) -> Result<(), &'static str> {
        let mut connections_table_lock = CONNECTIONS_TABLE.lock().unwrap();
        let unique_fd: u32 = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3] ]);
        let desired_port: u16 = u16::from_be_bytes([payload[4], payload[5]]);

        if connections_table_lock.values().any(|conn| conn.bound_port == Some(desired_port)) {
            return Err("[ERROR]: port number is already in use ...");
        }

        match connections_table_lock.get_mut(&unique_fd) {
            Some(connection) => {
                connection.socket_state = SocketState::Bound;
                connection.bound_port = Some(desired_port);
            },
            None => {
                return Err("[ERROR]: could not find unix connection when attempting to bind");
            }
        }
        //TODO - add some kind of response here?
        Ok(())
    }

    fn handle_listen_message(  payload: &[u8], response_buffer: &mut Vec::<u8>, stream: &Arc<Mutex<UnixStream>> ) -> Result<(), &'static str> {
        let mut connections_table_lock = CONNECTIONS_TABLE.lock().unwrap();
        let unique_fd: u32 = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3] ]);
        match connections_table_lock.get_mut(&unique_fd) {
            Some(connection) => {
                connection.socket_state = SocketState::Listening;
            },
            None => {
                return Err("[ERROR]: could not find unix connection when attempting to bind");
            }
        }
        //TODO - add some kind of response here?
        Ok(())
    }

    fn get_next_unique_fd_id () -> u32 {
        let mut num = ID_COUNTER.lock().unwrap();
        *num += 1; 
        *num
    }

    pub fn port_is_open (port: u16) -> bool {
        let connections_table_lock = CONNECTIONS_TABLE.lock().unwrap();
        for connection in connections_table_lock.values() {
            if let Some(bound_port) = connection.bound_port {
                if bound_port == port && matches!(connection.socket_state, SocketState::Listening) {
                    return true;
                }
            }
        }
        false
    }
}

enum SocketState {
    Created,
    Bound,
    Listening
}


enum MessageType {
    Connect = 1,
    Send = 2,
    Receive = 3,
    Close = 4,
    Accept = 5,
    Listen = 6,
    Bind = 7,
    Socket = 8,
}

impl MessageType {
    fn from_byte(byte: u8) -> Result<Self, &'static str> {
        match byte {
            1 => Ok(Self::Connect),
            2 => Ok(Self::Send),
            3 => Ok(Self::Receive),
            4 => Ok(Self::Close),
            5 => Ok(Self::Accept),
            6 => Ok(Self::Listen),
            7 => Ok(Self::Bind),
            8 => Ok(Self::Socket),
            _ => Err("[ERROR] invalid message type received over unix socket")
        }
    }
}


