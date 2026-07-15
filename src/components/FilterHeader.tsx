import { useState } from 'react';
import { Dropdown } from 'antd';

interface FilterHeaderProps {
  displayName: string;
  getOptions: () => string[];
  onFilter: (val: string | undefined) => void;
}

export function FilterHeader({ displayName, getOptions, onFilter }: FilterHeaderProps) {
  const [open, setOpen] = useState(false);

  const items = [
    { key: '__all__', label: `All ${displayName}s` },
    ...getOptions().map((v: string) => ({ key: v, label: v || '(empty)' })),
  ];

  const handleSelect = ({ key }: { key: string }) => {
    onFilter(key === '__all__' ? undefined : key);
    setOpen(false);
  };

  return (
    <Dropdown
      menu={{ items, onClick: handleSelect, style: { maxHeight: 300, overflow: 'auto' } }}
      open={open}
      onOpenChange={setOpen}
      trigger={['click']}
      getPopupContainer={() => document.body}
    >
      <span style={{ cursor: 'pointer', userSelect: 'none', display: 'inline-flex', alignItems: 'center', gap: 4, height: '100%' }}>
        {displayName}
        <span style={{ fontSize: 14, lineHeight: 1, color: '#6b7280' }}>▾</span>
      </span>
    </Dropdown>
  );
}
