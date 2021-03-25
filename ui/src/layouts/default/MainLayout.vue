<template>
  <q-layout view="lHh Lpr lFf">
    <q-drawer
      show-if-above
      :mini="miniState"
      @click.capture="drawerClick"

      :breakpoint="500"
      content-class="bg-grey-10 text-white"
    >
      <q-scroll-area class="fit">
        <q-list padding>
          <q-item clickable v-ripple :active="$route.name === 'home'">
            <q-item-section avatar>
              <q-icon name="fas fa-home" />
            </q-item-section>
            <q-tooltip
              anchor="center right"
              self="center middle"
              content-class="bg-orange"
              content-style="font-size: 16px;"
              :offset="[37, 0]"
            >
              Utama
            </q-tooltip>
          </q-item>

          <q-item clickable v-ripple>
            <q-item-section avatar>
              <q-icon name="fas fa-cogs" />
            </q-item-section>

            <q-tooltip
              anchor="center right"
              self="center left"
              content-class="bg-orange"
              content-style="font-size: 16px;"
              :offset="[5, 0]"
            >
              Pengaturan
            </q-tooltip>
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
