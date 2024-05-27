<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { getAvgPerformance, type Performance } from "./data";

  let avgPercent: Performance = {
    percent: 0,
    color: "green",
  };
  let intervalId: number;

  const refreshData = async () => {
    getAvgPerformance().then((data) => {
      avgPercent = data;
    });
  };

  onMount(async () => {
    refreshData();
    intervalId = setInterval(refreshData, 2000);
  });

  onDestroy(() => {
    clearInterval(intervalId);
  });
</script>

<main>
  <div class=" border border-primary card rounded-lg p-4 bg-base-200 w-80">
    <figure
      class={`flex justify-center items-center h-16 w-16 ${
        avgPercent.color === "green"
          ? "bg-success"
          : avgPercent.color === "yellow"
            ? "bg-warning"
            : avgPercent.color === "red"
              ? "bg-error"
              : ""
      } rounded-full mx-auto`}
    >
      <p class="text-success-content m-0 font-semibold">
        {avgPercent.percent}%
      </p>
    </figure>
    <div class="card-body items-center text-center">
      <h2 class="card-title">System Performance</h2>
    </div>
  </div>
</main>
