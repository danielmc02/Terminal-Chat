use std::io::stdin;

use tokio::{io::AsyncWriteExt, net::TcpStream};

/// Module for a chat session

#[derive(PartialEq)]
pub enum SessionRoles {
    Host,
    Client,
   
}

pub struct Session {
    pub role: SessionRoles,
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
    pub fn new() -> Session {
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
   Session { role: role }
    }

 


}

pub async fn establish_session(session: Session)
{
    println!("Starting session as: {}",session.role)

    match session.role {
        SessionRoles::Client=>{
            create_client().await
        }
        ,
        SessionRoles::Host=>{
            create_host().await
        }

    }
    
}



async fn create_client()
{
    let mut connection = TcpStream::connect("0.0.0.0:1963").await.unwrap();
    connection.write_all(b"Hello there").await.unwrap();
  //  tokio::net::
}


async fn create_host()
{
    
    tokio::sleep(Duration::new(3, 0));

    let address = "0.0.0.0:1963";
 let listener=   TcpListener::bind(address).await.unwrap();
    println!("Succesfully listening on port 1963");
      match  listener.accept().await
        {
            Ok((stream,adr))=>{
                let mut b1 = [0;10];
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



