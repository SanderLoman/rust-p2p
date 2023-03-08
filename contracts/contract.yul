// SPDX-License-Identifier: MIT

object "Flashloan" {
    code {
        datacopy(0, dataoffset("init"), datasize("init"))
    }
    object "init" {
        code {}
    }
}