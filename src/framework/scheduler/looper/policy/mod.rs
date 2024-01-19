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
pub mod config;
mod extract;

use std::time::Instant;

#[cfg(debug_assertions)]
use log::debug;

use super::Buffer;
use crate::framework::{Config, Mode};

use config::PolicyConfig;
use extract::PolicyData;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
pub enum NormalEvent {
    Restrictable,
    None,
    Release,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
pub enum JankEvent {
    None,
    Jank,
    BigJank,
}

impl Buffer {
    pub fn normal_event(&mut self, config: &Config, mode: Mode) -> NormalEvent {
        let config = PolicyConfig::new(config, mode, self);
        let Some(policy_data) = PolicyData::extract(self) else {
            return NormalEvent::None;
        };

        #[cfg(debug_assertions)]
        {
            debug!("policy config: {config:?}");
            debug!("policy data: {policy_data:?}");
        }

        self.frame_analyze(config, policy_data)
    }

    pub fn jank_event(&mut self, config: &Config, mode: Mode) -> JankEvent {
        let config = PolicyConfig::new(config, mode, self);
        let Some(policy_data) = PolicyData::extract(self) else {
            return JankEvent::None;
        };

        #[cfg(debug_assertions)]
        {
            debug!("policy config: {config:?}");
            debug!("policy data: {policy_data:?}");
        }

        self.jank_analyze(config, policy_data)
    }

    fn frame_analyze(&mut self, config: PolicyConfig, policy_data: PolicyData) -> NormalEvent {
        let diff = policy_data.normalized_frame.as_secs_f64() - 1.0;
        self.acc_frame += diff;

        if self.acc_timer.elapsed() * policy_data.target_fps < config.acc_dur {
            return NormalEvent::None;
        }

        let result = if self.acc_frame >= config.scale {
            #[cfg(debug_assertions)]
            debug!("JANK: unit jank");

            NormalEvent::Release
        } else {
            #[cfg(debug_assertions)]
            debug!("JANK: no jank");

            NormalEvent::Restrictable
        };

        self.acc_frame = 0.0;
        self.acc_timer = Instant::now();

        result
    }

    fn jank_analyze(&mut self, config: PolicyConfig, policy_data: PolicyData) -> JankEvent {
        let diff = policy_data.normalized_avg_frame.as_secs_f64() - 1.0;

        if diff >= config.big_jank_scale {
            #[cfg(debug_assertions)]
            debug!("JANK: big jank");

            self.acc_frame = 0.0;

            JankEvent::BigJank
        } else if diff >= config.jank_scale {
            #[cfg(debug_assertions)]
            debug!("JANK: simp jank");

            self.acc_frame = 0.0;

            JankEvent::Jank
        } else {
            JankEvent::None
        }
    }
}
