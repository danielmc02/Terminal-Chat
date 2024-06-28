use core::panic;
use std::{io::{stdin, BufRead, Read}, thread, time::Duration};
pub mod models;


use crate::models::session::Session;


use models::session::{self, SessionRoles};
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
#[tokio::main]
async fn main() {



    // Create a new session
    let mut session =  Session::new();

    session::establish_session(session).await;
    



}
