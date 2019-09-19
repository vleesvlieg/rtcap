# TODO
* define lib traits
* tpacketv3
* packet_mmap
* async


# Structure
## rtcap-lib
Defines traits for a modular traffic capture architecture.

## rtcap
Contains implementations of the traits in rtcap-lib. (mio, tokio, mmap, iovec).
This lib can be used to create a concrete capturing application. Use crate
features to enable certain items (features = ["tpacket", "mmap", "tokio"])

## rtcap-bin
CLI tool for capturing

## rtcap-tests
Integration tests
