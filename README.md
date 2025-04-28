# ORE Burn

**ORE Burn** 
        
## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Event`](api/src/event.rs) – Custom program events.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions

#### User  
- [`Burn`](program/src/claim.rs) – Claim accumulated rewards from a creator account.

#### Admin
- [`Initialize`](program/src/initialize.rs) – Initialize the creator program config.

## State
- [`Authority`](api/src/state/authority.rs) – Tracks the global program variables.

## Get started

Compile your program:
```sh
steel build
```

Run unit and integration tests:
```sh
steel test
```
