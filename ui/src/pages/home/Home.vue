<template>
  <q-page class="row items-center justify-center q-pa-md">
    <div class="col-6">
      <q-card dark class="text-center bg-grey-9">
        <q-card-section class="q-pb-none">
          <BannerComponent />
        </q-card-section>

        <q-separator dark inset />

        <br />
        <q-card-section class="card_font_size">
          <p>
            Aplikasi monitoring sensor tekanan udara berkecepatan tinggi melalui perangkat DTC Initium untuk
            mendukung pengujian teknologi Aero..🚀
          </p>
          <p>
            <b>Aeronitium</b> dikembangkan dengan <span class="text-red">❤️</span> oleh
            <a href="" @click.capture="onAbout" class="text-amber-9">
              <i>AEROTRONIKA</i>
            </a>.
          </p>
        </q-card-section>

        <q-card-section class="row justify-around">
          <q-input
            v-model="server"
            class="col-12 col-lg-5"
            filled
            dark
            type="text"
            label="Server"
            ref="scn_address"
            no-error-icon
            :rules="[val => !!val || 'Diperlukan']"
          />

          <q-input
            v-model="websocket"
            class="col-12 col-lg-5"
            filled
            dark
            type="text"
            label="Websocket"
            ref="scn_address"
            no-error-icon
            :rules="[val => !!val || 'Diperlukan']"
          />
        </q-card-section>
      </q-card>
    </div>
  </q-page>
</template>

<script src="./home.js" />
