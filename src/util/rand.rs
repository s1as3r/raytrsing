/*
 * Reference: PCG Random Number Generation for C.
 *
 * Copyright 2014 Melissa O'Neill <oneill@pcg-random.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * For additional information about the PCG random number generation scheme,
 * including its license and other licensing options, visit
 *
 *       http://www.pcg-random.org
 */

#[derive(Debug, Clone)]
pub struct PCG32RNG {
    state: u64,
    inc: u64,
}

impl Default for PCG32RNG {
    fn default() -> Self {
        Self {
            state: 0x853c49e6748fea9b,
            inc: 0xda3e39cb94b95bdb,
        }
    }
}

impl PCG32RNG {
    pub fn random(&mut self) -> u32 {
        let oldstate = self.state;

        // Advance internal state
        self.state = oldstate
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc | 1);
        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted = (((oldstate >> 18u32) ^ oldstate) >> 27u32) as u32;
        let rot = (oldstate >> 59u32) as u32;
        (xorshifted >> rot) | (xorshifted << ((-(rot as i32)) & 31))
    }

    pub fn random_f64(&mut self) -> f64 {
        self.random() as f64 / (u32::MAX as f64)
    }

    pub fn random_bounded_f64(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.random_f64()
    }
}
