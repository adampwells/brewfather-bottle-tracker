<script setup>
import * as wasm from "brewfather-bottle-tracker";

function do_stuff() {
  let resp = wasm.store_bottle_ids(5.24, 2.2, [1,3,5,7,55])
  console.log(resp.measured_ph)
  console.log(resp.fermenter_top_up)
  let r = wasm.read_bottle_ids(resp.measured_ph, resp.fermenter_top_up)
  console.log(r)
}

defineProps({
  msg: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="greetings">
    <h1 class="green">{{ msg }}</h1>
    <h3>
      You’ve successfully created a project with
      <button @click="do_stuff">Click Me</button>
      <a href="https://vitejs.dev/" target="_blank" rel="noopener">Vite</a> +
      <a href="https://vuejs.org/" target="_blank" rel="noopener">Vue 3</a>.
    </h3>
  </div>
</template>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  position: relative;
  top: -10px;
}

h3 {
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

@media (min-width: 1024px) {
  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}
</style>
