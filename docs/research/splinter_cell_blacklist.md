# Splinter Cell: Blacklist

## Network

- [Quazal](./quazal.md)
- Authentication key: `yl4NG7qZ`
- Encryption key: `CD&ML`

## Logic

- Primarily done through state machines
  - Each state machine has multiple states, identified by IDs (generated from the state name)
  - State transitions are either happening  directly or via Goals

## Inviting others

- Invites are done via UPlay ([UPLAY_Friends_InviteToGame](../../hooks/src/uplay_r1_loader/friends.rs))

### Questions
- Accepting invites as well?
- How does uplay launch the game into join mode? [Example](https://www.youtube.com/watch?v=d45CYK_LuYA)



## custom overlay

- Direct3D uses COM interfaces
 - Calling convention: stdcall with this as first parameter