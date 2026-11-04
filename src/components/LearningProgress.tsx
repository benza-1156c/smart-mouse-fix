import { ActivityIcon } from "./Icons";
import { Card, CardContent } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";

interface LearningProgressProps {
  currentClicks: number;
  targetClicks: number;
  progressPercent: number;
  isComplete: boolean;
}

export function LearningProgress(props: LearningProgressProps) {

  return (
    <Card class="gap-3 py-4">
      <CardContent class="flex flex-col gap-3 px-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2 text-sm font-medium text-foreground">
            <ActivityIcon size={16} />
            <span>ความคืบหน้าการเรียนรู้ (Learning Progress)</span>
          </div>
          <div class="flex items-baseline gap-1 font-mono text-sm text-foreground">
            <span class="font-semibold">{props.currentClicks}</span>
            <span class="text-muted-foreground">/</span>
            <span class="text-muted-foreground">{props.targetClicks}</span>
            <span class="text-xs text-muted-foreground">({props.progressPercent}%)</span>
          </div>
        </div>

        <Progress value={props.progressPercent} minValue={0} maxValue={100} />

      </CardContent>
    </Card>
  );
}
