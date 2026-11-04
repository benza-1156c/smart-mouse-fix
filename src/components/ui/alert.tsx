import type { ComponentProps, ValidComponent } from "solid-js";
import { splitProps } from "solid-js";
import { Alert as AlertPrimitive } from "@kobalte/core/alert";
import type { VariantProps } from "@/lib/utils";
import { cva, cn } from "@/lib/utils";

export const alertVariants = cva(
  "relative w-full border px-4 py-3 text-sm grid has-[>svg]:grid-cols-[1rem_1fr] grid-cols-[0_1fr] has-[>svg]:gap-x-3 gap-y-0.5 items-start [&>svg]:text-current",
  {
    variants: {
      variant: {
        default: "bg-background text-foreground",
        destructive: "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
);

export type AlertProps<T extends ValidComponent = "div"> = ComponentProps<
  typeof AlertPrimitive<T>
> &
  VariantProps<typeof alertVariants>;

export const Alert = <T extends ValidComponent = "div">(props: AlertProps<T>) => {
  const [, rest] = splitProps(props as AlertProps, ["class", "variant"]);

  return (
    <AlertPrimitive
      data-slot="alert"
      class={alertVariants({
        variant: props.variant,
        class: props.class,
      })}
      {...rest}
    />
  );
};

export type AlertTitleProps = ComponentProps<"div">;

export const AlertTitle = (props: AlertTitleProps) => {
  const [, rest] = splitProps(props, ["class"]);

  return (
    <div
      data-slot="alert-title"
      class={cn("col-start-2 line-clamp-1 min-h-4 font-medium tracking-tight", props.class)}
      {...rest}
    />
  );
};

export type AlertDescriptionProps = ComponentProps<"div">;

export const AlertDescription = (props: AlertDescriptionProps) => {
  const [, rest] = splitProps(props, ["class"]);

  return (
    <div
      data-slot="alert-description"
      class={cn("text-muted-foreground col-start-2 grid justify-items-start gap-1 text-sm [&_p]:leading-relaxed", props.class)}
      {...rest}
    />
  );
};

