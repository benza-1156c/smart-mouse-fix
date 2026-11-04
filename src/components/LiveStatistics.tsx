import { ShieldIcon, ZapIcon, TimerIcon } from "./Icons";
import { FilterStatus } from "../types";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

interface LiveStatisticsProps {
  stats: FilterStatus;
}

export function LiveStatistics(props: LiveStatisticsProps) {
  return (
    <div class="grid grid-cols-1 gap-3 sm:grid-cols-3">
      {/* 1. Blocked Double-Clicks */}
      <Card class="gap-2 py-3">
        <CardContent class="flex flex-col gap-2 px-4">
          <div class="flex items-center justify-between">
            <span class="text-xs font-medium text-muted-foreground">
              คลิกเบิ้ลที่บล็อกได้
            </span>
            <div class="text-muted-foreground" aria-hidden="true">
              <ShieldIcon size={16} />
            </div>
          </div>
          <div class="flex items-baseline gap-1.5">
            <span class="font-mono text-2xl font-bold tracking-tight text-foreground">
              {props.stats.total_blocked}
            </span>
            <span class="font-mono text-xs text-muted-foreground">ครั้ง</span>
          </div>
          <div>
            {props.stats.total_blocked > 0 ? (
              <Badge
                variant={
                  props.stats.total_blocked > 0 ? "destructive" : "secondary"
                }
                class="font-mono text-[10px]"
              >
                สกัดการลั่นสำเร็จ
              </Badge>
            ) : null}
          </div>
          <p class="text-[11px] text-muted-foreground leading-tight">
            จำนวนครั้งที่ฮาร์ดแวร์เมาส์ส่งสัญญาณเด้งซ้ำแล้วถูกสกัดกั้น
          </p>
        </CardContent>
      </Card>

      {/* 2. Dynamic Limit */}
      <Card class="gap-2 py-3">
        <CardContent class="flex flex-col gap-2 px-4">
          <div class="flex items-center justify-between">
            <span class="text-xs font-medium text-muted-foreground">
              ขีดจำกัดปัจจุบัน
            </span>
            <div class="text-muted-foreground" aria-hidden="true">
              <ZapIcon size={16} />
            </div>
          </div>
          <div class="flex items-baseline gap-1.5">
            <span class="font-mono text-2xl font-bold tracking-tight text-foreground">
              {props.stats.current_limit.toFixed(1)}
            </span>
            <span class="font-mono text-xs text-muted-foreground">ms</span>
          </div>
          <p class="text-[11px] text-muted-foreground leading-tight">
            เกณฑ์ตัดสัญญาณเด้งที่ AI ปรับเปลี่ยนตามความเร็วมือของคุณ
          </p>
        </CardContent>
      </Card>

      {/* 3. Shortest Interval */}
      <Card class="gap-2 py-3">
        <CardContent class="flex flex-col gap-2 px-4">
          <div class="flex items-center justify-between">
            <span class="text-xs font-medium text-muted-foreground">
              ระยะห่างที่สั้นที่สุด
            </span>
            <div class="text-muted-foreground" aria-hidden="true">
              <TimerIcon size={16} />
            </div>
          </div>
          <div class="flex items-baseline gap-1.5">
            <span class="font-mono text-2xl font-bold tracking-tight text-foreground">
              {props.stats.min_interval !== null
                ? props.stats.min_interval.toFixed(1)
                : "—"}
            </span>
            <span class="font-mono text-xs text-muted-foreground">
              {props.stats.min_interval !== null ? "ms" : ""}
            </span>
          </div>
          <p class="text-[11px] text-muted-foreground leading-tight">
            ระยะห่างระหว่างคลิกที่ต่ำที่สุด ใช้ประเมินความเสื่อมของสวิตช์
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
