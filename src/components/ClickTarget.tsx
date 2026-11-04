import { For } from "solid-js";
import { MouseIcon } from "./Icons";
import { Badge } from "@/components/ui/badge";
import { Ripple } from "../types";

interface ClickTargetProps {
  isComplete: boolean;
  isClickActive: boolean;
  isBlockedFlash: boolean;
  totalClicks: number;
  lastInterval: number | null;
  ripples: Ripple[];
  onMouseDown: (e: MouseEvent) => void;
}

export function ClickTarget(props: ClickTargetProps) {
  return (
    <div class="flex flex-col">
      <div
        class={`relative flex h-52 w-full cursor-pointer flex-col items-center justify-center gap-2 overflow-hidden border-2 border-dashed p-4 text-center transition-colors select-none ${
          props.isBlockedFlash
            ? "border-destructive bg-destructive/10"
            : props.isComplete
            ? "border-primary bg-card"
            : "border-border bg-card hover:border-primary/60 hover:bg-muted/30"
        } ${props.isClickActive ? "scale-[0.99]" : ""}`}
        onMouseDown={props.onMouseDown}
        role="button"
        tabIndex={0}
        aria-label="Click calibration target"
      >
        <For each={props.ripples}>
          {(ripple) => (
            <span
              class="ripple"
              style={{
                top: `${ripple.y}px`,
                left: `${ripple.x}px`,
                width: `${ripple.size}px`,
                height: `${ripple.size}px`,
              }}
            />
          )}
        </For>

        <div class="flex h-12 w-12 items-center justify-center border bg-background text-muted-foreground" aria-hidden="true">
          <MouseIcon size={24} />
        </div>

        <div class="text-sm font-semibold tracking-tight text-foreground">
          {props.isComplete ? "คลิกเพื่อทดสอบการตอบสนองต่อเนื่อง" : "คลิกซ้ายในพื้นที่นี้"}
        </div>

        <p class="max-w-md text-xs text-muted-foreground leading-normal">
          คลิกตามจังหวะธรรมชาติของคุณอย่างต่อเนื่องโดยไม่ต้องขยับเมาส์
          ระบบ Low-level Hook จะดักจับจังหวะสวิตช์อัตโนมัติ
        </p>

        <div class="mt-1 flex items-center gap-2">
          <Badge variant="secondary" class="font-mono text-xs">
            คลิกสะสม: {props.totalClicks} ครั้ง
          </Badge>
          <Badge variant="outline" class="font-mono text-xs">
            ระยะล่าสุด:{" "}
            {props.lastInterval !== null ? `${props.lastInterval.toFixed(1)} ms` : "พร้อมคลิก"}
          </Badge>
        </div>
      </div>
    </div>
  );
}
