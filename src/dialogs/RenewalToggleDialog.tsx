import { useState } from 'react';
import { message, Modal, Radio, Typography } from 'antd';
import { commands } from '../lib/commands';
import type { ProxyRow } from '../types';

interface Props { open: boolean; rows: ProxyRow[]; onClose: () => void; onDone: () => void; }

export function RenewalToggleDialog({ open, rows, onClose, onDone }: Props) {
  const [target, setTarget] = useState(true);
  const [loading, setLoading] = useState(false);
  const orderCodes = Array.from(new Set(rows.map((r) => r.order_code)));

  const submit = async () => {
    setLoading(true);
    try {
      for (const code of orderCodes) await commands.toggleRenewal(code, target);
      message.success(`Renewal ${target ? 'enabled' : 'disabled'} for ${orderCodes.length} order(s)`);
      onDone();
      onClose();
    } catch (err) {
      message.error(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal title="Set renewal" open={open} onOk={submit} onCancel={onClose} confirmLoading={loading} okText="Apply">
      <Typography.Paragraph>{orderCodes.length} selected order(s)</Typography.Paragraph>
      <Radio.Group value={target} onChange={(e) => setTarget(e.target.value)}>
        <Radio value={true}>Renewal ON</Radio>
        <Radio value={false}>Renewal OFF</Radio>
      </Radio.Group>
    </Modal>
  );
}
