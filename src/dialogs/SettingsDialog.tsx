import { useEffect, useState } from 'react';
import { Form, Input, InputNumber, message, Modal, Select, Switch, Tabs } from 'antd';
import { commands } from '../lib/commands';
import type { AppSettings } from '../types';

interface Props { open: boolean; onClose: () => void; onDone: () => void; }

export function SettingsDialog({ open, onClose, onDone }: Props) {
  const [form] = Form.useForm<AppSettings>();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!open) return;
    commands.getSettings().then((settings) => form.setFieldsValue(settings)).catch((err) => message.error(String(err)));
  }, [open, form]);

  const submit = async () => {
    const values = await form.validateFields();
    setLoading(true);
    try {
      await commands.saveSettings(values);
      message.success('Settings saved');
      onDone();
      onClose();
    } catch (err) {
      message.error(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal title="Settings" open={open} onOk={submit} onCancel={onClose} confirmLoading={loading} width={720} okText="Save settings">
      <Form form={form} layout="vertical">
        <Tabs items={[
          { key: 'api', label: 'MKVN', children: <Form.Item name="mkvn_token" label="MKVN Token"><Input.Password /></Form.Item> },
          { key: 'managers', label: 'Managers', children: <>
            <Form.Item name="gpm_standard_enabled" label="Enable GPM Standard" valuePropName="checked"><Switch /></Form.Item>
            <Form.Item name="gpm_standard_url" label="GPM Standard URL"><Input /></Form.Item>
            <Form.Item name="gpm_global_enabled" label="Enable GPM Global" valuePropName="checked"><Switch /></Form.Item>
            <Form.Item name="gpm_global_url" label="GPM Global URL"><Input /></Form.Item>
            <Form.Item name="donut_enabled" label="Enable Donut Browser" valuePropName="checked"><Switch /></Form.Item>
            <Form.Item name="donut_url" label="Donut Browser URL"><Input /></Form.Item>
          </> },
          { key: 'general', label: 'General', children: <>
            <Form.Item name="auto_sync_interval_secs" label="Auto sync interval (seconds)"><InputNumber min={60} max={3600} /></Form.Item>
            <Form.Item name="theme" label="Theme"><Select options={[{ value: 'light', label: 'Light' }, { value: 'dark', label: 'Dark' }]} /></Form.Item>
          </> },
        ]} />
      </Form>
    </Modal>
  );
}
