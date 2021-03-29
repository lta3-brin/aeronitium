import { openURL } from 'quasar'
import BannerComponent from "src/components/banner/banner.vue"

export default {
  name: 'HomePage',
  components: {
    BannerComponent
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
