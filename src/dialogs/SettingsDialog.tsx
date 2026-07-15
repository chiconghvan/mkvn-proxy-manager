import { useEffect, useState } from 'react';
import { Button, Form, Input, InputNumber, message, Modal, Select, Space, Switch, Tabs, Typography } from 'antd';
import { commands } from '../lib/commands';
import type { AppSettings, AppUpdateInfo } from '../types';

const { Text } = Typography;

interface Props { open: boolean; onClose: () => void; onDone: () => void; }

export function SettingsDialog({ open, onClose, onDone }: Props) {
  const [form] = Form.useForm<AppSettings>();
  const [loading, setLoading] = useState(false);
  const [version, setVersion] = useState('');
  const [updateInfo, setUpdateInfo] = useState<AppUpdateInfo | null>(null);
  const [checking, setChecking] = useState(false);

  useEffect(() => {
    if (!open) return;
    commands.getSettings().then((settings) => form.setFieldsValue(settings)).catch((err) => message.error(String(err)));
    commands.getAppVersion().then(setVersion).catch(() => {});
    setUpdateInfo(null);
  }, [open, form]);

  const handleCheckUpdate = async () => {
    setChecking(true);
    setUpdateInfo(null);
    try {
      const info = await commands.checkForUpdates();
      setUpdateInfo(info);
      if (info.update_available) {
        message.info(`New version ${info.new_version} available!`);
      } else {
        message.success('You are up to date');
      }
    } catch (err) {
      message.error(String(err));
    } finally {
      setChecking(false);
    }
  };

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
          { key: 'update', label: 'Update', children: <>
            <div style={{ marginBottom: 16 }}>
              <Text>Current version: <Text strong>{version || '...'}</Text></Text>
            </div>
            <Form.Item name="auto_check_update" label="Auto check for updates on startup" valuePropName="checked">
              <Switch />
            </Form.Item>
            <Space direction="vertical" style={{ width: '100%' }}>
              <Button type="primary" onClick={handleCheckUpdate} loading={checking}>Check for updates</Button>
              {updateInfo && (
                <div style={{ marginTop: 8 }}>
                  {updateInfo.update_available ? (
                    <>
                      <Text type="success">New version {updateInfo.new_version} available!</Text>
                      <br />
                      <Text>Release notes:</Text>
                      <pre style={{ maxHeight: 200, overflow: 'auto', background: '#f5f5f5', padding: 8, borderRadius: 4, fontSize: 12, whiteSpace: 'pre-wrap', marginTop: 4 }}>
                        {updateInfo.release_notes || 'No release notes'}
                      </pre>
                      {updateInfo.release_page_url && (
                        <div style={{ marginTop: 8 }}>
                          <a href={updateInfo.release_page_url} target="_blank" rel="noreferrer">View on GitHub</a>
                        </div>
                      )}
                    </>
                  ) : (
                    <Text>You are up to date.</Text>
                  )}
                </div>
              )}
            </Space>
          </> },
        ]} />
      </Form>
    </Modal>
  );
}
