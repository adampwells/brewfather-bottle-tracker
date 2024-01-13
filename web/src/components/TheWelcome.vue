<script setup>
import {QrcodeStream} from "vue-qrcode-reader";

function onDetect (content) {
  console.log(content)
}
</script>

<template>
<div class="container-sm">
  <qrcode-stream @detect="onDetect"></qrcode-stream>
</div>
</template>
