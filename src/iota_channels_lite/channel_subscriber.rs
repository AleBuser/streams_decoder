//!
//! Channel Subscriber
//!
use super::Network;
use crate::iota_channels_lite::utils::{payload::json::Payload, random_seed};

use iota::client as iota_client;
use iota_streams::app::transport::tangle::{
    client::{Client, SendTrytesOptions},
    PAYLOAD_BYTES,
};
use iota_streams::app_channels::{
    api::tangle::{Address, Subscriber},
    message,
};

use anyhow::Result;

///
/// Channel subscriber
///
pub struct Channel {
    subscriber: Subscriber<Client>,
    announcement_link: Address,
    subscription_link: Address,
    channel_address: String,
}

impl Channel {
    ///
    /// Initialize the subscriber
    ///
    pub fn new(
        node: Network,
        channel_address: String,
        announcement_tag: String,
        seed_option: Option<String>,
    ) -> Channel {
        let seed = match seed_option {
            Some(seed) => seed,
            None => random_seed::new(),
        };
        let send_opt = SendTrytesOptions::default();
        let client: Client = Client::new(
            send_opt,
            iota::client::ClientBuilder::new()
                .node(&node.as_string())
                .unwrap()
                .build()
                .unwrap(),
        );
        let subscriber = Subscriber::new(&seed, "utf-8", PAYLOAD_BYTES, client);

        let ann_link = match Address::from_str(&channel_address, &announcement_tag) {
            Ok(v) => v,
            Err(_) => Address::default(),
        };

        Self {
            subscriber: subscriber,
            announcement_link: ann_link,
            subscription_link: Address::default(),
            channel_address: channel_address,
        }
    }

    ///
    /// Connect
    ///
    pub fn connect(&mut self) -> Result<String> {
        self.subscriber
            .receive_announcement(&self.announcement_link)?;
        Ok(self.subscription_link.msgid.to_string())
    }
    ///
    /// Read signed packet
    ///
    pub fn read_signed(
        &mut self,
        signed_packet_tag: String,
    ) -> Result<Vec<(Option<String>, Option<String>)>> {
        let mut response: Vec<(Option<String>, Option<String>)> = Vec::new();

        let link = Address::from_str(&self.channel_address, &signed_packet_tag).unwrap();

        let (_, public_payload, _) = self.subscriber.receive_signed_packet(&link)?;
        response.push((
            Payload::unwrap_data(&String::from_utf8(public_payload.0).unwrap()).unwrap(),
            None, //Iot2Tanagle currently only support public masseges
        ));

        Ok(response)
    }

    ///
    /// Generates the next message in the channels
    ///
    pub fn get_next_message(&mut self) -> Option<Vec<String>> {
        let mut ids: Vec<String> = vec![];

        let mut msgs = self.subscriber.fetch_next_msgs();

        for msg in &msgs {
            ids.push(msg.link.msgid.to_string());
        }

        while !msgs.is_empty() {
            msgs = self.subscriber.fetch_next_msgs();

            for msg in &msgs {
                ids.push(msg.link.msgid.to_string());
            }
        }

        Some(ids)
    }
}
