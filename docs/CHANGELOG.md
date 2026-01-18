# CHANGELOG

## Version 0.2.1
Voxio is now the default backend.

Crossbeam has been integrated. All event driven
architecture now uses bounded crossbeam channels, and all
events are handled by the `select!` macro for increased
responsiveness. Furthermore, the crossbeam ArrayQueue
removes the need for any lock-based architecture within the
project.

Main loop and library initialization logic has been cleaned
up substantially.

Error handling throughout the program has been seriously
buffed.

A single FFMPEG check is made on intialization rather than
everytime a waveform is generated.

Small visual tweeks

Updated docs

New dependencies: 
- Voxio
- Crossbeam (channel and queue)

Removed dependencies:
- Parking lot
- Rodio

