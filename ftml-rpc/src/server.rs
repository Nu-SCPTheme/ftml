/*
 * server.rs
 *
 * ftml-rpc - RPC server to convert Wikidot code to HTML
 * Copyright (C) 2019-2020 Ammon Smith
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

use crate::api::{Ftml as FtmlApi, PROTOCOL_VERSION};
use crate::handle::FtmlHandle;
use crate::Result;
use ftml::html::HtmlOutput;
use ftml::{HtmlRender, PageInfoOwned};
use futures::future::{self, Ready};
use futures::prelude::*;
use serde_json::Value;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;
use tarpc::context::Context;
use tarpc::serde_transport::tcp;
use tarpc::server::{BaseChannel, Channel};
use tokio_serde::formats::Json;

// Prevent network socket exhaustion or related slowdown
const MAX_PARALLEL_REQUESTS: usize = 16;

#[derive(Debug, Clone)]
pub struct Server {
    handle: Arc<FtmlHandle>,
}

impl Server {
    pub fn new() -> Self {
        let handle = Arc::new(FtmlHandle);

        Server { handle }
    }

    pub async fn run(&self, address: SocketAddr) -> io::Result<()> {
        tcp::listen(&address, Json::default)
            .await?
            // Log requests
            .filter_map(|conn| {
                async move {
                    match conn {
                        Ok(conn) => {
                            match conn.peer_addr() {
                                Ok(addr) => info!("Accepted connection from {}", addr),
                                Err(error) => warn!("Unable to get peer address: {}", error),
                            }

                            Some(conn)
                        }
                        Err(error) => {
                            warn!("Error accepting connection: {}", error);

                            None
                        }
                    }
                }
            })
            // Create and fulfill channels for each request
            .map(BaseChannel::with_defaults)
            .map(|chan| {
                let resp = self.clone().serve();
                chan.respond_with(resp).execute()
            })
            .buffer_unordered(MAX_PARALLEL_REQUESTS)
            .for_each(|_| async {})
            .await;

        Ok(())
    }
}

impl FtmlApi for Server {
    // Misc

    type ProtocolFut = Ready<String>;

    #[inline]
    fn protocol(self, _: Context) -> Self::ProtocolFut {
        info!("Method: protocol");

        future::ready(str!(PROTOCOL_VERSION))
    }

    type PingFut = Ready<String>;

    #[inline]
    fn ping(self, _: Context) -> Self::PingFut {
        info!("Method: ping");

        future::ready(str!("pong!"))
    }

    type TimeFut = Ready<f64>;

    fn time(self, _: Context) -> Self::TimeFut {
        info!("Method: time");

        let now = SystemTime::now();
        let unix_time = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System time before epoch")
            .as_secs_f64();

        future::ready(unix_time)
    }

    // Core

    type PrefilterFut = Ready<Result<String>>;

    fn prefilter(self, _: Context, input: String) -> Self::PrefilterFut {
        info!("Method: prefilter");

        let mut text = input;
        let result = match ftml::prefilter(&mut text, &*self.handle) {
            Ok(_) => Ok(text),
            Err(error) => Err(error.to_string()),
        };

        future::ready(result)
    }

    type ParseFut = Ready<Result<Value>>;

    fn parse(self, _: Context, input: String) -> Self::ParseFut {
        info!("Method: parse");

        macro_rules! json {
            // Convert serializeable to StdResult<Value, String>
            ($value:expr) => {
                serde_json::to_value(&$value).map_err(|err| err.to_string())
            };
        }

        let result = match ftml::parse(&input) {
            Ok(tree) => json!(tree),
            Err(error) => Err(error.to_string()),
        };

        future::ready(result)
    }

    type RenderFut = Ready<Result<HtmlOutput>>;

    fn render(self, _: Context, page_info: PageInfoOwned, mut input: String) -> Self::RenderFut {
        info!("Method: render");

        use ftml::Render;

        let html = HtmlRender::new(&*self.handle);
        let info = page_info.as_borrow();
        let result = html
            .transform(&mut input, info, &*self.handle)
            .map_err(|err| err.to_string());

        future::ready(result)
    }
}
