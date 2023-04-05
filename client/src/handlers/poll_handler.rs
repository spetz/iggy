use std::{io, str};
use std::str::from_utf8;
use tokio::net::UdpSocket;
use tracing::info;
use crate::handlers::response_handler::handle_status;
use crate::message::Message;

const COMMAND: &[u8] = &[2];
const PARTS: usize = 5;

pub async fn handle(input: &[&str], socket: &UdpSocket, buffer: &mut [u8; 1024]) -> io::Result<()> {
    if input.len() != PARTS {
        return Err(io::Error::new(io::ErrorKind::Other, format!("Invalid poll command, expected {} parts.", PARTS)));
    }

    let topic = input[0].parse::<u32>();
    if let Err(error) = topic {
        return Err(io::Error::new(io::ErrorKind::Other, error));
    }

    let partition_id = input[1].parse::<u32>();
    if let Err(error) = partition_id {
        return Err(io::Error::new(io::ErrorKind::Other, error));
    }

    let kind = input[2].parse::<u8>();
    if let Err(error) = kind {
        return Err(io::Error::new(io::ErrorKind::Other, error));
    }

    let value = input[3].parse::<u64>();
    if let Err(error) = value {
        return Err(io::Error::new(io::ErrorKind::Other, error));
    }

    let count = input[4].parse::<u32>();
    if let Err(error) = count {
        return Err(io::Error::new(io::ErrorKind::Other, error));
    }

    let topic = &topic.unwrap().to_le_bytes();
    let partition_id = &partition_id.unwrap().to_le_bytes();
    let kind = &kind.unwrap().to_le_bytes();
    let value = &value.unwrap().to_le_bytes();
    let count = &count.unwrap().to_le_bytes();
    socket.send([COMMAND, topic, partition_id, kind, value, count].concat().as_slice()).await?;
    handle_response(socket, buffer).await?;
    Ok(())
}

async fn handle_response(socket: &UdpSocket, buffer: &mut [u8; 1024]) -> io::Result<()> {
    let payload_length = socket.recv(buffer).await?;
    handle_status(buffer)?;

    if payload_length == 1 {
        info!("No messages found.");
        return Ok(());
    }

    let payload = &buffer[1..payload_length];
    let payload_length = payload.len();
    let messages_count = u32::from_le_bytes(payload[..4].try_into().unwrap());
    info!("Received messages: {}", messages_count);
    let mut position = 4;
    let mut messages = Vec::new();
    while position < payload_length {
        let offset = u64::from_le_bytes(payload[position..position + 8].try_into().unwrap());
        let timestamp = u64::from_le_bytes(payload[position + 8 ..position + 16].try_into().unwrap());
        let message_length = u64::from_le_bytes(payload[position + 16..position + 24].try_into().unwrap()) as usize;
        let message = from_utf8(&payload[position + 24..position + 24 + message_length]).unwrap();
        position = position + 24 + message_length;
        messages.push(Message { offset, timestamp, length: message_length as u64, payload: message.to_string() });
        if position >= payload_length {
            break;
        }
    }

    messages.sort_by(|x,y| x.offset.cmp(&y.offset));
    info!("Messages: {:#?}", messages);

    Ok(())
}