<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let greeting = "";

  async function getGreeting() {
    greeting = await invoke("get_greeting");
  }

  getGreeting();
</script>

<main class="my-5">
  <!-- if no greeting -->
  {#if !greeting}
    <p>Loading...</p>
  {/if}
  <h2 class="font-bold text-3xl">{greeting}</h2>
  <p>Check your real-time metrics and performance.</p>
</main>
