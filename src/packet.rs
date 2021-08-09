// <copyright file="packet.rs" company="Fraunhofer Institute for Manufacturing Engineering and Automation IPA">
// Copyright 2021 Fraunhofer Institute for Manufacturing Engineering and Automation IPA
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// </copyright>

use std::time::Duration;

pub struct SentPacket {
    pub index: u64,
    pub sent_duration: Duration,
    pub sent_timestamp: Duration
}

pub struct ReceivedPacket {
    pub index: u64,
    pub received_duration: Duration,
    pub received_timestamp: Duration,
    pub server_timestamp: Duration
}
