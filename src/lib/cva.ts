import { cva as cvaBase, type VariantProps } from "class-variance-authority";
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const cx = cn;

export const cva: typeof cvaBase = (base, config) => {
  const cvaFn = cvaBase(base, config);
  return (props) => twMerge(cvaFn(props));
};

export type { VariantProps };

