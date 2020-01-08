/*
 * client.rs
 *
 * ftml-rpc - RPC server to convert Wikidot code to HTML
 * Copyright (C) 2019 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::Result;
use crate::api::{FtmlClient, PROTOCOL_VERSION};
use std::io;
use std::net::SocketAddr;
use tarpc::rpc::client::Config as RpcConfig;
use tarpc::rpc::context;
use tarpc::serde_transport::tcp;
use tokio_serde::formats::Json;

#[derive(Debug)]
pub struct Client {
    client: FtmlClient,
}

impl Client {
    pub async fn new(address: SocketAddr) -> io::Result<Self> {
        let transport = tcp::connect(&address, Json::default()).await?;
        let config = RpcConfig::default();
        let client = FtmlClient::new(config, transport).spawn()?;

        Ok(Client { client })
    }

    // Misc
    pub async fn protocol(&mut self) -> io::Result<String> {
        info!("Method: protocol");

        let version = self.client.protocol(context::current()).await?;

        if PROTOCOL_VERSION != version {
            warn!("Protocol version mismatch! Client: {}, server: {}", PROTOCOL_VERSION, version);
        }

        Ok(version)
    }

    pub async fn ping(&mut self) -> io::Result<()> {
        info!("Method: ping");

        self.client.ping(context::current()).await?;
        Ok(())
    }
}
