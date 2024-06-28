# OpenAPI Discord client generators

This project generates discord API clients from Discord's published OpenAPI 3.1 spec (https://github.com/discord/discord-api-spec)

**This project is in a early state and does not produce any usable clients yet.**

The goal is to be able to produce at the very least a valid rust and typescript/js client but more languages can be added in the feature.

There are a number of challenges with this project however:
- Discord uses OpenAPI 3.1 which is not supported by any existing generators i could find, but even if they did support it...
- Discord uses a custom one-of extension because of limitations in the discord api, so the need to write a dedicated generator was probably gonna be there anyways
- They haven't published anything for the gateway, my plan there is to hand write my own spec for it and use this to generate the types, it's not super big so should be doable but we'll see.

## The goals for this project are:
- Generate discord data types in rust and typescript/js
- Generate discord http API clients in Rust and Typescript/JS (I do need gateway support as well but that might come later because of the lack of gateway API in the spec)
- Have a somewhat reasonable API, i will be renaming certain objects to be more general purpose (e.g ChannelResponse -> Channel)

## The non-goals are:
- A fully compliant OpenAPI 3.1 generator, this generator will make a lot of assumptions and take a lot of shortcuts, it will only be handling the actual used parts of OpenAPI and so on, it will not work with anything else. (probably)
    - This is to speed up development of it, creating a fully compliant OpenAPI generator is a huge undertaking in and of itself, together with trying to make it output a nice usable API you'd probably be at the very least doubling or tripling the scope of this project.
- Output a high level API akin to discord.js
    - This will be fairly low level
    - Another library could be built on top of this that provides high level functionality, but doing that now would be like running before learning to walk.
- Support every single language out there
    - Rust and typescript/js first, then other languages, though i'm happy to accept contributions but maybe wait a small while until this project is more stable, things are changing a lot now in the early stages.

## Project structure

The generator consists of 3 parts

- Minimal OpenAPI 3.1 spec types (spec.rs)
    - Very bare bones typings that handles the used discord portions of it, i did not find any existing 3.1 supported crates so this will have to do
    - Does not follow the spec probably... (but that's a non-goal)
- Intermediary Representation generator (ir.rs)
    - It's somewhat clunky to work with the openAPI types directly so i do a lot of pre-processing to simplify everything and take a lot of the work off the actual code generators.
- Code generation (code_gen.rs, rust_codegen.rs)
    - Handles the actual generation using the intermediary types