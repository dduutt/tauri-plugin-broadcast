# tauri-plugin-broadcast

A high-performance Android Intent Broadcast plugin for Tauri V2. This plugin allows your Tauri application to listen to system broadcasts, such as barcode scanner events, system status changes, and custom intents.

Built with **Tauri V2 IPC Channels** for reliable, point-to-point data transmission without global event namespace conflicts.

## Features

- 🚀 **High Performance**: Direct IPC Channel communication.
- 📱 **Android Native**: Full support for Android BroadcastReceiver.
- 🛠️ **Developer Friendly**: Built-in TypeScript definitions and Vant-ready UI patterns.
- 🔒 **Security First**: Integrated with Tauri V2 permission system.

## Installation

### 1. JavaScript/TypeScript (Frontend)

```bash
npm install tauri-plugin-broadcast-api
# or
pnpm add tauri-plugin-broadcast-api
```

### 2. Rust (Backend)

Add the following to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-broadcast = { git = "https://github.com/dduutt/tauri-plugin-broadcast", branch = "main" }
```

## Setup

### Plugin Registration

Register the plugin in your `src-tauri/src/lib.rs`:

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_broadcast::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Permissions

Add the permission to your capability file (e.g., `src-tauri/capabilities/default.json`):

```json
{
  "permissions": [
    "broadcast:default"
  ]
}
```

## Usage

### Listen to Broadcasts (e.g., Barcode Scanner)

```typescript
import { register, unregister } from 'tauri-plugin-broadcast-api';

const action = "com.android.server.scannerservice.seuic.scan";

// Start listening
const stop = await register(action, (event) => {
  console.log('Received broadcast:', event.action);
  console.log('Extras:', event.extras);
});

// Stop listening when needed
// await unregister(action);
```

## API

### `register(action: string, onEvent: (event: BroadcastEvent) => void): Promise<() => void>`
Registers a broadcast receiver for the specified intent action.

### `unregister(action: string): Promise<void>`
Unregisters the broadcast receiver for the specified intent action.

### Interfaces

```typescript
export interface BroadcastEvent {
  action: string;
  extras: any;
}
```

## License

MIT or Apache-2.0.
