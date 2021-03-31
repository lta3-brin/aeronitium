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
          <MainLink path="/" name-url="home" title="Utama" icon="fas fa-home" />
          <MainLink path="/configs" name-url="configs" title="Atur" icon="fas fa-cogs" />
          <MainLink path="/stream" name-url="stream" title="Stream" icon="fas fa-stream" />
        </q-list>
      </q-scroll-area>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script src="./default.js" />
