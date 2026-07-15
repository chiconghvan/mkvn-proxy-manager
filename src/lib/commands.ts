import { invoke } from '@tauri-apps/api/core';
import type { AppSettings, Balance, Product, ProxyRow, SyncResult } from '../types';

export const commands = {
  syncAll: () => invoke<SyncResult>('sync_all'),
  getProxyRows: () => invoke<ProxyRow[]>('get_proxy_rows'),
  getCachedRows: () => invoke<ProxyRow[]>('get_cached_rows'),
  buyProxy: (productId: number, quantity: number, renewal: boolean, note: string) =>
    invoke<string[]>('buy_proxy', { productId, quantity, renewal, note }),
  renewOrder: (orderCode: string, months: number) =>
    invoke<void>('renew_order', { orderCode, months }),
  toggleRenewal: (orderCode: string, enable: boolean) =>
    invoke<void>('toggle_renewal', { orderCode, enable }),
  getProducts: () => invoke<Product[]>('get_products'),
  getBalance: () => invoke<Balance>('get_balance'),
  getSettings: () => invoke<AppSettings>('get_settings'),
  saveSettings: (settings: AppSettings) => invoke<void>('save_settings', { settings }),
};
