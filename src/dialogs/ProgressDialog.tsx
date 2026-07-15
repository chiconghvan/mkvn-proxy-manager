import { Modal, Progress, Typography } from 'antd';
import type { SyncProgress } from '../types';

interface Props { open: boolean; progress: SyncProgress | null; }

export function ProgressDialog({ open, progress }: Props) {
  const percent = progress && progress.total > 0 ? Math.round((progress.current / progress.total) * 100) : 0;
  return (
    <Modal title="Synchronization" open={open} footer={null} closable={false}>
      <Typography.Paragraph>{progress?.message ?? 'Working'}</Typography.Paragraph>
      <Progress percent={percent} />
    </Modal>
  );
}
