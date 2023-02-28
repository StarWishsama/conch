////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.ren>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
////////////////////////////////////////////////////////////////////////////////////////////////////

// 0x544 unknown 0x542

use bytes::{BufMut, BytesMut};

pub mod t1;
pub mod t8;
pub mod t18;
pub mod t100;
pub mod t106;
pub mod t107;
pub mod t109;
pub mod t116;
pub mod t124;
pub mod t128;
pub mod t141;
pub mod t142;
pub mod t144;
pub mod t145;
pub mod t147;
pub mod t154;
pub mod t16e;
pub mod t177;
pub mod t187;
pub mod t188;
pub mod t191;
pub mod t511;
pub mod t516;
pub mod t521;
pub mod t525;
pub mod t545;
pub mod t52d;
pub mod t548;

trait TlvField: Default {
    fn tag() -> u16;

    fn to_payload(&self, b: &mut BytesMut);

    fn to_bytes(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(4);

        b.put_u16(Self::tag());
        b.put_u16(0); // payload length
        self.to_payload(&mut b);

        let l = b.len() - 4;
        b[2..4].swap_with_slice(&mut l.to_be_bytes()); // set payload length

        b
    }
}
