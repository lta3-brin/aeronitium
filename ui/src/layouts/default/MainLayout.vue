<template>
  <q-layout view="lHh Lpr lFf">
    <q-drawer
      show-if-above
      :mini="miniState"
      @click.capture="drawerClick"

      :breakpoint="500"
      content-class="bg-black text-white"
    >
      <q-scroll-area class="fit">
        <q-list padding>
          <q-item clickable v-ripple :active="$route.name === 'home'">
            <q-item-section avatar>
              <q-icon name="home" />
            </q-item-section>
          </q-item>

          <q-item clickable v-ripple>
            <q-item-section avatar>
              <q-icon name="star" />
            </q-item-section>
          </q-item>

          <q-item clickable v-ripple>
            <q-item-section avatar>
              <q-icon name="send" />
            </q-item-section>
          </q-item>

          <q-item clickable v-ripple>
            <q-item-section avatar>
              <q-icon name="drafts" />
            </q-item-section>
          </q-item>
        </q-list>
      </q-scroll-area>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script src="./default.js" />
