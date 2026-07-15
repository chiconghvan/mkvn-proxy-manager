import { Progress, Space, Typography } from 'antd';
import type { SyncProgress } from '../types';

interface Props {
  selectedCount: number;
  totalCount: number;
  syncing: boolean;
  progress: SyncProgress | null;
}

export function StatusBar({ selectedCount, totalCount, syncing, progress }: Props) {
  const percent = progress && progress.total > 0 ? Math.min(100, Math.round((progress.current / progress.total) * 100)) : 0;
  return (
    <div className="status-bar">
      <Typography.Text>{selectedCount ? `${selectedCount} selected / ` : ''}{totalCount} proxies</Typography.Text>
      {syncing && progress ? (
        <Space>
          <Typography.Text type="secondary">{progress.message}</Typography.Text>
          <Progress percent={percent} size="small" className="status-progress" />
        </Space>
      ) : <Typography.Text type="secondary">Ready</Typography.Text>}
    </div>
  );
}
