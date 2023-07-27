# snd - cli based chat

Introducing snd - a command-line chat application built using Rust, designed for seamless communication in two distinct modes: the Local LAN Room and Local User Direct Messaging

➤ Room Chat (Multiuser)

<img alt="Room Chat" src="https://github.com/binbard/snd/assets/28684962/56f690b5-6c6c-4843-ad9a-9d0568709a4c" width="450" height="300">


# Workflow
➼ Identify essential features for both the Local LAN Room and Local User Direct Messaging modes, as well as any optional functionalities.

➼ Choose Rust as the primary programming language and select appropriate libraries and frameworks for networking, concurrency, and user interface design. 

➼ Utilize Rust's capabilities to write platform-independent code, avoid platform-specific dependencies.

<img alt="workflow" src="https://github.com/binbard/snd/assets/28684962/e7b38d88-cbe4-403f-94b2-84bcef3f18f4" width="800" height="300">


# Used tech
➼ __Rust__: The programming language used to build powerful applications due to its strong safety & guarantees performance benefits.

➼ __Crossterm__: A Rust library for terminal manipulation, enabling a cross-platform user interface for the chat tool.

➼ __Multicast__: A communication protocol allowing messages to be sent/receive on multiple recipients simultaneously within a group, facilitating local room-based chat.

-> __TCP/UDP Sockets__: Transport layer protocols utilized in the chat tool to establish reliable (TCP) and connectionless (UDP) communication between users.


# Features

➤ Direct Chat (TCP based)

<img alt="Direct Chat" src="https://github.com/binbard/snd/assets/28684962/0a45a016-58bd-48e1-af7e-91ecdf21a400" width="450" height="300" style="inline">

➤ Builds are Available for Windows, Linux, Android.

<img alt="On Android" src="https://github.com/binbard/snd/assets/28684962/f5d402d4-1b4f-4e10-ba8a-d28a154ec1e5" width="400" height="450">
