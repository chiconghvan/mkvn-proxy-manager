import type { ColDef, GridOptions } from 'ag-grid-community';
import type { ProxyRow } from '../types';
import { GroupHeader, ManagerHeader } from '../components/GroupHeader';

export const managerLabel = (value?: string | null) => {
  const map: Record<string, string> = {
    gpm_standard: 'GPM Standard',
    gpm_global: 'GPM Global',
    donut: 'Donut Browser',
  };
  return value ? map[value] ?? value : '';
};

export const columnDefs: ColDef<ProxyRow>[] = [
  { headerName: 'TT', valueGetter: 'node.rowIndex + 1', width: 55, pinned: 'left', sortable: false, filter: false },
  { field: 'order_code', headerName: 'Order ID', width: 140 },
  { field: 'raw_proxy', headerName: 'Proxy', width: 280, tooltipField: 'raw_proxy_ip' },
  { field: 'proxy_type', headerName: 'Type', width: 80 },
  { field: 'profile_name', headerName: 'Profile', width: 180 },
  { field: 'group_name', headerName: 'Group', width: 140, headerComponent: GroupHeader, filter: false },
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
