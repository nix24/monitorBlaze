<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Bar } from "svelte-chartjs";
  import {
    Chart,
    Title,
    Tooltip,
    Legend,
    BarElement,
    CategoryScale,
    LinearScale,
  } from "chart.js";
  import { getChartData } from "./data";
  import type { ChartData } from "./data";

  export let monitoringData: ChartData;
  let intervalId: number;

  const refreshData = async () => {
    getChartData().then((data) => {
      monitoringData = data;
    });
  };

  onMount(async () => {
    refreshData();
    intervalId = setInterval(refreshData, 2000);
  });

  onDestroy(() => {
    clearInterval(intervalId);
  });

  Chart.register(
    Title,
    Tooltip,
    Legend,
    BarElement,
    CategoryScale,
    LinearScale
  );
</script>

<main>
  <p>Performance Monitor</p>

  <!-- Wrapper for the Performance Monitor section -->
  <div class="flex items-end space-x-2 bg-base-200 rounded-md p-4">
    <Bar
      data={monitoringData}
      options={{
        responsive: true,
        animation: false,
      }}
    />
  </div>
</main>

<style>
</style>
