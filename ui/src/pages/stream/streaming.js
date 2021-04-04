import Plotly from "plotly.js-dist"
import axios from "axios"
import BannerComponent from "src/components/banner/banner.vue"

export default {
  name: "StreamPage",
  components: {
    Plotly,
    BannerComponent
  },
  data() {
    return {
      connection: null,
      showplot: false,
      pesan: "Lakukan konfigurasi sebelum pantau sensor tekanan.",
      traces: [
        {
          x: [],
          y: [],
          type: 'scatter',
          mode: 'lines',
          fill: 'tozeroy',
          name: 'PORT 1',
          xaxis: 'x',
          yaxis: 'y',
        },
        {
          x: [],
          y: [],
          type: 'scatter',
          mode: 'lines',
          fill: 'tozeroy',
          name: 'PORT 2',
          xaxis: 'x2',
          yaxis: 'y2',
        },
        {
          x: [],
          y: [],
          type: 'scatter',
          mode: 'lines',
          fill: 'tozeroy',
          name: 'PORT 3',
          xaxis: 'x3',
          yaxis: 'y3',
        },
        {
          x: [],
          y: [],
          type: 'scatter',
          mode: 'lines',
          fill: 'tozeroy',
          name: 'PORT 4',
          xaxis: 'x4',
          yaxis: 'y4',
        },
        {
          x: [],
          y: [],
          type: 'scatter',
          mode: 'lines',
          fill: 'tozeroy',
          name: 'PORT 5',
          xaxis: 'x5',
          yaxis: 'y5',
        }
      ],
      layout:{
        height: 720,
        plot_bgcolor:"transparent",
        paper_bgcolor:"transparent",
        legend: {
          orientation: 'h',
          font: {
            color: 'white'
          }
        },
        margin: {
          t: 10,
          b: 0,
          l: 32,
          r: 5,
          pad: 10
        },
        grid: {
          rows: 5,
          columns: 1,
          pattern: 'independent',
          roworder: 'bottom to top'
        },
        xaxis: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        xaxis2: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        xaxis3: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        xaxis4: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        xaxis5: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        yaxis: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#2A3842",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        yaxis2: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#2A3842",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        yaxis3: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#2A3842",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        yaxis4: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#2A3842",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        yaxis5: {
          gridcolor: "rgba(147,143,141,0.1)",
          zerolinecolor: "#2A3842",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        }
      }
    }
  },
  mounted() {
    const data = this.$store.getters["aeronitiummod/addressGetter"]
    const addr = `ws://${data.websocket}`

    this.connection = new WebSocket(addr)
    this.connection.onopen = async () => {
      this.showplot = false
      this.pesan = "Mencoba terhubung dengan Aeronitium server..."

      const stream_status = this.$store.getters["aeronitiummod/streamGetter"]

      if (stream_status.ready_stream) {
        this.plot()
        if (stream_status.count === 0) { await this.req_stream() }
      }
    }

    this.connection.onmessage = (event) => {
      this.showplot = true

      const data = event.data
      const payload = data.split(",")
      const tanggal = new Date(payload[0])

      this.updatePlot(tanggal, payload)
    }

    this.connection.onclose = () => {
      this.showplot = false
      this.pesan = "Koneksi tidak terhubung dengan Aeronitium server..."
    }
  },
  destroyed() {
    this.connection.close()
  },
  methods: {
    plot() {
      Plotly.newPlot('plotplot', this.traces, this.layout, {
        displaylogo: false,
        displayModeBar: false,
        responsive: true
      });
    },
    async req_stream() {
      try {
        const dtc = this.$store.getters["aeronitiummod/dtcGetter"]

        await axios.post(`${process.env.UI_ADDRESS}/startstream`, {
          stbl: dtc.stbl
        })

        this.$store.commit("aeronitiummod/stremReadyMutation", {
          count: 1,
          ready_stream: true
        })
      } catch (err) {
        this.showplot = false
        this.pesan = `Terjadi kesalahan seputar: ${err.message}`
      }
    },
    updatePlot(tanggal, payload) {
      const update = {
        x:  [[tanggal], [tanggal], [tanggal], [tanggal], [tanggal]],
        y: [
          [payload[1]],
          [payload[2]],
          [payload[3]],
          [payload[4]],
          [payload[5]],
        ]
      }

      const pastTime = tanggal.setMinutes(tanggal.getMinutes() - 1);
      const futureTime = tanggal.setMinutes(tanggal.getMinutes() + 1);

      Plotly.relayout('plotplot', {
        xaxis: {
          type: 'date',
          range: [pastTime,futureTime],
          gridcolor: "rgba(147, 143, 141, 0.1)",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        xaxis2: {
          type: 'date',
          range: [pastTime,futureTime],
          gridcolor: "rgba(147, 143, 141, 0.1)",
        },
        xaxis3: {
          type: 'date',
          range: [pastTime,futureTime],
          gridcolor: "rgba(147, 143, 141, 0.1)",
        },
        xaxis4: {
          type: 'date',
          range: [pastTime,futureTime],
          gridcolor: "rgba(147, 143, 141, 0.1)",
        },
        xaxis5: {
          type: 'date',
          range: [pastTime,futureTime],
          gridcolor: "rgba(147, 143, 141, 0.1)",
        }
      })

      Plotly.extendTraces('plotplot', update, [0,1,2,3,4])
    }
  }
}
