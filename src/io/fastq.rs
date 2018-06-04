/*
* Copyright 2014-2016 Johannes Köster.
* Licensed under the MIT license (http://opensource.org/licenses/MIT)
* This file may not be copied, modified, or distributed
* except according to those terms.
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/


// Structure used from Rust-bio
// https://github.com/rust-bio/rust-bio/blob/master/src/io/fastq.rs


pub struct Fastq {
    id: String,
    desc: String,
    seq: Vec<char>,
    qual: Vec<char>
}

impl Fastq {
    pub fn new() -> Fastq {
        Fastq {
            id: String::new(),
            seq: Vec::new(),
            desc: String::new(),
            qual: Vec::new()
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn desc(&self) -> &str {
        self.desc.as_ref()
    }

    pub fn seq(&self) -> &str {
        self.seq.as_bytes()
    }

    pub fn qual(&self) -> &str {
        self.qual.as_bytes()
    }
}