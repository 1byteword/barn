// src/components/ui/button.jsx
import { cn } from '../../lib/utils';

const Button = ({ children, className, ...props }) => {
  return (
    <button className={cn("p-2 bg-blue-500 hover:bg-blue-700 text-white", className)} {...props}>
      {children}
    </button>
  );
};

export { Button };
