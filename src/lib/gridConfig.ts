import type { ColDef, GridOptions } from 'ag-grid-community';
import type { ProxyRow } from '../types';
import { GroupHeader, ManagerHeader } from '../components/GroupHeader';

export const MANAGER_LABEL: Record<string, string> = {
  gpm_standard: 'GPM',
  gpm_global: 'GPM-G',
  donut: 'Donut',
};

export function mapLabel(value: string | null | undefined, labelMap: Record<string, string>): string {
  if (!value) return '';
  return value
    .split(' | ')
    .map((part) => labelMap[part] ?? part)
    .join(' | ');
}

export const managerLabel = (value?: string | null) => mapLabel(value, MANAGER_LABEL);
export const groupLabel = (value?: string | null) => mapLabel(value, MANAGER_LABEL);

export const columnDefs: ColDef<ProxyRow>[] = [
  { headerName: 'TT', valueGetter: 'node.rowIndex + 1', width: 55, pinned: 'left', sortable: false, filter: false },
  { field: 'order_code', headerName: 'Order ID', width: 140 },
  { field: 'raw_proxy', headerName: 'Proxy', width: 280, tooltipField: 'raw_proxy_ip' },
  { field: 'proxy_type', headerName: 'Type', width: 80 },
  { field: 'profile_name', headerName: 'Profile', width: 180 },
  { field: 'group_name', headerName: 'Group', width: 140, headerComponent: GroupHeader, filter: false, valueFormatter: (p) => groupLabel(p.value) },
  { field: 'manager', headerName: 'Manager', width: 140, headerComponent: ManagerHeader, valueFormatter: (p) => managerLabel(p.value) },
  { field: 'purchase_date', headerName: 'Purchased', width: 140 },
  { field: 'remaining_days', headerName: 'Days', width: 65, sort: 'asc' },
  { field: 'renewal', headerName: 'Renewal', width: 95 },
];

export const defaultGridOptions: GridOptions<ProxyRow> = {
  rowSelection: {
    mode: 'multiRow',
    enableClickSelection: true,
    checkboxes: false,
    headerCheckbox: false,
  },
  animateRows: true,
  enableCellTextSelection: false,
  suppressCellFocus: true,
  tooltipShowDelay: 300,
  defaultColDef: {
    sortable: true,
    filter: false,
    resizable: true,
  },
  getRowId: (p) => `${p.data.order_code}_${p.data.raw_proxy}`,
};
