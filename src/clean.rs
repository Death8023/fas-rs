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
use std::{thread, time::Duration};

use crate::misc::lock_value;

macro_rules! lock_values {
    (($($path: literal),*), $value: literal) => {
        $(
            let _ = lock_value($path, $value);
        )*
    }
}

pub fn cleaner() -> ! {
    loop {
        lock_values!(
            (
                "/sys/module/mtk_fpsgo/parameters/perfmgr_enable",
                "/sys/module/perfmgr/parameters/perfmgr_enable",
                "/sys/module/perfmgr_policy/parameters/perfmgr_enable",
                "/sys/module/perfmgr_mtk/parameters/perfmgr_enable",
                "/sys/module/migt/parameters/glk_fbreak_enable"
            ),
            "0"
        );

        lock_values!(("/sys/module/migt/parameters/glk_disable"), "1");

        thread::sleep(Duration::from_secs(10));
    }
}
