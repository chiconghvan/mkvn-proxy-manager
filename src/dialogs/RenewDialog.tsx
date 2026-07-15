import { useState } from 'react';
import { Form, InputNumber, message, Modal } from 'antd';
import { commands } from '../lib/commands';
import type { ProxyRow } from '../types';

interface Props { open: boolean; rows: ProxyRow[]; onClose: () => void; onDone: () => void; }

export function RenewDialog({ open, rows, onClose, onDone }: Props) {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const orderCodes = Array.from(new Set(rows.map((r) => r.order_code)));

  const submit = async () => {
    const values = await form.validateFields();
    setLoading(true);
    try {
      for (const code of orderCodes) await commands.renewOrder(code, values.months);
      message.success(`Renewed ${orderCodes.length} order(s)`);
      onDone();
      onClose();
    } catch (err) {
      message.error(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal title={`Renew ${orderCodes.length} order(s)`} open={open} onOk={submit} onCancel={onClose} confirmLoading={loading} okText="Renew">
      <Form form={form} layout="vertical">
        <Form.Item name="months" label="Months" initialValue={1} rules={[{ required: true, type: 'number', min: 1, max: 12 }]}>
          <InputNumber min={1} max={12} />
        </Form.Item>
      </Form>
    </Modal>
  );
}
