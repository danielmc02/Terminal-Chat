use std::{io::stdin, time::Duration};

use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use crate::{constants::consants::{BUFFER_SIZE, VERSION}, models::session};

/// Module for a chat session

#[derive(PartialEq)]
pub enum SessionRoles {
    Host,
    Client,
   
}


pub struct Session {
    pub role: SessionRoles,
    tcp_stream: Option<TcpStream>
   // pub attempt: u8,
}
impl std::fmt::Display for SessionRoles
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionRoles::Client=>{
                return  write!(f,"Client");
            },
            SessionRoles::Host =>{
              return  write!(f,"Host");
            }
        }
        
    }
}
impl Session {
    /// Returns default
    pub async fn new() -> Session {
        println!("Please choose an option:\n1) Host\n2) Client)");

        let mut buffer: String = String::new();
        let mut standard_input = stdin().lines();    
        let mut attempt: u8 = 3;

        let role: SessionRoles  = loop {
            
            // By default it is a host
            let mut temp_role = SessionRoles::Host;

            for lines in standard_input.next() {
                let user_command = lines.expect("Something went wrong extracting response");

                match user_command.to_lowercase().as_str() {
                    "host" => {
                        println!("IS HOST");
                        temp_role = SessionRoles::Host;
                        break;
                    }
                    "client" => {
                        println!("IS CLIENT");
                        temp_role = SessionRoles::Client;

                        break;
                    }
                    _ => {
                        print!(
                            "\"{}\" is not a recognized command. ",
                            user_command.to_lowercase().as_str()
                        )
                    }
                }
            }
            if temp_role == SessionRoles::Client || temp_role == SessionRoles::Host {
                break temp_role;
            }

            attempt -= 1;

            if attempt == 0 || attempt > 4 {
                println!("\n\nRan out of attempts, good bye.\n");
                std::process::exit(0)
            } else {
                println!("{} attempts left", attempt)
            }
        };
        let d: Option<TcpStream> = {
            if role == SessionRoles::Client
            {
                 let mut connection = TcpStream::connect("0.0.0.0:1963").await.unwrap();
                 Some(connection)
            }
            else {
                 None
            }
        };
   Session { role: role,tcp_stream:d }
    }

    pub async fn send_handshake(mut self)
    {
        self.tcp_stream.unwrap().write_all(b"VERSION: 0.0.1\nMESSAGE: fuck you").await.unwrap();

    }
 
    pub async fn create(self)
    {
        println!("Starting session as: {}",self.role);
    
        match self.role {
            SessionRoles::Client=>{
                self.create_client().await
            }
            ,
            SessionRoles::Host=>{
                create_host().await
            }
    
        }
        
    }

    async fn create_client(self)
{
    println!("Sending Handshake");
   self.send_handshake().await;

    //connection.write_all(b"Hello there").await.unwrap();
  //  tokio::net::
}
    

}






async fn create_host()
{
    
    tokio::time::sleep(Duration::new(3, 0)).await;

    let address = "0.0.0.0:1963";
 let listener=   TcpListener::bind(address).await.unwrap();
    println!("Succesfully listening on port 1963");
      match  listener.accept().await
        {
            Ok((stream,adr))=>{
                let mut b1 = [0;BUFFER_SIZE];
                println!("GOT CONNECTION");
                stream.peek(&mut b1).await.unwrap();
                println!("{:?}", b1);
              //  tokio::spawn(future)
            }
            Err(_)=>
            {
                println!("ERROR WITH CLIENT")
            }
        }
    

  
}



