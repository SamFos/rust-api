<template>
  <div style="background-color: #10121894; width: 100vw; height: 100vh;">
    <NuxtPage />

    <form action="http://localhost:6969/change-state" method="POST">
      <input name="new_state" type="number" />
      <button type="submit">Submit</button>
    </form>
  </div>
</template>

<script lang="js" setup>
</script>