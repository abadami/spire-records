# Spire Records
Slay the Spire 2 personal statistic keeper. This is a desktop app that gets data from a
local Slay the Spire 2 game.

## Architecture

### Spire Record Interpreter

Reads the Slay the Spire 2 .run files and interprets them to a better data object
to gather statistical data on.

### Spire Record Keeper

Reads data gathered from the Spire Record Interpreter and gleams insights both in
each individual run and all together

### Spire Record UI

Takes information from the record keeper and presents it to us, where we may hopefully interpret 
it as we would like
