<template>
  <q-page class="row items-center justify-center q-pa-xl">
    <div class="col-6">
      <q-card dark class="text-center bg-grey-9">
        <q-card-section class="q-pb-none">
          <BannerComponent />
        </q-card-section>

        <q-separator dark inset />

        <br />
        <h4 class="q-ma-none q-ml-md text-uppercase text-left">Konfigurasi</h4>
        <q-card-section class="card_font_size q-gutter-md row items-start">
          <q-select
            filled
            dark
            v-model="crs"
            :options="crs_options"
            label="CRS"
            dropdown-icon="fas fa-chevron-down"
            style="width: 170px;"
            hint="Cluster Rack Server"
          />

          <q-select
            filled
            dark
            v-model="num_channels"
            :options="num_channels_options"
            label="Jumlah Channel/port"
            dropdown-icon="fas fa-chevron-down"
            style="width: 220px;"
            hint="Jumlah port"
          />

          <q-input
            v-model="scn_address"
            filled
            dark
            type="text"
            label="Scanner"
            hint="1, 2, atau 1-8"
            style="width: 125px;"
            ref="scn_address"
            no-error-icon
            :rules="[val => !!val || 'Diperlukan']"
          />

          <q-select
            filled
            dark
            v-model="lrn"
            :options="lrn_options"
            label="LRN"
            dropdown-icon="fas fa-chevron-down"
            style="width: 90px;"
            hint="lrn=1"
          />

          <q-input
            v-model="stbl"
            filled
            dark
            label="STBL"
            type="number"
            hint="Setup table"
            style="width: 120px;"
            ref="stbl"
            no-error-icon
            :rules="[
              val => !!val || 'Diperlukan',
              val => val >= 1 || 'Min. 1'
            ]"
          />

          <q-input
            v-model="sport"
            filled
            dark
            type="text"
            label="PORT"
            hint="101-164 atau 101-116, 202-208, 301-308"
            style="width: 300px;"
            ref="sport"
            no-error-icon
            :rules="[val => !!val || 'Diperlukan']"
          />

          <q-select
            filled
            dark
            v-model="scn_number"
            :options="scn_number_options"
            label="No. Scanner"
            dropdown-icon="fas fa-chevron-down"
            style="width: 150px;"
            hint="Status scanner"
          />
        </q-card-section>

        <q-separator dark inset />

        <br />
        <h4 class="q-ma-none q-ml-md text-uppercase text-left">Pengukuran</h4>
        <q-card-section class="card_font_size q-gutter-md row items-start">
          <q-input
            v-model="nfr"
            filled
            dark
            label="NFR"
            type="number"
            hint="1=no avg., default=64"
            style="width: 180px;"
            no-error-icon
            ref="nfr"
            :rules="[
              val => !!val || 'Diperlukan',
              val => val >= 1 || 'Min. 1',
              val => val <= 127 || 'Maks. 127'
            ]"
          />

          <q-input
            v-model="frd"
            filled
            dark
            label="FRD"
            type="number"
            hint="Frame delay"
            style="width: 250px;"
            no-error-icon
            ref="frd"
            :rules="[
              val => !!val || 'Diperlukan',
              val => val >= 0 || 'Min. 0 microseconds',
              val => val <= 65000 || 'Maks. 65000 microseconds'
            ]"
          />

          <q-input
            v-model="nms"
            filled
            dark
            label="NMS"
            type="number"
            hint="Number measurement"
            style="width: 180px;"
            no-error-icon
            ref="nms"
            :rules="[
              val => !!val || 'Diperlukan',
              val => val >= 0 || 'Min. 0',
              val => val <= 65000 || 'Maks. 65000'
            ]"
          />

          <q-input
            v-model="msd"
            filled
            dark
            label="MSD"
            type="number"
            hint="Milli sec. delay"
            style="width: 180px;"
            no-error-icon
            ref="msd"
            :rules="[
              val => !!val || 'Diperlukan',
              val => val >= 3 || 'Min. 3 ms',
              val => val <= 1000 || 'Maks. 1000 ms'
            ]"
          />

          <q-select
            filled
            dark
            v-model="trm"
            :options="trm_options"
            label="TRM"
            dropdown-icon="fas fa-chevron-down"
            style="width: 150px;"
            hint="FREE->streamming"
          />

          <q-select
            filled
            dark
            v-model="scm"
            :options="scm_options"
            label="SCM"
            dropdown-icon="fas fa-chevron-down"
            style="width: 100px;"
            hint="Scan Mode."
          />

          <q-select
            filled
            dark
            v-model="ocf"
            :options="ocf_options"
            label="OCF"
            dropdown-icon="fas fa-chevron-down"
            style="width: 180px;"
            hint="1=raw,2=Engineer Unit"
          />

          <q-select
            filled
            dark
            v-model="unx"
            :options="unx_options"
            label="UNIT"
            dropdown-icon="fas fa-chevron-down"
            style="width: 180px;"
            hint="Engineering Unit"
          />
        </q-card-section>

        <q-separator dark inset />

        <q-card-actions align="around">
          <q-btn flat @click="dtc_command(1)">ReZero</q-btn>
          <q-btn flat @click="dtc_command(2)">Status Scanner</q-btn>
          <q-btn flat :disable="precheck_stream">Atur Stream</q-btn>
        </q-card-actions>
      </q-card>

      <q-dialog
        v-model="dtc_dialog"
        maximized
        persistent
        transition-show="slide-up"
        transition-hide="slide-down"
      >
        <q-card dark class="bg-primary bg-grey-9 full-height">
          <q-bar>
            <div>Pesan Aeronitium</div>
          </q-bar>

          <q-card-section class="row items-center justify-center card_font_size full-height"
                          style="margin-top: -32px"
          >
            <div class="col-5 text-center">
              <q-spinner-clock color="white" size="3em" class="q-mb-lg" v-if="!show_button_dialog" />

              <p v-if="!show_button_dialog">
                Proses sedang berlangsung dan membutuhkan waktu beberapa menit. Harap bersabar...🙏
              </p>

              <p v-else>
                {{ payload.code_message }}.
                {{ payload.kind_message }}.
                <b>{{ payload.data }}</b>
              </p>

              <br />
              <q-btn
                outline
                color="white"
                label="Tutup"
                class="q-mt-lg"
                v-if="show_button_dialog"
                @click="close_dialog"
              />
            </div>
          </q-card-section>
        </q-card>
      </q-dialog>
    </div>
  </q-page>
</template>

<script src="./configs.js" />
