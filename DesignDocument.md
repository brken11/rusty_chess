# Rust Chess Engine - System Design Document

## Project Goals

Build a multithreaded Rust based chess engine that can
 - [ ] Run on a Terminal UI
 - [ ] Run on a GUI
 - [ ] Play with a Local Player(User or AI)
 - [ ] Play with a Remote network Player(User or AI)
This project will be built the explicit purpose of avoiding external libraries, avoid async and will use queues to communicate

---

## Core Components Overview

### 1. `main.rs`

#### Purpose

Central coordinator that will initialize the other sub-systems

#### Tasks

 - [ ] Load and parse config file
 - [ ] Spawn Log Thread based on config
 - [ ] Spawn UI Thread based on config
 - [ ] Spawn Game Thread
 - [ ] Establish connection between both threads
 - [ ] Wait for shut down to orchestrate shutdown procedure

#### Interactions
 - `GameManager`
 - `UIManager`
 - `Logger`

---

### 2. `ui_manager.rs`

#### Purpose

Handles user facing interface operations (terminal or GUI)

#### Tasks
- [ ] Manage User Input
   - [ ] If Terminal, spawn thread for terminal input
   - [ ] If GUI, load GUI
      - [ ] Spawn GUI thread with handlers for user input
      - [ ] Spawn GUI thread to handle looks and animations
- [ ] Send UI requests to GameManager
- [ ] Update display based on messages/requests from GameManager

#### Interactions
 - `GameManager`
 - `GUIListener`
 - `GUIManager`
 - `Logger`

---

### 3. `game_manager.rs`

#### Purpose

Handles core gameplay and the arbiter of truth. If hosting between two programs, the host of the session. Coordinates player threads and maintains clocks and validate moves.

#### Tasks
 - [ ] Manage core game state
 - [ ] Process move requests
    - [ ] Reject and update threads if move is invalid
    - [ ] Inform threads when it's "their" turn
    - [ ] Manage turn orders
    - [ ] Handle premove updates
 - [ ] Manage timers (if enabled)
 - [ ] Spawn and Manage
    - [ ] Local AI thread
    - [ ] Remote player thread
 - [ ] Broadcast updates to UI and network

#### Interactions
 - `UIManager`/`GUIManager`
 - `PlayerHandler`
 - `NetworkManager`
 - `Logger`

---

### 4. `handler.rs`

#### Purpose

Interface for Game to handle different player inputs. Whether local, local AI, or remotely.

#### Types

 - `LocalHandler` - gets input from UIManager
 - `RemoteHandler` - gets input from dedicated `NetworkManager` socket.
 - `AIHandler` - gets input from local `AIThread`

#### Tasks
 - [ ] Validate move against local board copy
 - [ ] Submit move to `GameManager`
 - [ ] Wait for updates from `GameManager`
 - [ ] Synchronize local state with `GameManager`'s

#### Interactions
 - `GameManager`
 - `Logger`

---

### 5. `network_manager.rs`

#### Purpose

Handles TCP/IP communications

#### Tasks
 - [ ] Establish connection with remote client
 - [ ] Send/receive updates
 - [ ] Sync board, clock, and game data.
 - [ ] Translate socket data
    - [ ] Translate move data
    - [ ] Translate premove data
    - [ ] Translate `GameManager` instructions
       - [ ] Sync data
       - [ ] Error Messages
       - [ ] Sync instuctions
       - [ ] Game State Updates

#### Interactions
 - `GameManager`
 - `RemoteHandler`
 - `Logger`

---

### 6. `logger.rs`

#### Purpose

Thread for managing system logs

#### Tasks
 - [ ] Collect log messages from shared queue
 - [ ] Output to terminal
    - [ ] Optionally output to file
 - [ ] Filter based on loglevel

#### Interactions
 - All threads via shared queue
