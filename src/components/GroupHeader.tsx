import { createContext, useContext, useEffect, useState, type Dispatch, type SetStateAction } from 'react';
import { Dropdown } from 'antd';
import type { IHeaderParams } from 'ag-grid-community';
import type { ProxyRow } from '../types';
import { MANAGER_LABEL, mapLabel } from '../lib/gridConfig';

interface GridDataContextValue {
  allRows: ProxyRow[];
  groupFilter: string | undefined;
  managerFilter: string | undefined;
  setGroupFilter: Dispatch<SetStateAction<string | undefined>>;
  setManagerFilter: Dispatch<SetStateAction<string | undefined>>;
}

export const GridDataContext = createContext<GridDataContextValue>({
  allRows: [],
  groupFilter: undefined,
  managerFilter: undefined,
  setGroupFilter: () => {},
  setManagerFilter: () => {},
});

export function matchesFilter(rowValue: string | null | undefined, filterValue: string | undefined): boolean {
  if (!filterValue) return true;
  if (!rowValue) return false;
  return rowValue.split(' | ').includes(filterValue);
}

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
    if (v) {
      for (const part of String(v).split(' | ')) {
        vals.add(part);
      }
    }
  }
  const sorted = Array.from(vals).sort();
  return [
    { key: '__all__', label: allLabel ?? 'All', checked: !currentFilter },
    ...sorted.map((v) => ({ key: v, label: mapLabel(v, labelMap ?? {}), checked: currentFilter === v })),
  ];
}

export function GroupHeader(props: IHeaderParams) {
  const [open, setOpen] = useState(false);
  const { allRows, groupFilter, managerFilter, setGroupFilter } = useContext(GridDataContext);
  const [filter, setFilter] = useState<string | undefined>(() => groupFilter);

  useEffect(() => { setFilter(groupFilter); }, [groupFilter]);

  const handleOpenChange = (next: boolean) => {
    setOpen(next);
  };

  const handleSelect = ({ key }: { key: string }) => {
    const next = key === '__all__' ? undefined : (key === filter ? undefined : key);
    setFilter(next);
    setGroupFilter(next);
    setOpen(false);
  };

  const handleSort = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    props.progressSort(e.shiftKey);
  };

  const filteredForGroup = managerFilter ? allRows.filter((r) => matchesFilter(r.manager, managerFilter)) : allRows;
  const items = collectItems(filteredForGroup, 'group_name', filter, undefined, 'All Groups');

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

export function ManagerHeader(props: IHeaderParams) {
  const [open, setOpen] = useState(false);
  const { allRows, groupFilter, managerFilter, setManagerFilter } = useContext(GridDataContext);
  const [filter, setFilter] = useState<string | undefined>(() => managerFilter);

  useEffect(() => { setFilter(managerFilter); }, [managerFilter]);

  const handleOpenChange = (next: boolean) => {
    setOpen(next);
  };

  const handleSelect = ({ key }: { key: string }) => {
    const next = key === '__all__' ? undefined : (key === filter ? undefined : key);
    setFilter(next);
    setManagerFilter(next);
    setOpen(false);
  };

  const handleSort = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    props.progressSort(e.shiftKey);
  };

  const filteredForManager = groupFilter ? allRows.filter((r) => matchesFilter(r.group_name, groupFilter)) : allRows;
  const items = collectItems(filteredForManager, 'manager', filter, MANAGER_LABEL, 'All Managers');

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
