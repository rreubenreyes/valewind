# Changelog

## [Unreleased]

- Poll for user input
- Entity component system
- Better graphics API
  - Less leaky abstractions over texture placement
- More robust asset management/more useful asset cache structure
  - `AssetManager` should support caching different fonts/configurations for the same underlying asset

## [0.0.5]

- Asset loading without `unsafe` (hooray!)
  - However, assets are loaded from disk without memoization - this could get expensive.

## [0.0.4]

- Compose `Context` from separate system context structs
- Better(?) lifetime management for assets
  - `unsafe` used to give fonts a static lifetime - may need to look into alternatives

## [0.0.3]

- Font caching
- Context responsibility cleanup (ongoing)
  - Pass data into canvas by closure, removing the need for excessive borrows

## [0.0.2]

- Add rudimentary text rendering

## [0.0.1]

- Add foundations of game engine
  - Use SDL2 to establish system context and render canvas
  - Game loop renders at 60 FPS
