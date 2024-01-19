#!/bin/bash
#
# Copyright 2023 shadow3aaa@gitbub.com
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
fix_codes() {
	source $SHDIR/script/toolchains.sh
	FIX="$RR clippy --fix --allow-dirty --allow-staged -- --target aarch64-linux-android"

	cd $SHDIR && $FIX
	cd $SHDIR/zygisk/rust && $FIX
}