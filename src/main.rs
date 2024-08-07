use core::panic;
use std::{io::{stdin, BufRead, Read}, thread, time::Duration};
pub mod models;

pub mod constants;
use crate::models::session::Session;


use models::session::{self, SessionRoles};
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
#[tokio::main]
async fn main() {



    // Create a new session
    let current_session =  Session::new().await;


    current_session.create().await;
    



}
