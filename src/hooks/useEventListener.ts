import { useEffect } from 'react';
import { listen, type Event } from '@tauri-apps/api/event';

export function useEventListener<T>(eventName: string, handler: (payload: T) => void) {
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    listen<T>(eventName, (event: Event<T>) => handler(event.payload)).then((fn) => {
      unlisten = fn;
    });
    return () => {
      if (unlisten) unlisten();
    };
  }, [eventName, handler]);
}
