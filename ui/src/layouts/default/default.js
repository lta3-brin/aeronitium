export default {
  name: 'MainLayout',
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
