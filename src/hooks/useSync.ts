import { useCallback, useState } from 'react';
import { commands } from '../lib/commands';
import { useEventListener } from './useEventListener';
import type { ProxyRow, SyncProgress } from '../types';

export function useSync() {
  const [rows, setRows] = useState<ProxyRow[]>([]);
  const [syncing, setSyncing] = useState(false);
  const [progress, setProgress] = useState<SyncProgress | null>(null);
  const [lastError, setLastError] = useState<string | null>(null);

  const refreshRows = useCallback(async () => {
    const nextRows = await commands.getProxyRows();
    setRows(nextRows);
  }, []);

  const loadCachedRows = useCallback(async () => {
    const cached = await commands.getCachedRows();
    setRows(cached);
  }, []);

  const triggerSync = useCallback(async () => {
    setSyncing(true);
    setLastError(null);
    try {
      const result = await commands.syncAll();
      await refreshRows();
      setProgress({
        message: result.errors.length ? `Sync completed with ${result.errors.length} issue(s)` : 'Synchronization completed',
        current: 100,
        total: 100,
      });
    } catch (err) {
      const text = String(err);
      setLastError(text);
      setProgress({ message: text, current: 0, total: 100 });
    } finally {
      setSyncing(false);
    }
  }, [refreshRows]);

  useEventListener<SyncProgress>('sync_started', (payload) => {
    setSyncing(true);
    setProgress(payload);
  });
  useEventListener<SyncProgress>('sync_progress', setProgress);
  useEventListener('sync_completed', () => {
    setSyncing(false);
    refreshRows();
  });
  useEventListener('database_updated', refreshRows);

  return { rows, syncing, progress, lastError, loadCachedRows, refreshRows, triggerSync };
}
