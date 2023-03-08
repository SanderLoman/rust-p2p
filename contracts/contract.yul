// SPDX-License-Identifier: MIT

object "PureYul" {
    code {
        datacopy(0, dataoffset("runtime"), datasize("runtime"))
        return(0, datasize("runtime"))
    }
    
    object "runtime" {
        code {

        }
    }
}