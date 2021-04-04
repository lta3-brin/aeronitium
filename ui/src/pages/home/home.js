import { openURL } from 'quasar'
import BannerComponent from "src/components/banner/banner.vue"

export default {
  name: 'HomePage',
  components: {
    BannerComponent
  },
  computed: {
    server: {
      get: function() {
        const data = this.$store.getters["aeronitiummod/addressGetter"]

        return data.server
      },
      set: function (val) {
        this.$store.commit("aeronitiummod/addressServerMutation", val)
      }
    },
    websocket: {
      get: function() {
        const data = this.$store.getters["aeronitiummod/addressGetter"]

        return data.websocket
      },
      set: function (val) {
        this.$store.commit("aeronitiummod/addressWebsocketMutation", val)
      }
    }
  },
  methods: {
    onAbout(e) {
      openURL("https://github.com/bbta3-bppt/aeronitium", null, {
        noopener: true,
        noreferrer: true
      })

      e.preventDefault()
    }
  }
}
