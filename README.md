# Dynamic Object Editor Plugin

A SA-MP plugin written in Rust for managing dynamic objects with advanced editing capabilities.

## Features

- Create dynamic objects with custom positions and rotations
- Edit objects in real-time using the built-in SA-MP object editor
- Get and set object positions and rotations
- Export and import map data for easy object management
- Thread-safe object handling using Rust's advanced concurrency features

## Installation

1. Download the latest release:
   - Windows: `dynamic_object_editor.dll`
   - Linux: coming soon

2. Copy the plugin file to your server's `plugins` folder

3. Add the following to your `server.cfg`:
```
plugins dynamic_object_editor.dll
```
(Use .so extension for Linux)

4. Include the header file in your script:
```pawn
#include <dynamic_object_editor>
```

## Usage

```pawn
// Create a new dynamic object
new objectid = CreateDynamicObject(modelid, Float:x, Float:y, Float:z, Float:rx, Float:ry, Float:rz);

// Edit an object
EditDynamicObject(playerid, objectid);

// Get object position
new Float:x, Float:y, Float:z;
GetDynamicObjectPos(objectid, x, y, z);

// Set object position
SetDynamicObjectPos(objectid, Float:x, Float:y, Float:z);

// Get object rotation
new Float:rx, Float:ry, Float:rz;
GetDynamicObjectRot(objectid, rx, ry, rz);

// Set object rotation
SetDynamicObjectRot(objectid, Float:rx, Float:ry, Float:rz);

// Delete an object
DeleteDynamicObject(objectid);

// Save all objects to a file
ExportMapData("mymap.json");

// Load objects from a file
ImportMapData("mymap.json");
```

## Building from Source

### Requirements
- Rust (latest stable version)
- Cargo
- Visual Studio (Windows) or GCC (Linux)

### Steps
1. Clone the repository
2. Run `cargo build --release`
3. Find the compiled plugin in `target/release/`

## Testing
Use the provided `test.pwn` script to test all plugin features.

## License

MIT License

## Author

bluffblue
