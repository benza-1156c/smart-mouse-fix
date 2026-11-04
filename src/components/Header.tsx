import { Show } from "solid-js";
import { MouseIcon, RefreshIcon } from "./Icons";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

interface HeaderProps {
  isComplete: boolean;
  isAdaptive: boolean;
  onReset: () => void;
}

export function Header(props: HeaderProps) {
  return (
    <header class="flex items-center justify-between border-b pb-4">
      <div class="flex items-center gap-3">
        <div
          class="flex h-9 w-9 items-center justify-center border bg-card text-foreground"
          aria-hidden="true"
        >
          <MouseIcon size={18} />
        </div>
        <div>
          <div class="flex items-center gap-2">
            <h1 class="text-base font-semibold leading-none tracking-tight text-foreground">
              SmartMouseFix
            </h1>
            <Badge variant="outline" class="font-mono text-[10px]">
              v0.1.0
            </Badge>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <Show
          when={props.isComplete}
          fallback={
            <Show
              when={props.isAdaptive}
              fallback={<Badge variant="secondary">Baseline Mode</Badge>}
            >
              <Badge variant="outline">Calibrating</Badge>
            </Show>
          }
        >
          <Badge variant="default">Calibrated (Active)</Badge>
        </Show>

        <Button
          variant="outline"
          size="sm"
          onClick={props.onReset}
          type="button"
        >
          <RefreshIcon size={14} />
          <span>รีเซ็ต</span>
        </Button>
      </div>
    </header>
  );
}
