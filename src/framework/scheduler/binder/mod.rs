/* Copyright 2023 shadow3aaa@gitbub.com
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License. */
#![allow(non_snake_case)]
mod IRemoteService;

use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use binder::{BinderFeatures, Interface};
use log::{error, info};
use parking_lot::Mutex;

use super::{BinderMessage, FasData};
use crate::framework::{
    config::Config,
    error::{Error, Result},
};
use IRemoteService::BnRemoteService;

pub struct FasServer {
    config: Config,
    sx: Mutex<Sender<BinderMessage>>,
}

impl Interface for FasServer {}

impl IRemoteService::IRemoteService for FasServer {
    fn sendData(
        &self,
        buffer: i64,
        pkg: &str,
        pid: i32,
        frametime_ns: i64,
        cpu: i32,
    ) -> binder::Result<bool> {
        let Some(target_fps) = self.config.target_fps(pkg) else {
            return Ok(false);
        };

        let frametime = Duration::from_nanos(frametime_ns as u64);

        let data = FasData {
            buffer,
            target_fps,
            pid,
            pkg: pkg.to_string(),
            frametime,
            cpu,
        };

        if let Err(e) = self.sx.lock().send(BinderMessage::Data(data)) {
            error!("{e:?}");
        }

        Ok(true)
    }

    fn removeBuffer(&self, buffer: i64, pid: i32) -> binder::Result<()> {
        if let Err(e) = self
            .sx
            .lock()
            .send(BinderMessage::RemoveBuffer((buffer, pid)))
        {
            error!("{e:?}");
        }

        Ok(())
    }
}

impl FasServer {
    pub fn run_server(config: Config) -> Result<Receiver<BinderMessage>> {
        let (sx, rx) = mpsc::channel();

        thread::Builder::new()
            .name("BinderServer".into())
            .spawn(|| Self::run(sx, config))?;

        Ok(rx)
    }

    fn run(sx: Sender<BinderMessage>, config: Config) -> Result<()> {
        let server = Self {
            config,
            sx: Mutex::new(sx),
        };
        let server = BnRemoteService::new_binder(server, BinderFeatures::default());

        binder::add_service("fas_rs_server", server.as_binder())
            .map_err(|_| Error::Other("Failed to register binder service?"))?;

        info!("Binder server started");
        binder::ProcessState::join_thread_pool();

        Ok(())
    }
}
