import { useCallback, useEffect, useMemo, useRef } from 'react';
import { App as AntApp } from 'antd';
import { AgGridReact } from 'ag-grid-react';
import { AllCommunityModule, ModuleRegistry, ValidationModule, type CellContextMenuEvent, type CellDoubleClickedEvent, type ColumnResizedEvent, type GridApi, type GridReadyEvent, type SelectionChangedEvent } from 'ag-grid-community';
import { columnDefs, defaultGridOptions } from '../lib/gridConfig';
import { commands } from '../lib/commands';
import type { AppSettings, ProxyRow } from '../types';

ModuleRegistry.registerModules([AllCommunityModule, ValidationModule]);

interface Props {
  rows: ProxyRow[];
  loading: boolean;
  search: string;
  theme: string;
  context?: any;
  settings: AppSettings | null;
  onSelectionChanged: (rows: ProxyRow[]) => void;
  onContextMenu: (event: CellContextMenuEvent<ProxyRow>) => void;
}

export function ProxyGrid({ rows, loading, search, theme, context, settings, onSelectionChanged, onContextMenu }: Props) {
  const gridApi = useRef<GridApi<ProxyRow> | null>(null);
  const columnWidthsRef = useRef<Record<string, number>>({});
  const dragState = useRef<{ active: boolean; startRowIndex: number | null; additive: boolean }>({ active: false, startRowIndex: null, additive: false });
  const preDragSelection = useRef<Set<string>>(new Set());
  const { message } = AntApp.useApp();

  const restoredColumnDefs = useMemo(() => {
    if (!settings?.column_widths) return columnDefs;
    return columnDefs.map((col) => {
      if (col.field && settings.column_widths![col.field]) {
        return { ...col, width: settings.column_widths![col.field] };
      }
      return col;
    });
  }, [settings?.column_widths]);

  const handleReady = useCallback((event: GridReadyEvent<ProxyRow>) => {
    gridApi.current = event.api;
    event.api.setGridOption('quickFilterText', search);
    event.api.sizeColumnsToFit();
  }, [search]);

  const handleSelectionChanged = useCallback((event: SelectionChangedEvent<ProxyRow>) => {
    onSelectionChanged(event.api.getSelectedRows());
  }, [onSelectionChanged]);

  const handleCellDoubleClicked = useCallback((event: CellDoubleClickedEvent<ProxyRow>) => {
    if (event.value != null && event.value !== '') {
      const text = String(event.value);
      navigator.clipboard.writeText(text).catch(() => {});
      message.success(`Copied "${text}"`);

      const cellEl = ((event.event as MouseEvent).target as HTMLElement)?.closest?.('.ag-cell');
      if (cellEl) {
        cellEl.classList.add('cell-copied');
        setTimeout(() => cellEl.classList.remove('cell-copied'), 1000);
      }
    }
  }, [message]);

  const handleColumnResized = useCallback((event: ColumnResizedEvent) => {
    if (event.finished && settings) {
      const columnState = event.api.getColumnState();
      const widths: Record<string, number> = {};
      for (const state of columnState) {
        if (state.width) {
          widths[state.colId] = state.width;
        }
      }
      columnWidthsRef.current = widths;
      commands.saveSettings({ ...settings, column_widths: widths }).catch(() => {});
    }
  }, [settings]);

  useEffect(() => {
    gridApi.current?.setGridOption('quickFilterText', search);
  }, [search]);

  // Drag-to-select: mousedown on a row starts drag, mousemove selects range, mouseup ends
  const getRowIndexFromEvent = useCallback((e: React.MouseEvent): number | null => {
    const rowEl = (e.target as HTMLElement).closest<HTMLElement>('[row-index]');
    if (!rowEl) return null;
    const idx = parseInt(rowEl.getAttribute('row-index')!, 10);
    return isNaN(idx) ? null : idx;
  }, []);

  const selectRange = useCallback((from: number, to: number, additive: boolean) => {
    const api = gridApi.current;
    if (!api) return;
    // Non-additive: clear all. Additive: restore pre-drag selection first.
    if (!additive) {
      api.deselectAll();
    } else {
      api.forEachNode((node) => {
        const id = node.id ?? '';
        node.setSelected(preDragSelection.current.has(id));
      });
    }
    const lo = Math.min(from, to);
    const hi = Math.max(from, to);
    for (let i = lo; i <= hi; i++) {
      const node = api.getDisplayedRowAtIndex(i);
      if (node) node.setSelected(true);
    }
  }, []);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    // Only left button, skip if Shift held (AG-Grid handles Shift+click range)
    if (e.button !== 0 || e.shiftKey) return;
    const idx = getRowIndexFromEvent(e);
    if (idx === null) return;
    const additive = e.ctrlKey || e.metaKey;
    // Snapshot current selection for additive drag
    if (additive && gridApi.current) {
      preDragSelection.current = new Set(gridApi.current.getSelectedNodes().map(n => n.id ?? ''));
    } else {
      preDragSelection.current.clear();
    }
    dragState.current = { active: true, startRowIndex: idx, additive };
  }, [getRowIndexFromEvent]);

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (!dragState.current.active || dragState.current.startRowIndex === null) return;
    const idx = getRowIndexFromEvent(e);
    if (idx === null) return;
    selectRange(dragState.current.startRowIndex, idx, dragState.current.additive);
  }, [getRowIndexFromEvent, selectRange]);

  const handleMouseUp = useCallback(() => {
    dragState.current = { active: false, startRowIndex: null, additive: false };
  }, []);

  // End drag if mouse leaves grid
  const handleMouseLeave = useCallback(() => {
    dragState.current = { active: false, startRowIndex: null, additive: false };
  }, []);

  return (
    <div
      className={theme === 'dark' ? 'ag-theme-quartz proxy-grid proxy-grid-dark' : 'ag-theme-quartz proxy-grid'}
      onCopy={(e) => e.preventDefault()}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseLeave}
    >
      <AgGridReact<ProxyRow>
        rowData={rows}
        columnDefs={restoredColumnDefs}
        context={context}
        loading={loading}
        {...defaultGridOptions}
        onGridReady={handleReady}
        onSelectionChanged={handleSelectionChanged}
        onCellContextMenu={onContextMenu}
        onCellDoubleClicked={handleCellDoubleClicked}
        onColumnResized={handleColumnResized}
      />
    </div>
  );
}
