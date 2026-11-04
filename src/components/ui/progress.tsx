import { splitProps, type ComponentProps, type ValidComponent } from "solid-js";
import { Progress as ProgressPrimitive } from "@kobalte/core/progress";
import { cn } from "@/lib/utils";

export type ProgressProps<T extends ValidComponent = "div"> = ComponentProps<
  typeof ProgressPrimitive<T>
>;

export const Progress = <T extends ValidComponent = "div">(props: ProgressProps<T>) => {
  const [, rest] = splitProps(props as ProgressProps, ["class", "children"]);

  return (
    <ProgressPrimitive
      data-slot="progress"
      class={cn("flex w-full flex-col gap-2", props.class)}
      {...rest}
    >
      {props.children}
      <ProgressPrimitive.Track
        data-slot="progress-track"
        class="bg-secondary relative h-2.5 w-full overflow-hidden"
      >
        <ProgressPrimitive.Fill
          data-slot="progress-fill"
          class="bg-primary h-full transition-all"
        />
      </ProgressPrimitive.Track>
    </ProgressPrimitive>
  );
};

export type ProgressLabelProps<T extends ValidComponent = "span"> = ComponentProps<
  typeof ProgressPrimitive.Label<T>
>;

export const ProgressLabel = <T extends ValidComponent = "span">(
  props: ProgressLabelProps<T>
) => {
  const [, rest] = splitProps(props as ProgressLabelProps, ["class"]);

  return (
    <ProgressPrimitive.Label
      data-slot="progress-label"
      class={cn("text-sm font-medium select-none", props.class)}
      {...rest}
    />
  );
};

export type ProgressValueLabelProps<T extends ValidComponent = "span"> = ComponentProps<
  typeof ProgressPrimitive.ValueLabel<T>
>;

export const ProgressValueLabel = <T extends ValidComponent = "span">(
  props: ProgressValueLabelProps<T>
) => {
  const [, rest] = splitProps(props as ProgressValueLabelProps, ["class"]);

  return (
    <ProgressPrimitive.ValueLabel
      data-slot="progress-value-label"
      class={cn("text-sm font-medium select-none", props.class)}
      {...rest}
    />
  );
};

