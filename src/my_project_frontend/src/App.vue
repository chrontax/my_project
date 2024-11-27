<script setup>
import { ref } from "vue";
import { my_project_backend } from "declarations/my_project_backend/index";
let chat = ref([]);

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const msg = target.querySelector("#msg").value;
  await my_project_backend.save_msg(msg);
  chat.value = await my_project_backend.get_chat();
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="msg">Enter your message: &nbsp;</label>
      <input id="msg" alt="Message" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="chat">{{ chat }}</section>
  </main>
</template>
