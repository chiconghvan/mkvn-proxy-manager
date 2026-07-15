import { useCallback, useEffect, useState } from 'react';
import { commands } from '../lib/commands';
import { useEventListener } from './useEventListener';
import type { AppSettings } from '../types';

export function useSettings() {
  const [settings, setSettings] = useState<AppSettings | null>(null);

  const load = useCallback(async () => {
    const next = await commands.getSettings();
    setSettings(next);
    document.documentElement.dataset.theme = next.theme;
  }, []);

  useEffect(() => {
    load();
  }, [load]);

  useEventListener<AppSettings>('settings_changed', (payload) => {
    setSettings(payload);
    document.documentElement.dataset.theme = payload.theme;
  });

  return { settings, reload: load };
}
