<template>
  <q-page class="row justify-center q-pa-xl">
    <div class="col-6">
      <q-card class="text-center">
        <q-card-section class="q-pb-none">
          <BannerComponent />
        </q-card-section>

        <q-separator inset />

        <br />
        <h4 class="q-ma-none q-ml-md text-uppercase text-left">Konfigurasi</h4>
        <q-card-section class="card_font_size q-gutter-md row items-start">
          <q-select
            filled
            v-model="crs"
            :options="crs_options"
            label="CRS"
            dropdown-icon="fas fa-chevron-down"
            style="width: 170px;"
            hint="Cluster Rack Server"
          />

          <q-select
            filled
            v-model="num_channels"
            :options="num_channels_options"
            label="Jumlah Channel/port"
            dropdown-icon="fas fa-chevron-down"
            style="width: 220px;"
            hint="Jumlah port"
          />

          <q-input
            v-model="scn_address"
            ref="input"
            filled
            type="text"
            label="Scanner"
            hint="1, 2, atau 1-8"
            style="width: 125px;"
            no-error-icon
            :rules="[val => !!val || 'Diperlukan']"
          />

          <q-select
            filled
            v-model="lrn"
            :options="lrn_options"
            label="LRN"
            dropdown-icon="fas fa-chevron-down"
            style="width: 90px;"
            hint="lrn=1"
          />

          <q-input
            v-model="stbl"
            ref="input"
            filled
            label="STBL"
            type="number"
            hint="Setup table"
            style="width: 120px;"
            no-error-icon
            :rules="[val => !!val || 'Diperlukan']"
          />

          <q-input
            v-model="sport"
            ref="input"
            filled
            type="text"
            label="PORT"
            hint="101-164 atau 101-116, 202-208, 301-308"
            style="width: 300px;"
            no-error-icon
            :rules="[val => !!val || 'Diperlukan']"
          />
        </q-card-section>

        <q-separator inset />

        <br />
        <h4 class="q-ma-none q-ml-md text-uppercase text-left">Pengukuran</h4>
        <q-card-section class="card_font_size q-gutter-md row items-start">
          <q-input
            v-model="nfr"
            filled
            label="NFR"
            type="number"
            hint="1=no avg., default=64"
            style="width: 180px;"
            no-error-icon
            ref="input"
            :rules="[
              val => !!val || 'Diperlukan',
              val => val >= 1 || 'Min. 1',
              val => val <= 127 || 'Maks. 127'
            ]"
          />

          <q-input
            v-model="frd"
            filled
            label="FRD"
            type="number"
            hint="Frame delay"
            style="width: 250px;"
            no-error-icon
            ref="input"
            :rules="[
              val => !!val || 'Diperlukan',
              val => val >= 0 || 'Min. 0 microseconds',
              val => val <= 65000 || 'Maks. 65000 microseconds'
            ]"
          />

          <q-input
            v-model="nms"
            filled
            label="NMS"
            type="number"
            hint="Number measurement"
            style="width: 180px;"
            no-error-icon
            ref="input"
            :rules="[
              val => !!val || 'Diperlukan',
              val => val >= 0 || 'Min. 0',
              val => val <= 65000 || 'Maks. 65000'
            ]"
          />

          <q-input
            v-model="msd"
            filled
            label="MSD"
            type="number"
            hint="Milli sec. delay"
            style="width: 180px;"
            no-error-icon
            ref="input"
            :rules="[
              val => !!val || 'Diperlukan',
              val => val >= 3 || 'Min. 3 ms',
              val => val <= 1000 || 'Maks. 1000 ms'
            ]"
          />

          <q-select
            filled
            v-model="trm"
            :options="trm_options"
            label="TRM"
            dropdown-icon="fas fa-chevron-down"
            style="width: 150px;"
            hint="FREE->streamming"
          />

          <q-select
            filled
            v-model="scm"
            :options="scm_options"
            label="SCM"
            dropdown-icon="fas fa-chevron-down"
            style="width: 100px;"
            hint="Scan Mode."
          />

          <q-select
            filled
            v-model="ocf"
            :options="ocf_options"
            label="OCF"
            dropdown-icon="fas fa-chevron-down"
            style="width: 180px;"
            hint="1=raw,2=Engineer Unit"
          />

          <q-select
            filled
            v-model="unx"
            :options="unx_options"
            label="UNIT"
            dropdown-icon="fas fa-chevron-down"
            style="width: 180px;"
            hint="Engineering Unit"
          />
        </q-card-section>
      </q-card>
    </div>
  </q-page>
</template>

<script src="./configs.js" />
