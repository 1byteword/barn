// src/components/ui/select.jsx
import { cn } from '../../lib/utils';

const Select = ({ children }) => {
  return <div className="relative">{children}</div>;
};

const SelectTrigger = ({ children, className }) => {
  return (
    <div className={cn("p-2 border border-gray-300 rounded", className)}>
      {children}
    </div>
  );
};

const SelectValue = ({ placeholder }) => {
  return <span className="text-gray-500">{placeholder}</span>;
};

export { Select, SelectTrigger, SelectValue };
