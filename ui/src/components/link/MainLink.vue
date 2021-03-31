<template>
  <q-item clickable v-ripple :to="path"
          :active="$route.name === nameUrl"
          :exact="$route.name !== nameUrl">
    <q-item-section avatar>
      <q-icon :name="icon" />
    </q-item-section>
    <q-tooltip
      anchor="center right"
      self="center middle"
      content-class="bg-orange"
      content-style="font-size: 16px;"
      :offset="[37, 0]"
    >
      {{ title }}
    </q-tooltip>
  </q-item>
</template>

<script src="./link.js" />
