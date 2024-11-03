# Changelog

## [Unreleased]

- Context responsibility cleanup

  - Context should just be an API by which to pass abstracted data into to SDL2
  - Engine components should be able to send data to the canvas, rather than having to excessively borrow from the context to do anything

- Font caching
- Poll for user input
- Entity component system

## [0.0.2]

- Add rudimentary text rendering

## [0.0.1]

- Add foundations of game engine
  - Use SDL2 to establish system context and render canvas
  - Game loop renders at 60 FPS
