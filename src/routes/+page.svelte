<script>
  import Greetings from "$lib/Greetings.svelte";
  import Performance from "$lib/Performance.svelte";
  import PerformanceMonitor from "$lib/PerformanceMonitor.svelte";
  import SystemAlert from "$lib/SystemAlert.svelte";
  import TasksSection from "$lib/TasksSection.svelte";
</script>

<body class="container mx-auto">
  <main>
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
      <!-- Main content spans 9 of 12 columns on larger screens -->
      <section class="lg:col-span-9">
        <!-- Greetings section spans full width -->
        <Greetings />
        <!-- Tasks and Performance button in a 2x1 grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <TasksSection />
          <Performance />
        </div>
        <!-- Performance Monitor and System Alerts in a 2x1 grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <PerformanceMonitor />
          <SystemAlert />
        </div>
      </section>
    </div>
  </main>
</body>
