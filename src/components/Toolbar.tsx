import { Button, Input, Space, Tooltip } from 'antd';
import { CopyOutlined, ReloadOutlined, SettingOutlined, ShoppingCartOutlined, SyncOutlined } from '@ant-design/icons';
import { message } from 'antd';
import type { Balance, ProxyRow } from '../types';

interface Props {
  balance: Balance | null;
  syncing: boolean;
  selectedRows: ProxyRow[];
  search: string;
  onSearchChange: (value: string) => void;
  onReload: () => void;
  onBuy: () => void;
  onRenew: () => void;
  onToggleRenewal: (enable: boolean) => void;
  onSettings: () => void;
}

export function Toolbar(props: Props) {
  const copySelected = async () => {
    const text = props.selectedRows.map((row) => row.raw_proxy).filter(Boolean).join('\n');
    if (!text) return;
    try {
      await navigator.clipboard.writeText(text);
    } catch {
      return;
    }
    const count = props.selectedRows.length;
    message.success(`Copied ${count} ${count > 1 ? 'proxies' : 'proxy'}`);
  };

  const hasSelection = props.selectedRows.length > 0;

  return (
    <div className="toolbar-shell">
      <Space size={6}>
        <Button size="small" icon={<ReloadOutlined spin={props.syncing} />} loading={props.syncing} onClick={props.onReload}>Reload</Button>
        <Button size="small" type="primary" icon={<ShoppingCartOutlined />} onClick={props.onBuy}>Buy</Button>
        <Button size="small" icon={<CopyOutlined />} disabled={!hasSelection} onClick={copySelected}>Copy</Button>
        <Button size="small" icon={<SyncOutlined />} disabled={!hasSelection} onClick={props.onRenew}>Renew</Button>
        <Tooltip title="Enable auto renewal"><Button size="small" disabled={!hasSelection} onClick={() => props.onToggleRenewal(true)}>Ren ON</Button></Tooltip>
        <Tooltip title="Disable auto renewal"><Button size="small" disabled={!hasSelection} onClick={() => props.onToggleRenewal(false)}>Ren OFF</Button></Tooltip>
        <Button size="small" icon={<SettingOutlined />} onClick={props.onSettings} />
      </Space>
      <div className="toolbar-right">
        <Input.Search placeholder="Search..." value={props.search} onChange={(e) => props.onSearchChange(e.target.value)} allowClear className="toolbar-search" />
        <div className="balance-card">
          <span className="balance-label">Balance</span>
          <span className="balance-value">
            {props.balance
              ? `${props.balance.balance.toLocaleString()} ₫`
              : '---'}
          </span>
        </div>
      </div>
    </div>
  );
}
