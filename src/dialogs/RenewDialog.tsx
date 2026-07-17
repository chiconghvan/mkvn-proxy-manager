import { useState } from 'react';
import { Form, InputNumber, message, Modal, Progress } from 'antd';
import { commands } from '../lib/commands';
import type { ProxyRow } from '../types';

interface Props { open: boolean; rows: ProxyRow[]; onClose: () => void; onDone: () => void; }

const MAX_RETRIES = 3;

export function RenewDialog({ open, rows, onClose, onDone }: Props) {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState({ current: 0, total: 0 });

  const reset = () => {
    setLoading(false);
    setProgress({ current: 0, total: 0 });
  };

  const submit = async () => {
    const values = await form.validateFields();
    const orderCodes = Array.from(new Set(rows.map((r) => r.order_code)));
    if (orderCodes.length === 0) {
      message.warning('No order(s) to renew');
      return;
    }
    setProgress({ current: 0, total: orderCodes.length });
    setLoading(true);

    const errors: string[] = [];
    let completed = 0;

    for (const [i, code] of orderCodes.entries()) {
      if (i > 0) {
        await new Promise((r) => setTimeout(r, 20));
      }

      let lastErr: unknown;
      for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
        try {
          await commands.renewOrder(code, values.months);
          lastErr = undefined;
          break;
        } catch (err) {
          lastErr = err;
          if (attempt < MAX_RETRIES) {
            await new Promise((r) => setTimeout(r, 1000));
          }
        }
      }

      if (lastErr) {
        errors.push(`${code}: ${lastErr}`);
      }

      completed++;
      setProgress({ current: completed, total: orderCodes.length });
    }

    setLoading(false);

    if (errors.length === 0) {
      message.success(`Renewed ${orderCodes.length} order(s)`);
      onDone();
      onClose();
      reset();
    } else if (errors.length < orderCodes.length) {
      message.warning(`Renewed ${orderCodes.length - errors.length}/${orderCodes.length} order(s). Failed: ${errors.join('; ')}`);
      onDone();
      onClose();
      reset();
    } else if (errors.length === 1) {
      message.error(errors[0]);
      reset();
    } else {
      message.error(`All ${errors.length} renewal(s) failed: ${errors.join('; ')}`);
      reset();
    }
  };

  const handleClose = () => {
    if (loading) return;
    reset();
    onClose();
  };

  const orderCount = new Set(rows.map((r) => r.order_code)).size;
  const progressPercent = progress.total > 0 ? Math.round((progress.current / progress.total) * 100) : 0;

  return (
    <Modal title={`Renew ${orderCount} order(s)`} open={open} onOk={submit} onCancel={handleClose} confirmLoading={loading} okText="Renew" maskClosable={!loading} closable={!loading}>
      <Form form={form} layout="vertical">
        <Form.Item name="months" label="Months" initialValue={1} rules={[{ required: true, type: 'number', min: 1, max: 12 }]}>
          <InputNumber min={1} max={12} disabled={loading} />
        </Form.Item>
        {loading && (
          <Form.Item label="Progress">
            <Progress percent={progressPercent} format={() => `${progress.current} / ${progress.total}`} />
          </Form.Item>
        )}
      </Form>
    </Modal>
  );
}