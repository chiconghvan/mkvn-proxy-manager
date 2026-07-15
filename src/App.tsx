import { useEffect, useMemo, useState } from 'react';
import { App as AntApp, ConfigProvider, theme as antdTheme } from 'antd';
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
import type { ProxyRow } from './types';

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
  const [contextMenu, setContextMenu] = useState<{ visible: boolean; x: number; y: number; row: ProxyRow | null }>({ visible: false, x: 0, y: 0, row: null });
  const [renewRows, setRenewRows] = useState<ProxyRow[]>([]);
  const theme = settings?.theme ?? 'light';

  useEffect(() => {
    sync.loadCachedRows().then(sync.triggerSync).catch((err) => msg.error(String(err)));
  }, []);

  useEffect(() => {
    if (settings?.auto_check_update) {
      commands.checkForUpdates().then((info) => {
        if (info.update_available) {
          msg.info(`Update ${info.new_version} available! Open Settings > Update to view.`);
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
    (!groupFilter || row.group_name === groupFilter) &&
    (!managerFilter || row.manager === managerFilter)
  ), [sync.rows, groupFilter, managerFilter]);

  // context object passed to ag-Grid for GroupHeader dropdown callback
  const gridContext = useMemo(() => ({
    onGroupFilter: setGroupFilter,
    onManagerFilter: setManagerFilter,
    allRows: sync.rows,
    groupFilter,
    managerFilter,
  }), [sync.rows, groupFilter, managerFilter]);

  const renewSelected = () => { setRenewRows(selectedRows); setRenewOpen(true); };

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

  return (
    <div className="app-shell">
      <Toolbar
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
      <main className="grid-panel">
        <ProxyGrid rows={filteredRows} loading={sync.syncing} search={search} theme={theme} context={gridContext} settings={settings} onSelectionChanged={setSelectedRows} onContextMenu={handleContextMenu} />
      </main>
      <StatusBar selectedCount={selectedRows.length} totalCount={filteredRows.length} syncing={sync.syncing} progress={sync.progress} />
      <ContextMenu
        visible={contextMenu.visible} x={contextMenu.x} y={contextMenu.y} row={contextMenu.row}
        onClose={() => setContextMenu({ visible: false, x: 0, y: 0, row: null })}
        onRenew={(row) => { setRenewRows([row]); setRenewOpen(true); }}
        onToggleRenewal={async (row, enable) => { await commands.toggleRenewal(row.order_code, enable); sync.triggerSync(); }}
      />
      <BuyProxyDialog open={buyOpen} onClose={() => setBuyOpen(false)} onDone={sync.triggerSync} />
      <RenewDialog open={renewOpen} rows={renewRows.length ? renewRows : selectedRows} onClose={() => setRenewOpen(false)} onDone={sync.triggerSync} />
      <RenewalToggleDialog open={renewalOpen} rows={selectedRows} onClose={() => setRenewalOpen(false)} onDone={sync.triggerSync} />
      <SettingsDialog open={settingsOpen} onClose={() => setSettingsOpen(false)} onDone={reloadSettings} />
    </div>
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
