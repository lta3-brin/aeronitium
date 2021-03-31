import Plotly from "plotly.js-dist"
import BannerComponent from "src/components/banner/banner.vue"

export default {
  name: "StreamPage",
  components: {
    Plotly,
    BannerComponent
  },
  data() {
    return {
      interval: null,
      traces:[
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
    this.plot()
  },
  destroyed() {
    clearInterval(this.interval)
  },
  methods: {
    randomPressure() {
      return 101325 * Math.random()
    },
    plot() {
      Plotly.newPlot('plotplot', this.traces, this.layout, {
        displaylogo: false,
        responsive: true
      });

      this.interval = setInterval(() => {
        const time = new Date();

        const update = {
          x:  [[time], [time], [time], [time], [time]],
          y: [
            [this.randomPressure()],
            [this.randomPressure()],
            [this.randomPressure()],
            [this.randomPressure()],
            [this.randomPressure()],
          ]
        }

        const pastTime = time.setMinutes(time.getMinutes() - 1);
        const futureTime = time.setMinutes(time.getMinutes() + 1);

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
      }, 1000)
    }
  }
}
