import { useEffect } from 'react';
import { Menu } from 'antd';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { message } from 'antd';
import type { ProxyRow } from '../types';

interface Props {
  visible: boolean;
  x: number;
  y: number;
  row: ProxyRow | null;
  onClose: () => void;
  onRenew: (row: ProxyRow) => void;
  onToggleRenewal: (row: ProxyRow, enable: boolean) => void;
}

export function ContextMenu({ visible, x, y, row, onClose, onRenew, onToggleRenewal }: Props) {
  useEffect(() => {
    if (!visible) return;
    const close = () => onClose();
    document.addEventListener('click', close);
    return () => document.removeEventListener('click', close);
  }, [visible, onClose]);

  if (!visible || !row) return null;

  return (
    <div className="context-menu" style={{ left: x, top: y }}>
      <Menu
        items={[
          { key: 'copy-proxy', label: 'Copy Proxy', onClick: async () => { await writeText(row.raw_proxy); message.success('Copied proxy'); } },
          { key: 'copy-profile', label: 'Copy Profile', disabled: !row.profile_name, onClick: async () => { if (row.profile_name) await writeText(row.profile_name); } },
          { type: 'divider' },
          { key: 'renew', label: 'Renew', onClick: () => onRenew(row) },
          { key: 'on', label: 'Renewal ON', onClick: () => onToggleRenewal(row, true) },
          { key: 'off', label: 'Renewal OFF', onClick: () => onToggleRenewal(row, false) },
        ]}
      />
    </div>
  );
}
