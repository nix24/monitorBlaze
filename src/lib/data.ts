import { invoke } from "@tauri-apps/api/tauri";

//reminder, use svelte-chart and chartjs in performanceMonitor.svelte
//here invoke functions from rust backend to get data from rust backend
export type ChartData = {
  labels: string[];
  datasets: Array<{
    label: string;
    data: number[];
    backgroundColor: string[];
    borderWidth: number;
    borderColor: string[];
  }>;
};

export type Performance = {
  percent: number;
  color: string;
};

export async function getChartData(): Promise<ChartData> {
  const monitoringData: ChartData = await invoke("get_sys_metrics");
  return monitoringData;
}

//function to get avg performance data from rust
export async function getAvgPerformance(): Promise<Performance> {
  const sysMetrics: ChartData = await invoke("get_sys_metrics");
  const cpuUsage = sysMetrics.datasets[0].data[0];
  const ramUsage = sysMetrics.datasets[0].data[1];
  const storageUsage = sysMetrics.datasets[0].data[2];

  const avgPerformance: Performance = await invoke(
    "calculate_sys_performance",
    {
      cpuUsage,
      ramUsage,
      storageUsage,
    }
  );
  return avgPerformance;
}
