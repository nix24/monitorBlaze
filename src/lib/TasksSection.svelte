<script>
  let tasks = [
    {
      name: "Task 1",
      due: "12/12/2022",
    },
    {
      name: "Task 2",
      due: "12/12/2022",
    },
  ];
</script>

<main class="">
  <h2 class="font-bold text-2xl">My Tasks</h2>
  <div class="skeleton w-96 h-64 flex flex-col">
    <!-- task name, due at some time  -->
    <!-- will be desc and due on same line  -->
    <!-- task description -->
    <div class="flex-grow">
      {#each tasks as task}
        <div class="grid grid-cols-3 px-5 py-2">
          <div class="col-span-2">
            <p>
              Task name: <span class="font-bold text-sm">
                {task.name}
              </span>
            </p>
          </div>
          <p>Due: {task.due}</p>
        </div>
      {/each}
    </div>
    <button class="btn btn-outline mt-auto">Add Task</button>
  </div>
</main>
