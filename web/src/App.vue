<script setup>
import {RouterLink, RouterView} from 'vue-router'
</script>

<template>
  <div class="container-md">

    <nav class="navbar navbar-expand-lg bg-body-tertiary">
      <div class="container-fluid">
        <a class="navbar-brand" href="#">
          <img src="@/assets/logo.png" alt="Logo" width="30" height="30" class="d-inline-block align-text-top">
        </a>
        <div class="navbar-nav">
          <div class="container-fluid">
            <RouterLink to="/"><span class="material-icons">sports_bar</span></RouterLink>
            <RouterLink to="/settings"><span class="material-icons">settings</span></RouterLink>
            <RouterLink to="/instructions"><span class="material-icons">description</span></RouterLink>
          </div>
        </div>
      </div>
    </nav>
  </div>
  <RouterView/>
</template>

<style scoped>
header {
  line-height: 1;
  max-height: 100vh;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

nav {
  width: 100%;
  font-size: 10px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;

    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
