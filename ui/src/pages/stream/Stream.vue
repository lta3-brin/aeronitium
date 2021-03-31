<template>
  <q-page class="row items-center justify-center">
    <div class="col-12 q-px-md">
      <q-card dark class="text-center bg-grey-9">
        <q-card-section class="q-pb-none">
          <BannerComponent />
        </q-card-section>

        <q-separator dark inset />

        <q-card-section>
          <Plotly :data="data" :layout="layout" :displaylogo="false" />
        </q-card-section>
      </q-card>
    </div>
  </q-page>
</template>

<script src="./streaming.js" />
