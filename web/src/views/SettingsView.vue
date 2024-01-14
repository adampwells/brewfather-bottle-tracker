<script setup>
import {ref, onMounted} from "vue";

import api from "@/js/brewfather.js"

let brewFatherKey = ref('')
let brewFatherId = ref('')

onMounted(() => {
      brewFatherKey.value = localStorage.getItem('brewfatherKey')
      brewFatherId.value = localStorage.getItem('brewfatherId')
      console.log('Loaded Brewfather Credentials' + brewFatherKey.value + ' ' + brewFatherId.value)
    }
)

function saveCredentials() {
  console.log('Saving Brewfather Credentials')
  localStorage.setItem('brewfatherKey', brewFatherKey.value)
  localStorage.setItem('brewfatherId', brewFatherId.value)
}

async function loadBatches() {
  api.getConditioningBatches().then((data) => {
    console.log(data)
  })
}

</script>
<template>
  <div class="container-md">
    <h1>Settings</h1>
    <div class="mb-3">
      <label for="exampleInputEmail1" class="form-label">Brewfather User ID</label>
      <input type="text" class="form-control" id="exampleInputEmail1" v-model="brewFatherId">
    </div>
    <div class="mb-3">
      <label for="exampleInputPassword1" class="form-label">Brewfather API Key</label>
      <input type="password" class="form-control" id="exampleInputPassword1" v-model="brewFatherKey">
    </div>
    <div id="emailHelp" class="form-text">This data is only stored locally on your device - it is only ever used directly between this device and Brewfather.</div>
    <br>
    <button class="btn btn-primary" @click="saveCredentials">Save</button>
    <button class="btn btn-secondary" @click="loadBatches">Load Batches</button>
  </div>
</template>

<style>
</style>
