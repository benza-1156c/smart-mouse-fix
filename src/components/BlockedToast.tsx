import { Show } from "solid-js";
import { AlertCircleIcon } from "./Icons";
import { Alert, AlertTitle } from "@/components/ui/alert";

interface BlockedToastProps {
  message: string | null;
}

export function BlockedToast(props: BlockedToastProps) {
  return (
    <Show when={props.message}>
      <Alert variant="destructive">
        <AlertCircleIcon size={16} />
        <AlertTitle>{props.message}</AlertTitle>
      </Alert>
    </Show>
  );
}
