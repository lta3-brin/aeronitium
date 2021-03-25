import BannerComponent from "src/components/banner/banner.vue"

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
      lrn: 1,
      lrn_options: [1, 2, 3, 4, 5],
      stbl: 2,
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
      unx_options: ["psi", "Pa", "atm", "mmHg", "mmH20", "bar", "kPa", "mbar"]
    }
  },
  methods: {
    check_ocf(ocf) {
      switch (ocf) {
        case "raw":
          return 1
        case "EU":
          return 2
      }
    },
    check_unx(unx) {
      switch (unx) {
        case "psi":
          return 1
        case "Pa":
          return 3
        case "atm":
          return 6
        case "mmHg":
          return 7
        case "mmH20":
          return 8
        case "bar":
          return 9
        case "kPa":
          return 10
        case "mbar":
          return 11
      }
    }
  }
}
