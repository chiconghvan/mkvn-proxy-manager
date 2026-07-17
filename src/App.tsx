import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Alert, App as AntApp, Button, ConfigProvider, theme as antdTheme } from 'antd';
import type { CellContextMenuEvent } from 'ag-grid-community';
import { commands } from './lib/commands';
import { Toolbar } from './components/Toolbar';
import { ProxyGrid } from './components/ProxyGrid';
import { StatusBar } from './components/StatusBar';
import { ContextMenu } from './components/ContextMenu';
import { BuyProxyDialog } from './dialogs/BuyProxyDialog';
import { RenewDialog } from './dialogs/RenewDialog';
import { RenewalToggleDialog } from './dialogs/RenewalToggleDialog';
import { SettingsDialog } from './dialogs/SettingsDialog';
import { useSync } from './hooks/useSync';
import { useSettings } from './hooks/useSettings';
import { GridDataContext, matchesFilter } from './components/GroupHeader';
import type { AppUpdateInfo, Balance, ProxyRow } from './types';

function AppContent() {
  const sync = useSync();
  const { settings, reload: reloadSettings } = useSettings();
  const { message: msg } = AntApp.useApp();
  const [selectedRows, setSelectedRows] = useState<ProxyRow[]>([]);
  const [search, setSearch] = useState('');
  const [groupFilter, setGroupFilter] = useState<string | undefined>();
  const [managerFilter, setManagerFilter] = useState<string | undefined>();
  const [buyOpen, setBuyOpen] = useState(false);
  const [renewOpen, setRenewOpen] = useState(false);
  const [renewalOpen, setRenewalOpen] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [balance, setBalance] = useState<Balance | null>(null);
  const [updateInfo, setUpdateInfo] = useState<AppUpdateInfo | null>(null);
  const [updateReady, setUpdateReady] = useState(false);
  const [updateDownloading, setUpdateDownloading] = useState(false);
  const [updateDownloadedPath, setUpdateDownloadedPath] = useState<string | null>(null);
  const [contextMenu, setContextMenu] = useState<{ visible: boolean; x: number; y: number; row: ProxyRow | null }>({ visible: false, x: 0, y: 0, row: null });
  const [contextRenewRow, setContextRenewRow] = useState<ProxyRow | null>(null);
  const theme = settings?.theme ?? 'light';

  useEffect(() => {
    sync.loadCachedRows().then(sync.triggerSync).catch((err) => msg.error(String(err)));
  }, []);

  const fetchBalance = useCallback(async () => {
    try {
      const b = await commands.getBalance();
      setBalance(b);
    } catch { /* token not configured yet */ }
  }, []);

  useEffect(() => { fetchBalance(); }, [fetchBalance]);

  useEffect(() => {
    if (sync.rows.length > 0) fetchBalance();
  }, [sync.rows, fetchBalance]);

  useEffect(() => {
    if (settings?.auto_check_update) {
      commands.checkForUpdates().then(async (info) => {
        if (info.update_available) {
          setUpdateInfo(info);
          try {
            setUpdateDownloading(true);
            const path = await commands.downloadUpdate(info.download_url);
            setUpdateDownloadedPath(path);
            setUpdateReady(true);
          } catch {
            msg.info(`Update ${info.new_version} available! Open Settings > Update to view.`);
          } finally {
            setUpdateDownloading(false);
          }
        }
      }).catch(() => {});
    }
  }, [settings?.auto_check_update]);

  useEffect(() => {
    const handler = (event: KeyboardEvent) => {
      if (event.ctrlKey && event.key.toLowerCase() === 'r') { event.preventDefault(); sync.triggerSync(); }
      if (event.ctrlKey && event.key.toLowerCase() === 'b') { event.preventDefault(); setBuyOpen(true); }
      if (event.ctrlKey && event.key === ',') { event.preventDefault(); setSettingsOpen(true); }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [sync.triggerSync]);

  const filteredRows = useMemo(() => sync.rows.filter((row) =>
    matchesFilter(row.group_name, groupFilter) &&
    matchesFilter(row.manager, managerFilter)
  ), [sync.rows, groupFilter, managerFilter]);

  const gridContextRef = useRef({}).current;

  const renewSelected = () => { setRenewOpen(true); };

  const handleRestartUpdate = async () => {
    if (!updateDownloadedPath) return;
    try {
      await commands.restartApplication(updateDownloadedPath);
    } catch (err) {
      msg.error(String(err));
    }
  };

  const toggleSelectedRenewal = async (enable: boolean) => {
    const codes = Array.from(new Set(selectedRows.map((row) => row.order_code)));
    try {
      for (const code of codes) await commands.toggleRenewal(code, enable);
      msg.success(`Renewal ${enable ? 'enabled' : 'disabled'} for ${codes.length} order(s)`);
      sync.triggerSync();
    } catch (err) { msg.error(String(err)); }
  };

  const handleContextMenu = (event: CellContextMenuEvent<ProxyRow>) => {
    event.event?.preventDefault();
    const mouse = event.event as MouseEvent;
    setContextMenu({ visible: true, x: mouse.clientX, y: mouse.clientY, row: event.data ?? null });
  };

  const gridDataValue = useMemo(() => ({
    allRows: sync.rows,
    groupFilter,
    managerFilter,
    setGroupFilter,
    setManagerFilter,
  }), [sync.rows, groupFilter, managerFilter]);

  return (
    <GridDataContext.Provider value={gridDataValue}>
      <div className="app-shell">
        <Toolbar
          balance={balance}
          syncing={sync.syncing}
          selectedRows={selectedRows}
          search={search}
          onSearchChange={setSearch}
          onReload={sync.triggerSync}
          onBuy={() => setBuyOpen(true)}
          onRenew={renewSelected}
          onToggleRenewal={toggleSelectedRenewal}
          onSettings={() => setSettingsOpen(true)}
        />
        {updateReady && (
          <Alert
            type="success"
            showIcon
            message={`Update ${updateInfo?.new_version ?? ''} ready!`}
            description="Download complete. Restart to install the update."
            action={
              <Button size="small" type="primary" danger onClick={handleRestartUpdate}>
                Restart Now
              </Button>
            }
            closable
            onClose={() => setUpdateReady(false)}
            style={{ margin: '0 14px', borderRadius: 8 }}
          />
        )}
        <main className="grid-panel">
          <ProxyGrid rows={filteredRows} loading={sync.syncing} search={search} theme={theme} context={gridContextRef} settings={settings} onSelectionChanged={setSelectedRows} onContextMenu={handleContextMenu} />
        </main>
      <StatusBar selectedCount={selectedRows.length} totalCount={filteredRows.length} syncing={sync.syncing} progress={sync.progress} />
      <ContextMenu
        visible={contextMenu.visible} x={contextMenu.x} y={contextMenu.y} row={contextMenu.row}
        onClose={() => setContextMenu({ visible: false, x: 0, y: 0, row: null })}
        onRenew={(row) => { setContextRenewRow(row); setRenewOpen(true); }}
        onToggleRenewal={async (row, enable) => { await commands.toggleRenewal(row.order_code, enable); sync.triggerSync(); }}
      />
      <BuyProxyDialog open={buyOpen} onClose={() => setBuyOpen(false)} onDone={sync.triggerSync} />
      <RenewDialog open={renewOpen} rows={contextRenewRow ? [contextRenewRow] : selectedRows} onClose={() => { setRenewOpen(false); setContextRenewRow(null); }} onDone={sync.triggerSync} />
      <RenewalToggleDialog open={renewalOpen} rows={selectedRows} onClose={() => setRenewalOpen(false)} onDone={sync.triggerSync} />
      <SettingsDialog open={settingsOpen} onClose={() => setSettingsOpen(false)} onDone={reloadSettings} />
      </div>
    </GridDataContext.Provider>
  );
}

export default function App() {
  const [appSettings, setAppSettings] = useState(() => {
    try {
      const raw = document.cookie.match(/theme=(\w+)/)?.[1];
      return raw === 'dark' ? antdTheme.darkAlgorithm : antdTheme.defaultAlgorithm;
    } catch { return antdTheme.defaultAlgorithm; }
  });

  useEffect(() => {
    const interval = setInterval(() => {
      const el = document.documentElement.dataset.theme;
      setAppSettings(() => el === 'dark' ? antdTheme.darkAlgorithm : antdTheme.defaultAlgorithm);
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <ConfigProvider theme={{ algorithm: appSettings, token: { colorPrimary: '#2563eb', borderRadius: 6, fontFamily: 'Inter, Segoe UI, Arial, sans-serif' } }}>
      <AntApp>
        <AppContent />
      </AntApp>
    </ConfigProvider>
  );
}
