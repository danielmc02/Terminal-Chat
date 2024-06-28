use core::panic;
use std::{io::{stdin, BufRead, Read}, thread, time::Duration};

use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
#[tokio::main]
async fn main() {
    println!("Please choose an option:\n1) Host\n2) Client)");

    let mut buffer: String = String::new();
    let mut standard_input = stdin().lines();

    let mut attempt: u8 = 1;
    let mut session_role: SessionRoles = SessionRoles::None;
    while attempt <= 3 {
        for lines in standard_input.next() {
            let user_command = lines.unwrap();

            match user_command.to_lowercase().as_str() {
                "host" => {
                    println!("IS HOST");
                    session_role = SessionRoles::Host;
                    break;
                }
                "client" => {
                    println!("IS CLIENT");
                    session_role = SessionRoles::Client;
                    break;
                }
                _ => {
                    print!(
                        "\"{}\" is not a recognized command",
                        user_command.to_lowercase().as_str()
                    )
                }
            }
        }
        if session_role == SessionRoles::Client || session_role == SessionRoles::Host {
            break;
        }
        attempt += 1;

        if attempt == 4 {
            println!("Ran out of attempts, good bye.");
            std::process::exit(0)
        } else {
            println!(". {} attempts left", 4 - attempt)
        }
    }
    
    match  session_role{
        
        SessionRoles::Host =>
        {
            println!("SETTING UP HOST");
            create_host().await;
        }

        SessionRoles::Client =>
        {
            create_client().await;
        }
        _=>
        {
            panic!("Error: Should have not gotten here")
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
    thread::sleep(Duration::new(3, 0));

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



#[derive(PartialEq)]
enum SessionRoles {
    Host,
    Client,
    None,
}
