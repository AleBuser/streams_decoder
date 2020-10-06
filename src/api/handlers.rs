use crate::iota_channels_lite::channel_subscriber::Channel;
use crate::responses::response_list::ResponseList;
use actix_web::{Error, HttpResponse, HttpRequest};

use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::iota_channels_lite::Network;

pub async fn decode_channel(req: HttpRequest) -> Result<HttpResponse, Error> {
    
    let channel_root = req.match_info().get("channel_root");


    println!(
        "POST /decode_channel -- {:?}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    match channel_root {
        Some(data) => {
            let mut subscriber: Channel = Channel::new(
                Network::Main,
                data.to_string(),
                "000000000000000000000000".to_string(),
                None,
            );

            subscriber.connect().unwrap();

            let msg_list = read_all_public(&mut subscriber).await.unwrap();

            Ok(HttpResponse::Ok().json(ResponseList {
                status: "Success".to_string(),
                messages: msg_list,
            }))
        }
        None => Ok(HttpResponse::Ok().json(format!("No thing!"))),
    }
}

async fn read_all_public(subscriber: &mut Channel) -> Result<Vec<String>> {
    let mut tag = subscriber.get_next_message();

    let mut tag_list: Vec<String> = vec![];
    while tag.is_some() {
        tag_list.push(tag.unwrap());
        tag = subscriber.get_next_message();
    }

    let mut msg_list: Vec<String> = vec![];
    for signed_message_tag in tag_list {
        let msgs: Vec<(Option<String>, Option<String>)> =
            subscriber.read_signed(signed_message_tag).unwrap();
        for (msg_p, _msg_m) in msgs {
            match msg_p {
                None => continue,
                Some(message) => msg_list.push(message),
            }
        }
    }
    Ok(msg_list)
}
