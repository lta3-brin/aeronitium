import BannerComponent from "src/components/banner/banner.vue"
import { check_unx } from "src/pages/configs/helpers/unx"
import { check_ocf } from "src/pages/configs/helpers/ocf"
import { precheck_stream } from "src/pages/configs/helpers/check_stream"
import { do_rezero, check_scanner, setup_stream } from "src/pages/configs/helpers/dtc"

export default {
  name: "ConfigurationsPage",
  components: {
    BannerComponent
  },
  data() {
    return {
      crs: "111",
      crs_options: ["111", "112", "113", "114", "115", "116", "117", "118"],
      num_channels: 64,
      num_channels_options: [32, 48, 64],
      scn_address: "1",
      scn_number: 1,
      scn_number_options: [1, 2, 3, 4, 5, 6, 7, 8],
      lrn: 1,
      lrn_options: [1, 2, 3, 4, 5],
      sport: "101-164",
      nfr: 1,
      frd: 0,
      nms: 0,
      msd: 500,
      trm: "FREE",
      trm_options: ["FREE", "ATRIG", "ATRIG"],
      scm: "PAM",
      scm_options: ["SEQ", "PAM"],
      ocf: "EU",
      ocf_options: ["raw", "EU"],
      unx: "Pa",
      unx_options: ["psi", "Pa", "atm", "mmHg", "mmH20", "bar", "kPa", "mbar"],
      isMounted: false,
      dtc_dialog: false,
      show_button_dialog: false,
      payload: {},
      data_coef: [],
      columns_coef: [
        { name: 'gradient', label: 'Gradient (m)', align: 'center', field: 'gradient', sortable: false },
        { name: 'bias', label: 'Bias (b)', align: 'center', field: 'bias', sortable: false },
      ],
    }
  },
  mounted() { this.isMounted = true },
  computed: {
    precheck_stream() {
      if (this.isMounted) {
        return precheck_stream(this.$refs)
      }
    },
    stbl: {
      get: function () {
        return this.$store.getters["aeronitiummod/dtcGetter"].stbl
      },
      set: function (val) {
        this.$store.commit("aeronitiummod/dtcMutation", { stbl: val })
      }
    }
  },
  methods: {
    close_dialog() {
      this.dtc_dialog = false
      this.show_button_dialog = false
      this.data_coef = []
    },
    async dtc_command(number) {
      this.dtc_dialog = true

      switch (number) {
        case 1:
          this.payload = await do_rezero(this.lrn)
          break
        case 2:
          this.payload = await check_scanner(this.crs, this.scn_number)
          break
        case 3:
          this.payload = await setup_stream({
            crs: this.crs,
            num_channels: this.num_channels,
            scn_address: this.scn_address,
            lrn: this.lrn,
            stbl: this.stbl,
            sport: this.sport,
            nfr: this.nfr,
            frd: this.frd,
            nms: this.nms,
            msd: this.msd,
            trm: this.trm,
            scm: this.scm,
            ocf: check_ocf(this.ocf),
            unx: check_unx(this.unx)
          })

          if (Array.isArray(this.payload.data)) {
            this.payload.data.forEach(val => {
              const row = val.split(",")

              this.data_coef.push({
                bias: row[0],
                gradient: row[1]
              })
            })
          }

          break
      }

      this.show_button_dialog = true
    }
  }
}
