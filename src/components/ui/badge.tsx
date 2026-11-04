import type { ComponentProps, ValidComponent } from "solid-js";
import { splitProps } from "solid-js";
import { Badge as BadgePrimitive } from "@kobalte/core/badge";
import type { VariantProps } from "@/lib/utils";
import { cva } from "@/lib/utils";

export const badgeVariants = cva(
  "inline-flex items-center justify-center border px-2.5 py-0.5 text-xs font-semibold w-fit whitespace-nowrap shrink-0 gap-1 transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2",
  {
    variants: {
      variant: {
        default: "border-transparent bg-primary text-primary-foreground",
        secondary: "border-transparent bg-secondary text-secondary-foreground",
        destructive: "border-transparent bg-destructive text-destructive-foreground",
        outline: "text-foreground border-border",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
);

export type BadgeProps<T extends ValidComponent = "span"> = ComponentProps<
  typeof BadgePrimitive<T>
> &
  VariantProps<typeof badgeVariants>;

export const Badge = <T extends ValidComponent = "span">(props: BadgeProps<T>) => {
  const [, rest] = splitProps(props as BadgeProps, ["class", "variant"]);

  return (
    <BadgePrimitive
      data-slot="badge"
      class={badgeVariants({
        variant: props.variant,
        class: props.class,
      })}
      {...rest}
    />
  );
};

