export function dtcMutation (state, payload) {
  state.dtc.stbl = payload.stbl
}

export function stremReadyMutation (state, payload) {
  state.dtc.ready_stream = payload.ready_stream
}
