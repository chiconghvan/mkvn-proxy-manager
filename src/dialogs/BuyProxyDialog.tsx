import { useEffect, useState } from 'react';
import { Form, Input, InputNumber, message, Modal, Select, Switch, Typography } from 'antd';
import { commands } from '../lib/commands';
import type { Balance, Product } from '../types';

interface Props { open: boolean; onClose: () => void; onDone: () => void; }

export function BuyProxyDialog({ open, onClose, onDone }: Props) {
  const [form] = Form.useForm();
  const [products, setProducts] = useState<Product[]>([]);
  const [balance, setBalance] = useState<Balance | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!open) return;
    Promise.all([commands.getProducts(), commands.getBalance()])
      .then(([nextProducts, nextBalance]) => { setProducts(nextProducts); setBalance(nextBalance); })
      .catch((err) => message.error(String(err)));
  }, [open]);

  const submit = async () => {
    const values = await form.validateFields();
    setLoading(true);
    try {
      const codes = await commands.buyProxy(values.product_id, values.quantity, values.renewal ?? false, values.note ?? '');
      message.success(`Bought ${codes.length} proxy order(s)`);
      onDone();
      onClose();
      form.resetFields();
    } catch (err) {
      message.error(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal title="Buy Proxy" open={open} onOk={submit} onCancel={onClose} confirmLoading={loading} okText="Buy proxy">
      <Typography.Text type="secondary">Balance: {balance ? `${balance.balance.toLocaleString()}đ` : '—'}</Typography.Text>
      <Form form={form} layout="vertical" className="dialog-form">
        <Form.Item name="product_id" label="Product" rules={[{ required: true, message: 'Choose product' }]}>
          <Select showSearch optionFilterProp="label" options={products.filter((p) => (p.store_quantity ?? 1) > 0).map((p) => ({ value: p.id_product, label: `${p.name_product} — ${(p.price ?? 0).toLocaleString()}đ — Stock ${p.store_quantity ?? 'n/a'}` }))} />
        </Form.Item>
        <Form.Item name="quantity" label="Quantity" initialValue={1} rules={[{ required: true, type: 'number', min: 1, max: 50 }]}><InputNumber min={1} max={50} /></Form.Item>
        <Form.Item name="renewal" label="Auto renewal" valuePropName="checked"><Switch /></Form.Item>
        <Form.Item name="note" label="Note"><Input.TextArea rows={3} /></Form.Item>
      </Form>
    </Modal>
  );
}
