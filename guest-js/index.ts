import { invoke, Channel } from '@tauri-apps/api/core'

export interface BroadcastEvent {
  action: string
  extras: any
}

export async function register(action: string, onEvent: (event: BroadcastEvent) => void) {
  const handler = new Channel<BroadcastEvent>();
  handler.onmessage = (event) => {
    onEvent(event);
  };
  
  try {
    await invoke('plugin:broadcast|register', {
        action,
        handler
    });
  } catch (e) {
    throw e;
  }

  return () => {
    // 基础清理，具体注销应调用 unregister
  };
}

export async function unregister(action: string): Promise<void> {
  await invoke('plugin:broadcast|unregister', {
    payload: { action }
  });
}
