export interface FilterStatus {
  total_clicks: number;
  total_blocked: number;
  buffer_count: number;
  capacity: number;
  is_warmed_up: boolean;
  is_adaptive: boolean;
  current_limit: number;
  min_interval: number | null;
  last_interval: number | null;
}

export interface Ripple {
  id: number;
  x: number;
  y: number;
  size: number;
}

