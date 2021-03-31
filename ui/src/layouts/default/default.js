import MainLink from "src/components/link/MainLink"

export default {
  name: 'MainLayout',
  components: {
    MainLink
  },
  data () {
    return {
      miniState: true
    }
  },
  methods: {
    drawerClick (e) {
      e.stopPropagation()
    }
  }
}
