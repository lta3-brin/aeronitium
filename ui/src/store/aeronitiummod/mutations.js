export function dtcMutation (state, payload) {
  state.dtc.stbl = payload.stbl
}

export function stremReadyMutation (state, payload) {
  state.stream.ready_stream = payload.ready_stream
  state.stream.count = payload.count
}
