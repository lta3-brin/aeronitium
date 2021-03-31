import { Plotly } from "vue-plotly"
import BannerComponent from "src/components/banner/banner.vue"

export default {
  name: "StreamPage",
  components: {
    Plotly,
    BannerComponent
  },
  data() {
    return {
      data:[
        {
          x: [1, 2, 3, 4],
          y: [10, 15, 13, 17],
          type: 'scatter',
          mode: 'lines',
          name: 'Port 1'
        },
        {
          x: [1, 2, 3, 4],
          y: [16, 5, 11, 9],
          type: 'scatter',
          mode: 'lines',
          name: 'Port 2',
          xaxis: 'x2',
          yaxis: 'y2',
        },
        {
          x: [1, 2, 3, 4],
          y: [11, 9, 22, 5],
          type: 'scatter',
          mode: 'lines',
          name: 'Port 3',
          xaxis: 'x3',
          yaxis: 'y3',
        },
        {
          x: [1, 2, 3, 4],
          y: [2, 7, 3, 9],
          type: 'scatter',
          mode: 'lines',
          name: 'Port 4',
          xaxis: 'x4',
          yaxis: 'y4',
        },
        {
          x: [1, 2, 3, 4],
          y: [5, 17, 15, 10],
          type: 'scatter',
          mode: 'lines',
          name: 'Port 5',
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
          l: 15,
          r: 10
        },
        xaxis1: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#0A222A",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        xaxis2: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#0A222A",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        xaxis3: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#0A222A",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        xaxis4: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#0A222A",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        xaxis5: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#0A222A",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        yaxis1: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#0A222A",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        yaxis2: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#0A222A",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        yaxis3: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#0A222A",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        yaxis4: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#0A222A",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        yaxis5: {
          gridcolor: "rgba(147, 143, 141, 0.1)",
          zerolinecolor: "#0A222A",
          tickfont : {
            size : 10,
            color : '#F2A488'
          }
        },
        grid: {
          rows: 5,
          columns: 1,
          pattern: 'independent',
          roworder: 'bottom to top'}
      }
    }
  }
}
