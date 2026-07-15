import { useState } from 'react';
import { Dropdown } from 'antd';
import type { IHeaderParams } from 'ag-grid-community';
import type { ProxyRow } from '../types';

function collectItems(
  allRows: ProxyRow[],
  field: keyof ProxyRow,
  currentFilter: string | undefined,
  labelMap?: Record<string, string>,
  allLabel?: string,
) {
  const vals = new Set<string>();
  for (const row of allRows) {
    const v = row[field];
    if (v) vals.add(String(v));
  }
  const sorted = Array.from(vals).sort();
  const mapFn = (v: string) => labelMap?.[v] ?? v;
  return [
    { key: '__all__', label: allLabel ?? 'All', checked: !currentFilter },
    ...sorted.map((v) => ({ key: v, label: mapFn(v), checked: currentFilter === v })),
  ];
}

export function GroupHeader(props: IHeaderParams) {
  const [open, setOpen] = useState(false);
  const [filter, setFilter] = useState<string | undefined>(() => props.context?.groupFilter as string | undefined);

  const handleOpenChange = (next: boolean) => {
    setOpen(next);
  };

  const handleSelect = ({ key }: { key: string }) => {
    const next = key === '__all__' ? undefined : (key === filter ? undefined : key);
    setFilter(next);
    props.context?.onGroupFilter?.(next);
    setOpen(false);
  };

  const handleSort = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    props.progressSort(e.shiftKey);
  };

  const allRows: ProxyRow[] = props.context?.allRows ?? [];
  const items = collectItems(allRows, 'group_name', filter, undefined, 'All Groups');

  return (
    <div style={{ display: 'flex', alignItems: 'center', height: '100%', gap: 4 }}>
      <span
        onClick={handleSort}
        style={{ cursor: 'pointer', userSelect: 'none', flex: 1, display: 'inline-flex', alignItems: 'center', gap: 4 }}
      >
        <span>Group</span>
        {props.column.isSortAscending() && <span className="ag-sort-indicator-icon ag-sort-ascending-icon" />}
        {props.column.isSortDescending() && <span className="ag-sort-indicator-icon ag-sort-descending-icon" />}
      </span>
      <Dropdown
        menu={{
          items: items.map((item) => ({
            key: item.key,
            label: (
              <span style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <span style={{ width: 16, textAlign: 'center' }}>{item.checked ? '✓' : ''}</span>
                {item.label}
              </span>
            ),
          })),
          onClick: handleSelect,
          style: { maxHeight: 300, overflow: 'auto' },
        }}
        open={open}
        onOpenChange={handleOpenChange}
        trigger={['click']}
        getPopupContainer={() => document.body}
      >
        <span
          onClick={(e) => e.stopPropagation()}
          style={{ cursor: 'pointer', userSelect: 'none', display: 'inline-flex', alignItems: 'center', color: filter ? '#2563eb' : '#6b7280' }}
        >
          <svg stroke="currentColor" fill="none" strokeWidth="2" viewBox="0 0 24 24" strokeLinecap="round" strokeLinejoin="round" width="1em" height="1em" xmlns="http://www.w3.org/2000/svg"><path d="m6 9 6 6 6-6"></path></svg>
        </span>
      </Dropdown>
    </div>
  );
}

const MANAGER_LABEL: Record<string, string> = {
  gpm_standard: 'GPM Standard',
  gpm_global: 'GPM Global',
  donut: 'Donut Browser',
};

export function ManagerHeader(props: IHeaderParams) {
  const [open, setOpen] = useState(false);
  const [filter, setFilter] = useState<string | undefined>(() => props.context?.managerFilter as string | undefined);

  const handleOpenChange = (next: boolean) => {
    setOpen(next);
  };

  const handleSelect = ({ key }: { key: string }) => {
    const next = key === '__all__' ? undefined : (key === filter ? undefined : key);
    setFilter(next);
    props.context?.onManagerFilter?.(next);
    setOpen(false);
  };

  const handleSort = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    props.progressSort(e.shiftKey);
  };

  const allRows: ProxyRow[] = props.context?.allRows ?? [];
  const items = collectItems(allRows, 'manager', filter, MANAGER_LABEL, 'All Managers');

  return (
    <div style={{ display: 'flex', alignItems: 'center', height: '100%', gap: 4 }}>
      <span
        onClick={handleSort}
        style={{ cursor: 'pointer', userSelect: 'none', flex: 1, display: 'inline-flex', alignItems: 'center', gap: 4 }}
      >
        <span>Manager</span>
        {props.column.isSortAscending() && <span className="ag-sort-indicator-icon ag-sort-ascending-icon" />}
        {props.column.isSortDescending() && <span className="ag-sort-indicator-icon ag-sort-descending-icon" />}
      </span>
      <Dropdown
        menu={{
          items: items.map((item) => ({
            key: item.key,
            label: (
              <span style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <span style={{ width: 16, textAlign: 'center' }}>{item.checked ? '✓' : ''}</span>
                {item.label}
              </span>
            ),
          })),
          onClick: handleSelect,
          style: { maxHeight: 300, overflow: 'auto' },
        }}
        open={open}
        onOpenChange={handleOpenChange}
        trigger={['click']}
        getPopupContainer={() => document.body}
      >
        <span
          onClick={(e) => e.stopPropagation()}
          style={{ cursor: 'pointer', userSelect: 'none', display: 'inline-flex', alignItems: 'center', color: filter ? '#2563eb' : '#6b7280' }}
        >
          <svg stroke="currentColor" fill="none" strokeWidth="2" viewBox="0 0 24 24" strokeLinecap="round" strokeLinejoin="round" width="1em" height="1em" xmlns="http://www.w3.org/2000/svg"><path d="m6 9 6 6 6-6"></path></svg>
        </span>
      </Dropdown>
    </div>
  );
}
