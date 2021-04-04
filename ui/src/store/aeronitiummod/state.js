export default function () {
  return {
    address: {
      server: "127.0.0.1:8080",
      websocket: "127.0.0.1:3000",
    },
    stream: {
      ready_stream: false,
      count: 0
    },
    dtc: {
      stbl: 2,
    }
  }
}
