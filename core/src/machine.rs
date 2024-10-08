// Copyright ©️ 2024 Rogério Senna. All rights reserved.
//
// Licensed under the EUPL, Version 1.2 or – as soon they will be approved by
// the European Commission - subsequent versions of the EUPL (the "Licence");
// You may not use this work except in compliance with the Licence.
// You may obtain a copy of the Licence at:
//
// https://joinup.ec.europa.eu/software/page/eupl
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the Licence is distributed on an "AS IS" basis,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the Licence for the specific language governing permissions and
// limitations under the Licence.
//

use crate::hart::SimpleRV32IHart;

// Init memory as 128MB
pub const DRAM_SIZE: usize = 1024 * 1024 * 128;

// TODO implement a *true* shareable memory between different processes
pub struct Machine {
    pub hart: SimpleRV32IHart,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            hart: SimpleRV32IHart::new(DRAM_SIZE),
        }
    }
}
